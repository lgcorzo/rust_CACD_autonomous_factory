"""Configuration for the tests."""

# %% IMPORTS

import os
import typing as T
from typing import Any, cast

import omegaconf
import pytest
from _pytest import logging as pl
from agent_framework import ChatMessage
from agent_framework.openai import OpenAIChatClient
from autogen_team.core import metrics, models, schemas
from autogen_team.io import datasets, registries, services
from autogen_team.utils import searchers, signers, splitters
from mocogpt import GptServer, gpt_server
from openai import OpenAI

# %% COMPATIBILITY PATCH
# agent-framework ALWAYS sends 'content' as a list of dicts: [{'type': 'text', 'text': '...'}]
# mocogpt's token counter (tiktoken) expects a string for 'content' and crashes with 500.
_orig_prepare = OpenAIChatClient._prepare_message_for_openai


def _patched_prepare(self: OpenAIChatClient, message: ChatMessage) -> T.List[T.Dict[str, T.Any]]:
    res_list = _orig_prepare(self, message)
    for res in res_list:
        if (
            isinstance(res.get("content"), list)
            and len(res["content"]) == 1
            and res["content"][0].get("type") == "text"
        ):
            res["content"] = res["content"][0]["text"]
    return res_list


OpenAIChatClient._prepare_message_for_openai = _patched_prepare  # type: ignore[assignment]

# %% CONFIGS

LIMIT = 4
N_SPLITS = 2
TEST_SIZE = 1  # 1 week

# %% FIXTURES

# %% - Paths


@pytest.fixture(scope="session")
def tests_path() -> str:
    """Return the path of the tests folder."""
    file = os.path.abspath(__file__)
    parent = os.path.dirname(file)
    return parent


@pytest.fixture(scope="session")
def data_path(tests_path: str) -> str:
    """Return the path of the data folder."""
    return os.path.join(tests_path, "data")


@pytest.fixture(scope="session")
def confs_path(tests_path: str) -> str:
    """Return the path of the confs folder."""
    return os.path.join(tests_path, "confs")


@pytest.fixture(scope="session")
def inputs_path(data_path: str) -> str:
    """Return the path of the inputs dataset."""
    return os.path.join(data_path, "inputs_sample.parquet")


@pytest.fixture(scope="session")
def targets_path(data_path: str) -> str:
    """Return the path of the targets dataset."""
    return os.path.join(data_path, "targets_sample.parquet")


@pytest.fixture(scope="session")
def outputs_path(data_path: str) -> str:
    """Return the path of the outputs dataset."""
    return os.path.join(data_path, "outputs_sample.parquet")


@pytest.fixture(scope="function")
def tmp_outputs_path(tmp_path: str) -> str:
    """Return a tmp path for the outputs dataset."""
    return os.path.join(tmp_path, "outputs.parquet")


@pytest.fixture(scope="function")
def tmp_models_explanations_path(tmp_path: str) -> str:
    """Return a tmp path for the model explanations dataset."""
    return os.path.join(tmp_path, "models_explanations.parquet")


@pytest.fixture(scope="function")
def tmp_samples_explanations_path(tmp_path: str) -> str:
    """Return a tmp path for the samples explanations dataset."""
    return os.path.join(tmp_path, "samples_explanations.parquet")


# %% - Configs


@pytest.fixture(scope="session")
def extra_config() -> str:
    """Extra config for scripts."""
    # use OmegaConf resolver: ${tmp_path:}
    config = """
    {
        "job": {
            "alerts_service": {
                "enable": false,
            },
            "mlflow_service": {
                "tracking_uri": "${tmp_path:}/tracking/",
                "registry_uri": "${tmp_path:}/registry/",
            }
        }
    }
    """
    return config


# %% - Datasets


@pytest.fixture(scope="session")
def inputs_reader(inputs_path: str) -> datasets.ParquetReader:
    """Return a reader for the inputs dataset."""
    return datasets.ParquetReader(path=inputs_path, limit=LIMIT)


