use async_openai::{
    Client,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use chrono::Utc;
use factory_infrastructure::{HttpR2rClient, R2rClient};
use hatchet_sdk::Hatchet;
use hatchet_sdk::runnables::Workflow;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use zeroize::Zeroize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeepSearchInput {
    pub query: String,
    pub job_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeepSearchOutput {
    pub job_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlanOutput {
    pub job_id: String,
    pub query: String,
    pub sub_queries: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecutionOutput {
    pub job_id: String,
    pub query: String,
    pub okf_content: String,
}

pub fn create_deep_research_workflow(
    hatchet: &Hatchet,
    r2r_url: String,
) -> Workflow<DeepSearchInput, DeepSearchOutput> {
    // R2R Client
    let r2r_user = std::env::var("R2R_SUPERUSER_EMAIL")
        .unwrap_or_else(|_| "admin@darkgravity.com".to_string());
    let r2r_pwd = std::env::var("R2R_SUPERUSER_PASSWORD").unwrap_or_else(|_| "admin".to_string());
    let _r2r_client: Arc<dyn R2rClient> = Arc::new(HttpR2rClient::new(r2r_url, r2r_user, r2r_pwd));

    // FinOps tags for LiteLLM
    let mut headers = HeaderMap::new();
    let epic = std::env::var("FINOPS_EPIC").unwrap_or_else(|_| "DeepSearch".to_string());
    let task = std::env::var("FINOPS_TASK").unwrap_or_else(|_| "ResearchDAG".to_string());
    let finops_tag = serde_json::json!({
        "epic": epic,
        "task": task
    });
    if let Ok(tag_json) = serde_json::to_string(&finops_tag)
        && let Ok(header_val) = HeaderValue::from_str(&tag_json)
    {
        headers.insert("litellm-tags", header_val);
    }

    let http_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap_or_default();

    let litellm_api_key = std::env::var("LITELLM_API_KEY").expect("LITELLM_API_KEY must be set");
    let litellm_base_url = std::env::var("LITELLM_BASE_URL").expect("LITELLM_BASE_URL must be set");
    let litellm_model =
        std::env::var("LITELLM_MODEL").unwrap_or_else(|_| "gpt-4-turbo".to_string());

    let config = async_openai::config::OpenAIConfig::new()
        .with_api_key(litellm_api_key)
        .with_api_base(litellm_base_url);

    let openai_client = Client::with_config(config).with_http_client(http_client);
    let openai_client_arc = Arc::new(openai_client);
    let openai_client_clone = openai_client_arc.clone();
    let model_clone = litellm_model.clone();

    // Deep Research Task (Planning + Execution + Ingestion)
    let research_task = hatchet
        .task("deep_research_task", move |input: DeepSearchInput, _ctx| {
            let client = openai_client_clone.clone();
            let model = model_clone.clone();
            Box::pin(async move {
                tracing::info!("DeepSearch Phase started for job {}", input.job_id);

                // 1. Planning Phase
                let request = CreateChatCompletionRequestArgs::default()
                    .model(&model)
                    .messages([
                        ChatCompletionRequestSystemMessageArgs::default()
                            .content("You are a deep search query planner. Break down the user's research topic into a JSON array of 3 to 5 highly specific sub-queries string that can be sent to a search engine.")
                            .build().map_err(|e| { tracing::error!("Builder error: {}", e); e })?.into(),
                        ChatCompletionRequestUserMessageArgs::default()
                            .content(input.query.clone())
                            .build().map_err(|e| { tracing::error!("Builder error: {}", e); e })?.into(),
                    ])
                    // REMOVE JSON OBJECT FORMAT to avoid compatibility issues with Ollama models
                    .build().map_err(|e| { tracing::error!("Builder error: {}", e); e })?;

                let response = client.chat().create(request).await.map_err(|e| {
                    tracing::error!("LiteLLM Chat Create Error (Planning): {}", e);
                    e
                })?;
                let content = response.choices
                    .first()
                    .and_then(|c| c.message.content.clone())
                    .unwrap_or_else(|| "{}".to_string());

                let parsed: serde_json::Value = serde_json::from_str(&content).unwrap_or(serde_json::json!({}));

                let mut sub_queries = vec![];
                if let Some(arr) = parsed.as_array() {
                    for v in arr {
                        if let Some(s) = v.as_str() { sub_queries.push(s.to_string()); }
                    }
                } else if let Some(arr) = parsed.get("sub_queries").and_then(|v| v.as_array()) {
                    for v in arr {
                        if let Some(s) = v.as_str() { sub_queries.push(s.to_string()); }
                    }
                }

                if sub_queries.is_empty() {
                    sub_queries.push(input.query.clone());
                }

                // 2. Execution Phase (Tavily Sequential Extraction + RAM Clamping)
                let tavily_api_key = std::env::var("TAVILY_API_KEY").map_err(|_| anyhow::anyhow!("TAVILY_API_KEY must be set"))?;
                if tavily_api_key.is_empty() {
                    return Err(anyhow::anyhow!("TAVILY_API_KEY is empty"));
                }

                let http = reqwest::Client::new();

                let mut consolidated_summaries = String::new();

                for sq in sub_queries {
                    tracing::info!("Querying Tavily for: {}", sq);
                    let tavily_req = serde_json::json!({
                        "api_key": tavily_api_key,
                        "query": sq,
                        "search_depth": "advanced",
                        "include_answer": true,
                        "include_raw_content": true,
                    });

                    let mut raw_markdown = String::new();

                    match http.post("https://api.tavily.com/search").json(&tavily_req).send().await {
                        Ok(res) => {
                            if let Ok(json_resp) = res.json::<serde_json::Value>().await
                                && let Some(results) = json_resp.get("results").and_then(|r| r.as_array()) {
                                    for r in results {
                                        if let Some(content) = r.get("raw_content").and_then(|c| c.as_str()) {
                                            raw_markdown.push_str(content);
                                            raw_markdown.push_str("\n\n");
                                        }
                                    }
                                }
                        }
                        Err(e) => tracing::warn!("Tavily search failed for query {}: {}", sq, e)
                    }

                    if raw_markdown.is_empty() {
                        continue;
                    }

                    let request = CreateChatCompletionRequestArgs::default()
                        .model(&model)
                        .messages([
                            ChatCompletionRequestSystemMessageArgs::default()
                                .content("You are a technical analyst. Extract the most important facts, code snippets, and architectural decisions from the provided raw text. Discard noise. Output in Markdown.")
                                .build().map_err(|e| { tracing::error!("Builder error: {}", e); e })?.into(),
                            ChatCompletionRequestUserMessageArgs::default()
                                .content(raw_markdown.clone())
                                .build().map_err(|e| { tracing::error!("Builder error: {}", e); e })?.into(),
                        ])
                        .build().map_err(|e| { tracing::error!("Builder error: {}", e); e })?;

                    let response = client.chat().create(request).await.map_err(|e| {
                        tracing::error!("LiteLLM Chat Create Error (Extraction): {}", e);
                        e
                    })?;
                    let summary = response.choices
                        .first()
                        .and_then(|c| c.message.content.clone())
                        .unwrap_or_default();

                    consolidated_summaries.push_str(&format!("### Sub-query: {}\n\n{}\n\n", sq, summary));

                    // RAM Clamping: Zeroize the raw markdown buffer securely
                    raw_markdown.zeroize();
                }

                // 3. Knowledge Ingestion Phase (R2R)
                let date_str = Utc::now().format("%Y-%m-%d").to_string();
                let okf_content = format!(
                    "---\ntype: deep_research\ntitle: \"{}\"\ndate: \"{}\"\njob_id: \"{}\"\n---\n\n# Deep Research Report\n\n## Synthesis\n\n{}",
                    input.query, date_str, input.job_id, consolidated_summaries
                );

                let sanitized_job_id = input.job_id.replace(|c: char| !c.is_ascii_alphanumeric() && c != '-', "");
                let tmp_path = format!("/tmp/okf_research_{}.md", sanitized_job_id);
                tokio::fs::write(&tmp_path, &okf_content).await?;

                let r2r_ingest_url = std::env::var("R2R_INGEST_URL").unwrap_or_else(|_| "http://localhost:8000/v3/ingestion/documents".to_string());
                tracing::info!("Ingesting research to R2R at {}", r2r_ingest_url);

                let res = reqwest::Client::new()
                    .post(&r2r_ingest_url)
                    .header("Content-Type", "text/markdown")
                    .body(okf_content.clone())
                    .send()
                    .await;

                match res {
                    Ok(r) if r.status().is_success() => tracing::info!("Ingestion successful"),
                    Ok(r) => tracing::warn!("Ingestion returned status {}", r.status()),
                    Err(e) => tracing::error!("Ingestion failed: {}", e),
                }

                let _ = tokio::fs::remove_file(&tmp_path).await;

                Ok(DeepSearchOutput {
                    job_id: input.job_id,
                    status: "completed".to_string(),
                })
            })
        })
        .with_timeout(Some(std::time::Duration::from_secs(600)))
        .build()
        .unwrap();

    hatchet
        .workflow("deep-search-workflow")
        .version("1.0.0".to_string())
        .build()
        .unwrap()
        .add_task(&research_task)
}
