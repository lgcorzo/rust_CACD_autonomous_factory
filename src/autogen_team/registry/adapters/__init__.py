"""Registry Adapters."""

from .mlflow_adapter import (
    Saver,
    CustomSaver,
    SaverKind,
    Register,
    MlflowRegister,
    RegisterKind,
    Loader,
    CustomLoader,
    LoaderKind,
    Info,
    Alias,
    Version,
    uri_for_model_alias,
    uri_for_model_version,
    uri_for_model_alias_or_version,
)

__all__ = [
    "Saver",
    "CustomSaver",
    "SaverKind",
    "Register",
    "MlflowRegister",
    "RegisterKind",
    "Loader",
    "CustomLoader",
    "LoaderKind",
    "Info",
    "Alias",
    "Version",
    "uri_for_model_alias",
    "uri_for_model_version",
    "uri_for_model_alias_or_version",
]