@pytest.fixture(scope="session")
def inputs_samples_reader(inputs_path: str) -> datasets.ParquetReader:
    """Return a reader for the inputs samples dataset."""
    return datasets.ParquetReader(path=inputs_path, limit=LIMIT)


@pytest.fixture(scope="session")
def targets_reader(targets_path: str) -> datasets.ParquetReader:
    """Return a reader for the targets dataset."""
    return datasets.ParquetReader(path=targets_path, limit=LIMIT)


@pytest.fixture(scope="session")
def outputs_reader(
    outputs_path: str, inputs_reader: datasets.ParquetReader, targets_reader: datasets.ParquetReader
) -> datasets.ParquetReader:
    """Return a reader for the outputs dataset."""
    # generate outputs if it is missing
    if not os.path.exists(outputs_path):
        model_config = {
            "provider": "openai_chat_completion_client",  # Use LiteLLM-compatible client
            "config": {
                "model": "gpt-4",  # LiteLLM model
                "api_base": "http://localhost:12306/v1",  # LiteLLM Gateway URL
                "api_key": "sk-123456789",
                "temperature": 0.7,  # Optional
                "max_tokens": 512,  # Optional
            },
        }
        inputs = schemas.InputsSchema.check(inputs_reader.read())
        targets = schemas.TargetsSchema.check(targets_reader.read())
        model = models.BaselineAutogenModel()
        model.load_context(model_config=model_config)
        model.fit(inputs=inputs, targets=targets)
        outputs = schemas.OutputsSchema.check(model.predict(inputs=inputs))
        outputs_writer = datasets.ParquetWriter(path=outputs_path)
        outputs_writer.write(data=outputs)
    return datasets.ParquetReader(path=outputs_path, limit=LIMIT)


@pytest.fixture(scope="function")
def tmp_outputs_writer(tmp_outputs_path: str) -> datasets.ParquetWriter:
    """Return a writer for the tmp outputs dataset."""
    return datasets.ParquetWriter(path=tmp_outputs_path)


@pytest.fixture(scope="function")
def tmp_models_explanations_writer(tmp_models_explanations_path: str) -> datasets.ParquetWriter:
    """Return a writer for the tmp model explanations dataset."""
    return datasets.ParquetWriter(path=tmp_models_explanations_path)


@pytest.fixture(scope="function")
def tmp_samples_explanations_writer(tmp_samples_explanations_path: str) -> datasets.ParquetWriter:
    """Return a writer for the tmp samples explanations dataset."""
    return datasets.ParquetWriter(path=tmp_samples_explanations_path)


# %% - Dataframes


@pytest.fixture(scope="session")
def inputs(inputs_reader: datasets.ParquetReader) -> schemas.Inputs:
    """Return the inputs data."""
    data = inputs_reader.read()
    return schemas.InputsSchema.check(data)


@pytest.fixture(scope="session")
def inputs_samples(inputs_samples_reader: datasets.ParquetReader) -> schemas.Inputs:
    """Return the inputs samples data."""
    data = inputs_samples_reader.read()
    return schemas.InputsSchema.check(data)


@pytest.fixture(scope="session")
def targets(targets_reader: datasets.ParquetReader) -> schemas.Targets:
    """Return the targets data."""
    data = targets_reader.read()
    return schemas.TargetsSchema.check(data)


@pytest.fixture(scope="session")
def outputs(outputs_reader: datasets.ParquetReader) -> schemas.Outputs:
    """Return the outputs data."""
    data = outputs_reader.read()
    return schemas.OutputsSchema.check(data)


# %% - Splitters


@pytest.fixture(scope="session")
def train_test_splitter() -> splitters.TrainTestSplitter:
    """Return the default train test splitter."""
    return splitters.TrainTestSplitter(test_size=TEST_SIZE)


