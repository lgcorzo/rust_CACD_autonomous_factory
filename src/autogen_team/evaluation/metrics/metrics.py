"""Evaluate model performances with metrics."""
# mypy: disable-error-code=name-defined
# mypy: disable-error-code=operator

# %% IMPORTS

from __future__ import annotations

import abc
import typing as T
from difflib import SequenceMatcher
from typing import Optional, cast

import mlflow
import pandas as pd
import pydantic as pdt
from mlflow.metrics import MetricValue

from autogen_team.core import schemas
from autogen_team.models import entities as models

# %% TYPINGS
MlflowMetric: T.TypeAlias = MetricValue
MlflowThreshold: T.TypeAlias = mlflow.models.MetricThreshold
MlflowModelValidationFailedException: T.TypeAlias = (
    mlflow.models.evaluation.validation.ModelValidationFailedException
)


# %% METRICS


class Metric(abc.ABC, pdt.BaseModel, strict=True, frozen=True, extra="forbid"):
    """Base class for a project metric.

    Use metrics to evaluate model performance.
    e.g., accuracy, precision, recall, MAE, F1, ...

    Parameters:
        name (str): name of the metric for the reporting.
        greater_is_better (bool): maximize or minimize result.
    """

    KIND: str

    name: str
    greater_is_better: bool

    @abc.abstractmethod
    def score(self, targets: pd.DataFrame, outputs: pd.DataFrame) -> float:
        """Score the outputs against the targets.

        Args:
            targets (pd.DataFrame): expected values.
            outputs (pd.DataFrame): predicted values.

        Returns:
            float: single result from the metric computation.
        """

    def scorer(self, model: models.Model, inputs: schemas.Inputs, targets: pd.DataFrame) -> float:
        """Score model outputs against targets.

        Args:
            model (models.Model): model to evaluate.
            inputs (schemas.Inputs): model inputs values.
            targets (schemas.Targets): model expected values.

        Returns:
            float: single result from the metric computation.
        """
        outputs = model.predict(inputs=inputs)
        score = self.score(targets=targets, outputs=outputs)
        return score

    def to_mlflow(self) -> MlflowMetric:
        """Convert the metric to an Mlflow metric.

        Returns:
            MlflowMetric: the Mlflow metric.
        """

        def eval_fn(predictions: pd.Series[str], targets: pd.Series[str]) -> MlflowMetric:
            """Evaluation function associated with the mlflow metric.
            https://mlflow.org/docs/latest/llms/llm-evaluate/index.html
            Args:
                predictions (pd.Series): model predictions.
                targets (pd.Series | None): model targets.

            Returns:
                MlflowMetric: the mlflow metric.
            """
            score_outputs = pd.DataFrame(
                {schemas.OutputsSchema.response: predictions}, index=predictions.index
            )
            score_targets = pd.DataFrame(
                {schemas.TargetsSchema.response: targets}, index=targets.index
            )

            sign = 1 if self.greater_is_better else -1  # reverse the effect
            score = self.score(targets=score_targets, outputs=score_outputs)
            return MlflowMetric(aggregate_results={self.name: score * sign})

        return cast(
            MlflowMetric,
            mlflow.metrics.make_metric(
                eval_fn=eval_fn, name=self.name, greater_is_better=self.greater_is_better
            ),
        )


class AutogenMetric(Metric):
    """Evaluate text-based Autogen responses using conversation metrics.

    Parameters:
        metric_type (str): Type of text metric (exact_match, similarity, length_ratio)
        similarity_threshold (float): Minimum similarity score for partial matches
    """

    KIND: T.Literal["AutogenMetric"] = "AutogenMetric"
    metric_type: T.Literal["exact_match", "similarity", "length_ratio"] = "similarity"
    similarity_threshold: Optional[float] = 0.7

    def score(self, targets: pd.DataFrame, outputs: pd.DataFrame) -> float:
        # Extract text responses from targets and outputs
        y_true: pd.Series[str] = targets.response
        y_pred: pd.Series[str] = outputs.response

        if self.metric_type == "exact_match":
            return self._exact_match_score(y_true, y_pred)
        elif self.metric_type == "similarity":
            return self._similarity_score(y_true, y_pred)
        elif self.metric_type == "length_ratio":
            return self._length_ratio(y_true, y_pred)
        else:
            raise ValueError(f"Unknown metric type: {self.metric_type}")

    def _exact_match_score(self, y_true: pd.Series[str], y_pred: pd.Series[str]) -> float:
        # Reset index to align the series
        y_true = y_true.reset_index(drop=True)
        y_pred = y_pred.reset_index(drop=True)
        return (y_true == y_pred).mean()

    def _similarity_score(self, y_true: pd.Series[str], y_pred: pd.Series[str]) -> float:
        def calculate_similarity(true_text: str, pred_text: str) -> float:
            return SequenceMatcher(None, true_text, pred_text).ratio()

        ##TBD
        similarities = y_true.combine(y_pred, calculate_similarity)
        return (similarities >= self.similarity_threshold).mean()

    def _length_ratio(self, y_true: pd.Series[str], y_pred: pd.Series[str]) -> float:
        length_ratios = y_pred.str.len() / y_true.str.len().replace(0, 1)
        return length_ratios.mean()


class AutogenConversationMetric(Metric):
    """Evaluate conversation quality metrics for Autogen interactions.

    Parameters:
        check_termination (bool): Verify if conversation reached termination
        check_error_messages (bool): Check for error messages in output
    """

    KIND: T.Literal["AutogenConversationMetric"] = "AutogenConversationMetric"

    check_termination: bool = True
    check_error_messages: bool = True

    def score(self, targets: pd.DataFrame, outputs: pd.DataFrame) -> float:
        metadata = outputs[schemas.OutputsSchema.metadata]

        score = 1.0

        if self.check_termination:
            terminated = metadata.apply(lambda x: x.get("terminated", False))
            score *= terminated.mean()

        if self.check_error_messages:
            has_errors = metadata.apply(lambda x: "error" in x.get("messages", []))
            score *= 1 - has_errors.mean()

        return float(score)


MetricKind = AutogenMetric | AutogenConversationMetric
MetricsKind: T.TypeAlias = list[T.Annotated[MetricKind, pdt.Field(discriminator="KIND")]]


# %% THRESHOLDS


class Threshold(abc.ABC, pdt.BaseModel, strict=True, frozen=True, extra="forbid"):
    """A project threshold for a metric.

    Use thresholds to monitor model performances.
    e.g., to trigger an alert when a threshold is met.

    Parameters:
        threshold (int | float): absolute threshold value.
        greater_is_better (bool): maximize or minimize result.
    """

    threshold: int | float
    greater_is_better: bool

    def to_mlflow(self) -> MlflowThreshold:
        """Convert the threshold to an mlflow threshold.

        Returns:
            MlflowThreshold: the mlflow threshold.
        """
        return mlflow.models.MetricThreshold(
            threshold=self.threshold, greater_is_better=self.greater_is_better
        )
