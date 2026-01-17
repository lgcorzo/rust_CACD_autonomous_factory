# %% IMPORTS

from autogen_team.core import schemas
from autogen_team.infrastructure import services
from autogen_team.infrastructure.utils import signers
from autogen_team.models import entities as models
from autogen_team.registry.adapters import mlflow_adapter as registries

# %% HELPERS


def test_uri_for_model_alias() -> None:
    # given
    name = "testing"
    alias = "Champion"
    # when
    uri = registries.uri_for_model_alias(name=name, alias=alias)
    # then
    assert uri == f"models:/{name}@{alias}", "The model URI should be valid!"


def test_uri_for_model_version() -> None:
    # given
    name = "testing"
    version = 1
    # when
    uri = registries.uri_for_model_version(name=name, version=str(version))
    # then
    assert uri == f"models:/{name}/{version}", "The model URI should be valid!"


def test_uri_for_model_alias_or_version() -> None:
    # given
    name = "testing"
    alias = "Champion"
    version = 1
    # when
    alias_uri = registries.uri_for_model_alias_or_version(name=name, alias_or_version=alias)
    version_uri = registries.uri_for_model_alias_or_version(name=name, alias_or_version=version)
    # then
    assert alias_uri == registries.uri_for_model_alias(
        name=name, alias=alias
    ), "The alias URI should be valid!"
    assert version_uri == registries.uri_for_model_version(
        name=name, version=str(version)
    ), "The version URI should be valid!"


# %% SAVERS/LOADERS/REGISTERS


def test_custom_pipeline(
    model: models.Model,
    inputs: schemas.Inputs,
    signature: signers.Signature,
    mlflow_service: services.MlflowService,
) -> None:
    # given
    path = "test_model_artifact"
    name = "test_model"
    tags = {"registry": "mlflow"}
    saver = registries.CustomSaver(path=path)
    loader = registries.CustomLoader()
    register = registries.MlflowRegister(tags=tags)
    run_config = mlflow_service.RunConfig(name="Custom-Run")
    # when
    with mlflow_service.run_context(run_config=run_config) as run:
        info = saver.save(model=model, signature=signature, input_example=inputs)
        version = register.register(name=name, model_uri=info.model_uri)
    model_uri = registries.uri_for_model_version(name=name, version=str(version.version))
    adapter = loader.load(uri=model_uri)
    outputs = adapter.predict(inputs=inputs)
    # then
    # - uri
    assert model_uri == f"models:/{name}/{version.version}", "The model URI should be valid!"
    # - info
    assert info.run_id == run.info.run_id, "The run id should be the same!"
    # assert info.artifact_path == path, "The artifact path should be the same!"
    assert info.signature == signature, "The model signature should be the same!"
    assert info.flavors.get("python_function"), "The model should have a pyfunc flavor!"
    # - version
    assert version.name == name, "The model version name should be the same!"
    assert version.tags == tags, "The model version tags should be the same!"
    assert version.aliases == [], "The model version aliases should be empty!"
    assert version.run_id == run.info.run_id, "The model version run id should be the same!"
    # - adapter
    assert (
        adapter.model.metadata.run_id == version.run_id
    ), "The adapter model run id should be the same!"
    assert (
        adapter.model.metadata.signature == signature
    ), "The adapter model signature should be the same!"
    assert (
        adapter.model.metadata.flavors.get("python_function") is not None
    ), "The adapter model should have a python_function flavor!"
    # - output
    assert schemas.OutputsSchema.check(outputs) is not None, "Outputs should be valid!"