@pytest.fixture(scope="session")
def time_series_splitter() -> splitters.TimeSeriesSplitter:
    """Return the default time series splitter."""
    return splitters.TimeSeriesSplitter(n_splits=N_SPLITS, test_size=TEST_SIZE)


# %% - Searchers


@pytest.fixture(scope="session")
def searcher() -> searchers.Searcher:
    """Return the default searcher object."""
    param_grid = {"max_tokens": [10000, 20000], "temperature": [3.0]}
    return searchers.GridCVSearcher(param_grid=cast(dict[str, list[Any]], param_grid))


# %% - Subsets


@pytest.fixture(scope="session")
def train_test_sets(
    train_test_splitter: splitters.Splitter, inputs: schemas.Inputs, targets: schemas.Targets
) -> tuple[schemas.Inputs, schemas.Targets, schemas.Inputs, schemas.Targets]:
    """Return the inputs and targets train and test sets from the splitter."""
    train_index, test_index = next(train_test_splitter.split(inputs=inputs, targets=targets))
    inputs_train, inputs_test = inputs.iloc[train_index], inputs.iloc[test_index]
    targets_train, targets_test = targets.iloc[train_index], targets.iloc[test_index]
    return (
        T.cast(schemas.Inputs, inputs_train),
        T.cast(schemas.Targets, targets_train),
        T.cast(schemas.Inputs, inputs_test),
        T.cast(schemas.Targets, targets_test),
    )


# %% - Models


@pytest.fixture(scope="session")
def model(
    train_test_sets: tuple[schemas.Inputs, schemas.Targets, schemas.Inputs, schemas.Targets],
) -> models.BaselineAutogenModel:
    """Return a train model for testing."""
    model_config = {
        "provider": "openai_chat_completion_client",  # Use LiteLLM-compatible client
        "config": {
            "model": "gpt-4",  # LiteLLM model
            "api_base": "http://localhost:12306/v1",  # LiteLLM Gateway URL
            "api_key": "sk-123456789",
            "temperature": 0.7,  # Optional
            "max_tokens": 512,  # Optional
        },
    }
    model = models.BaselineAutogenModel()
    model.load_context(model_config=model_config)
    inputs_train, targets_train, _, _ = train_test_sets
    model.fit(inputs=inputs_train, targets=targets_train)
    return model


# %% - Metrics


@pytest.fixture(scope="session")
def metric() -> metrics.AutogenMetric:
    """Return the default metric."""
    return metrics.AutogenMetric(
        name="AutogenMetricTest", metric_type="exact_match", greater_is_better=True
    )


# %% - Signers


@pytest.fixture(scope="session")
def signer() -> signers.Signer:
    """Return a model signer."""
    return signers.InferSigner()


# %% - Services


@pytest.fixture(scope="session", autouse=True)
def logger_service() -> T.Generator[services.LoggerService, None, None]:
    """Return and start the logger service."""
    service = services.LoggerService(colorize=False, diagnose=True)
    service.start()
    yield service
    service.stop()


@pytest.fixture
def logger_caplog(
    caplog: pl.LogCaptureFixture, logger_service: services.LoggerService
) -> T.Generator[pl.LogCaptureFixture, None, None]:
    """Extend pytest caplog fixture with the logger service (loguru)."""
    # https://loguru.readthedocs.io/en/stable/resources/migration.html#replacing-caplog-fixture-from-pytest-library
    logger = logger_service.logger()
    handler_id = logger.add(
        caplog.handler,
        level=0,
        format="{message}",
        filter=lambda record: record["level"].no >= caplog.handler.level,
        enqueue=False,  # Set to 'True' if your test is spawning child processes.
    )
    yield caplog
    logger.remove(handler_id)


@pytest.fixture(scope="session", autouse=True)
def alerts_service() -> T.Generator[services.AlertsService, None, None]:
    """Return and start the alerter service."""
    service = services.AlertsService(enable=False)
    service.start()
    yield service
    service.stop()


