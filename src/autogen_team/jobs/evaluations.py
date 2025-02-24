"""Define a job for evaluating registered models with data."""

# %% IMPORTS

import typing as T
from typing import Dict, List

import mlflow
import pandas as pd
import pydantic as pdt

from autogen_team.core import metrics as metrics_
from autogen_team.core import schemas
from autogen_team.io import datasets, registries, services
from autogen_team.jobs import base


# %% JOBS


class EvaluationsJob(base.Job):
    """Generate evaluations from a registered model and a dataset.

    Parameters:
        run_config (services.MlflowService.RunConfig): mlflow run config.
        inputs (datasets.ReaderKind): reader for the inputs data.
        targets (datasets.ReaderKind): reader for the targets data.
        model_type (str): model type (e.g. "regressor", "classifier").
        alias_or_version (str | int): alias or version for the  model.
        metrics (metrics_.MetricKind): metrics for the reporting.
        evaluators (list[str]): list of evaluators to use.
        thresholds (dict[str, metrics_.Threshold] | None): metric thresholds.
    """

    KIND: T.Literal["EvaluationsJob"] = "EvaluationsJob"

    # Run
    run_config: services.MlflowService.RunConfig = services.MlflowService.RunConfig(
        name="Evaluations"
    )
    # Data
    inputs: datasets.ReaderKind = pdt.Field(..., discriminator="KIND")
    targets: datasets.ReaderKind = pdt.Field(..., discriminator="KIND")
    # Model
    model_type: str = "question-answering"
    alias_or_version: T.Union[str, int] = "Champion"
    # Metrics
    metrics: List[metrics_.AutogenMetric] = [  # Assuming MetricsKind is List[AutogenMetric]
        metrics_.AutogenMetric(
            name="AutogenMetric", metric_type="exact_match", greater_is_better=True
        )
    ]
    # Evaluators
    evaluators: List[str] = ["default"]
    # Thresholds
    thresholds: Dict[str, metrics_.Threshold] = {  # Assuming metrics_.Threshold is the correct type
        "exact_match": metrics_.Threshold(threshold=0.5, greater_is_better=True)
    }

    def run(self) -> base.Locals:  # Assuming base.Locals is dict[str, Any]
        # services
        # - logger
        logger = self.logger_service.logger()
        logger.info("With logger: {}", logger)
        # - mlflow
        client = self.mlflow_service.client()
        logger.info("With client: {}", client.tracking_uri)
        with self.mlflow_service.run_context(run_config=self.run_config) as run:
            logger.info("With run context: {}", run.info)
            # data
            # - inputs
            logger.info("Read inputs: {}", self.inputs)
            inputs_ = self.inputs.read()  # unchecked!
            inputs = schemas.InputsSchema.check(inputs_)  # Assuming check returns DataFrame
            logger.debug("- Inputs shape: {}", inputs.shape)
            # - targets
            logger.info("Read targets: {}", self.targets)
            targets_ = self.targets.read()  # unchecked!
            targets = schemas.TargetsSchema.check(targets_)  # Assuming check returns DataFrame
            logger.debug("- Targets shape: {}", targets.shape)
            # lineage
            # - inputs
            logger.info("Log lineage: inputs")
            inputs_lineage = self.inputs.lineage(
                data=inputs, name="inputs"
            )  # Assuming lineage returns mlflow.data.Dataset
            mlflow.log_input(dataset=inputs_lineage, context=self.run_config.name)
            logger.debug("- Inputs lineage: {}", inputs_lineage.to_dict())
            # - targets
            logger.info("Log lineage: targets")
            targets_lineage = self.targets.lineage(
                data=targets, name="targets", targets=schemas.TargetsSchema.response
            )  # Assuming lineage returns mlflow.data.Dataset
            mlflow.log_input(dataset=targets_lineage, context=self.run_config.name)
            logger.debug("- Targets lineage: {}", targets_lineage.to_dict())
            # dataset
            logger.info("Create dataset: inputs & targets")
            dataset = mlflow.data.from_pandas(
                df=pd.concat([inputs, targets], axis="columns"),
                name="evaluation",
                source=f"{inputs_lineage.source.uri} & {targets_lineage.source.uri}",
                targets=schemas.TargetsSchema.response,
            )  # Assuming from_pandas returns mlflow.data.Dataset
            logger.debug("- Dataset: {}", dataset.to_dict())
            # model
            logger.info("With model: {}", self.mlflow_service.registry_name)
            model_uri = registries.uri_for_model_alias_or_version(
                name=self.mlflow_service.registry_name, alias_or_version=self.alias_or_version
            )  # Assuming this returns str model_uri
            logger.debug("- Model URI: {}", model_uri)
            # metrics
            logger.debug("Convert metrics: {}", self.metrics)
            extra_metrics = [
                metric.to_mlflow() for metric in self.metrics
            ]  # Assuming to_mlflow returns mlflow metric object
            logger.debug("- Extra metrics: {}", extra_metrics)
            # thresholds
            logger.info("Convert thresholds: {}", self.thresholds)
            validation_thresholds = {
                name: threshold.to_mlflow()
                for name, threshold in self.thresholds.items()  # Assuming to_mlflow returns mlflow threshold object
            }
            logger.debug("- Validation thresholds: {}", validation_thresholds)
            # evaluations
            logger.info("Compute evaluations: {}", self.model_type)
            evaluations = mlflow.evaluate(
                data=dataset,
                model=model_uri,
                model_type=self.model_type,
                evaluators=self.evaluators,
                extra_metrics=extra_metrics,
                validation_thresholds=validation_thresholds,
                predictions=schemas.OutputsSchema.response,
            )  # Assuming evaluate returns mlflow.EvaluationResult
            logger.debug("- Evaluations metrics: {}", evaluations.metrics)
            # notify
            self.alerts_service.notify(
                title="Evaluations Job Finished",
                message=f"Evaluation metrics: {evaluations.metrics}",
            )
        return locals()
