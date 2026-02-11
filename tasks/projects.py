"""Project tasks for pyinvoke."""

# mypy: disable-error-code="arg-type"

# %% IMPORTS

import json

from invoke.context import Context
from invoke.tasks import call, task

# %% CONFIGS

PYTHON_VERSION = ".python-version"
REQUIREMENTS = "requirements.txt"
ENVIRONMENT = "python_env.yaml"

# %% TASKS


@task
def requirements(ctx: Context) -> None:
    """Export the project requirements file."""
    ctx.run(f"poetry export --without-urls --without-hashes --output={REQUIREMENTS}")


@task(pre=[requirements])
def environment(ctx: Context) -> None:
    """Export the project environment file."""
    with open(PYTHON_VERSION, "r") as reader:
        python = reader.read().strip()  # version
    configuration: dict[str, object] = {"python": python}
    with open(REQUIREMENTS, "r") as reader:
        dependencies: list[str] = []
        for line in reader:
            dependency = line.split(" ")[0]
            if "pywin32" not in dependency:
                dependencies.append(dependency)
    configuration["dependencies"] = dependencies
    with open(ENVIRONMENT, "w") as writer:
        # Safe as YAML is a superset of JSON
        json.dump(configuration, writer, indent=4)
        writer.write("\n")  # add new line at the end


@task
def run(ctx: Context, job: str) -> None:
    """Run an mlflow project from the MLproject file."""
    ctx.run(
        f"poetry run mlflow run --experiment-name={ctx.project.repository}"
        f" --run-name={job.capitalize()} -P conf_file=confs/{job}.yaml ."
    )


@task
def mcp(ctx: Context, prompts: str | None = None) -> None:
    """Run the MCP server.

    Args:
        prompts (str, optional): Path to the prompts YAML config.
    """
    env = {}
    if prompts:
        env["MCP_PROMPTS_PATH"] = prompts

    ctx.run(
        "poetry run python -m autogen_team.application.mcp.mcp_server",
        env=env,
    )


@task
def kafka(ctx: Context) -> None:
    """Run the Kafka inference service."""
    ctx.run("poetry run python -m autogen_team.infrastructure.messaging.kafka_app")


@task(
    pre=[
        environment,
        call(run, job="tuning"),
        call(run, job="training"),
        call(run, job="promotion"),
        call(run, job="inference"),
        call(run, job="hatchet_inference"),
        call(run, job="evaluations"),
        call(run, job="explanations"),
        kafka,
        call(mcp, prompts="confs/mcp_prompts.yaml"),
    ],
    default=True,
)
def all(_: Context) -> None:
    """Run all project tasks."""