@pytest.fixture(scope="function", autouse=True)
def mlflow_service(tmp_path: str) -> T.Generator[services.MlflowService, None, None]:
    """Return and start the mlflow service."""
    service = services.MlflowService(
        tracking_uri=f"{tmp_path}/tracking/",
        registry_uri=f"{tmp_path}/registry/",
        experiment_name="Experiment-Testing",
        registry_name="Registry-Testing",
    )
    service.start()
    yield service
    service.stop()


@pytest.fixture(scope="session", autouse=True)
def chtgpt_service(targets: schemas.Targets, inputs_samples: schemas.Inputs) -> GptServer:
    """Return and start the logger service."""
    server = gpt_server(12306)
    # Read the targets and configure the chat responses
    for input_text, response_text in zip(inputs_samples.input, targets.response):
        server.chat.completions.request(prompt=input_text).response(content=response_text)

    server.chat.completions.request(prompt="Hola").response(content="Cómo puedo ayudarte?")
    with server:
        client = OpenAI(base_url="http://localhost:12306/v1", api_key="sk-123456789")
        response = client.chat.completions.create(
            model="gpt-4", messages=[{"role": "user", "content": "Hola"}]
        )

        assert response.choices[0].message.content == "Cómo puedo ayudarte?"
        yield server
    del server


# %% - Resolvers


@pytest.fixture(scope="session", autouse=True)
def tests_path_resolver(tests_path: str) -> str:
    """Register the tests path resolver with OmegaConf."""

    def resolver() -> str:
        """Get tests path."""
        return tests_path

    omegaconf.OmegaConf.register_new_resolver("tests_path", resolver, use_cache=True, replace=False)
    return tests_path


@pytest.fixture(scope="function", autouse=True)
def tmp_path_resolver(tmp_path: str) -> str:
    """Register the tmp path resolver with OmegaConf."""

    def resolver() -> str:
        """Get tmp data path."""
        return tmp_path

    omegaconf.OmegaConf.register_new_resolver("tmp_path", resolver, use_cache=False, replace=True)
    return tmp_path


# %% - Signatures


@pytest.fixture(scope="session")
def signature(
    signer: signers.Signer, inputs: schemas.Inputs, outputs: schemas.Outputs
) -> signers.Signature:
    """Return the signature for the testing model."""
    return signer.sign(inputs=inputs, outputs=outputs)


# %% - Registries


@pytest.fixture(scope="session")
def saver() -> registries.CustomSaver:
    """Return the default model saver."""
    return registries.CustomSaver(path="test_model_artifact")


@pytest.fixture(scope="session")
def loader() -> registries.CustomLoader:
    """Return the default model loader."""
    return registries.CustomLoader()


@pytest.fixture(scope="session")
def register() -> registries.MlflowRegister:
    """Return the default model register."""
    tags = {"context": "test", "role": "fixture"}
    return registries.MlflowRegister(tags=tags)


@pytest.fixture(scope="function")
def model_version(
    model: models.Model,
    inputs: schemas.Inputs,
    signature: signers.Signature,
    saver: registries.Saver,
    register: registries.Register,
    mlflow_service: services.MlflowService,
) -> registries.Version:
    """Save and register the default model version."""
    run_config = mlflow_service.RunConfig(name="Custom-Run")
    with mlflow_service.run_context(run_config=run_config):
        info = saver.save(model=model, signature=signature, input_example=inputs)
        version = register.register(name=mlflow_service.registry_name, model_uri=info.model_uri)
    return version


@pytest.fixture(scope="function")
def model_alias(
    model_version: registries.Version,
    mlflow_service: services.MlflowService,
) -> registries.Alias:
    """Promote the default model version with an alias."""
    alias = "Promotion"
    client = mlflow_service.client()
    client.set_registered_model_alias(
        name=mlflow_service.registry_name, alias=alias, version=model_version.version
    )
    model_alias = client.get_model_version_by_alias(name=mlflow_service.registry_name, alias=alias)
    return model_alias
