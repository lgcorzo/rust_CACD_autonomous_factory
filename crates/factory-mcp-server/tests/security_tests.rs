use factory_mcp_server::tools::security_review::SecurityReviewTool;
use factory_mcp_server::tools::Tool;
use factory_mcp_server::protocol::McpContent;
use serde_json::{json, Value};

#[tokio::test]
async fn test_security_review_sql_injection() {
    let tool = SecurityReviewTool;
    let malicious_diff = "+ cursor.execute(\"SELECT * FROM users WHERE id = \" + id)";
    let params = json!({ "diff": malicious_diff });
    
    let result = tool.call(params).await.unwrap();
    let McpContent::Text { text } = &result.content[0] else { panic!("Expected Text content") };
    let content: Value = serde_json::from_str(text).unwrap();
    
    assert_eq!(content["status"], "rejected");
    assert!(content["findings"].as_array().unwrap().iter().any(|f| f["rule"] == "A03:Injection - SQL"));
}

#[tokio::test]
async fn test_security_review_command_injection() {
    let tool = SecurityReviewTool;
    let malicious_diff = "+ os.system(f\"rm -rf {path}\")";
    let params = json!({ "diff": malicious_diff });
    
    let result = tool.call(params).await.unwrap();
    let McpContent::Text { text } = &result.content[0] else { panic!("Expected Text content") };
    let content: Value = serde_json::from_str(text).unwrap();
    
    assert_eq!(content["status"], "rejected");
    assert!(content["findings"].as_array().unwrap().iter().any(|f| f["rule"] == "A03:Injection - Command"));
}

#[tokio::test]
async fn test_security_review_hardcoded_secret() {
    let tool = SecurityReviewTool;
    let malicious_diff = "+ api_key = \"sk-1234567890abcdef1234567890abcdef\"";
    let params = json!({ "diff": malicious_diff });
    
    let result = tool.call(params).await.unwrap();
    let McpContent::Text { text } = &result.content[0] else { panic!("Expected Text content") };
    let content: Value = serde_json::from_str(text).unwrap();
    
    assert!(content["findings"].as_array().unwrap().iter().any(|f| f["rule"] == "A02:Crypto - Hardcoded Secret"));
}

#[tokio::test]
async fn test_security_review_safe_code() {
    let tool = SecurityReviewTool;
    let safe_diff = "+ def hello():\n+     print(\"Hello world\")";
    let params = json!({ "diff": safe_diff });
    
    let result = tool.call(params).await.unwrap();
    let McpContent::Text { text } = &result.content[0] else { panic!("Expected Text content") };
    let content: Value = serde_json::from_str(text).unwrap();
    
    assert_eq!(content["status"], "approved");
    assert_eq!(content["findings"].as_array().unwrap().len(), 0);
}
