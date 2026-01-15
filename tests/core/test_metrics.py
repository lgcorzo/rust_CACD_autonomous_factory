# test_metrics.py

from typing import Any, Dict, Iterator, List, Literal, Optional
from unittest.mock import MagicMock, patch

import pandas as pd
import pytest

# Assuming the metrics are in a module named 'metrics.py'
from autogen_team.evaluation.metrics import (
    AutogenConversationMetric,
    AutogenMetric,
    Threshold,
)


# Mock schemas to avoid dependencies
@pytest.fixture(autouse=True)
def mock_schemas() -> Iterator[MagicMock]:
    with patch("autogen_team.evaluation.metrics.metrics.schemas") as mock_schemas:
        mock_schemas.TargetsSchema.response = "response"
        mock_schemas.OutputsSchema.response = "response"
        mock_schemas.OutputsSchema.metadata = "metadata"
        yield mock_schemas


# ... (omitted similar lines, focus on changes)


# Test Metric Integration
class TestMetricIntegration:
    def test_scorer_flow(self) -> None:
        # Mock dependencies
        mock_model = MagicMock()
        mock_inputs = MagicMock()
        mock_targets = MagicMock()
        mock_outputs = MagicMock()

        # Configure model to return outputs
        mock_model.predict.return_value = mock_outputs

        # Create metric with mocked score method
        metric = AutogenMetric(
            name="AutogenMetricTest", metric_type="exact_match", greater_is_better=True
        )

        # Execute scorer
        with patch("autogen_team.evaluation.metrics.AutogenMetric.score") as mock_score:
            mock_score.return_value = 0.5
            result = metric.scorer(mock_model, mock_inputs, mock_targets)

        # Verify calls
        mock_score.assert_called_once_with(targets=mock_targets, outputs=mock_outputs)
        assert result == 0.5


# Test AutogenMetric
class TestAutogenTextMetric:
    @pytest.mark.parametrize(
        "metric_type, y_true, y_pred, expected, threshold",
        [
            # Exact match cases
            (
                "exact_match",
                ["cat", "dog", "bird"],
                ["cat", "dog", "bird"],
                1.0,
                None,
            ),
            (
                "exact_match",
                ["apple", "banana", "cherry"],
                ["apple", "berry", "cherry"],
                0.666,
                0.7,
            ),
            # Similarity threshold cases
            (
                "similarity",
                ["apple", "banana"],
                ["app", "bananna"],
                1.0,  # Both above 0.7 threshold
                0.7,
            ),
            (
                "similarity",
                ["hello world", "foo bar"],
                ["helo world", "foo baz"],
                1.0,  # One above, one below threshold
                0.8,
            ),
            # Length ratio cases
            (
                "length_ratio",
                ["short", "medium text", "longer text"],
                ["shorter", "medium", "longer"],
                0.8303030303030302,
                None,
            ),
        ],
    )
    def test_score(
        self,
        metric_type: Literal["exact_match", "similarity", "length_ratio"],  # Key change
        y_true: List[str],
        y_pred: List[str],
        expected: float,
        threshold: Optional[float],
    ) -> None:
        # Mock targets and outputs
        targets = MagicMock()
        targets.response = pd.Series(y_true)
        outputs = MagicMock()
        outputs.response = pd.Series(y_pred)

        # Initialize metric
        metric = AutogenMetric(
            name="test_metric",
            metric_type=metric_type,
            similarity_threshold=threshold or 0.7,
            greater_is_better=True,
        )

        # Calculate and verify score
        assert pytest.approx(metric.score(targets, outputs), rel=0.01) == expected


# Test AutogenConversationMetric
class TestAutogenConversationMetric:
    @pytest.mark.parametrize(
        "metadata, check_term, check_err, expected",
        [
            # Perfect case
            (
                [{"terminated": True, "messages": []}] * 3,
                True,
                True,
                1.0,
            ),
            # Mixed quality case
            (
                [
                    {"terminated": True, "messages": []},
                    {"terminated": False, "messages": ["error"]},
                    {"terminated": True, "messages": ["error"]},
                ],
                True,
                True,
                pytest.approx((2 / 3) * (1 - 2 / 3), rel=0.01),
            ),
            # Only check termination
            (
                [{"terminated": False}] * 4,
                True,
                False,
                0.0,
            ),
        ],
    )
    def test_score(
        self,
        metadata: List[Dict[str, Any]],
        check_term: bool,
        check_err: bool,
        expected: float,
    ) -> None:
        # Mock outputs
        outputs = MagicMock()
        outputs.__getitem__.return_value = pd.Series(metadata)

        metric = AutogenConversationMetric(
            name="conv_metric",
            check_termination=check_term,
            check_error_messages=check_err,
            greater_is_better=True,
        )

        # Mock targets (not used)
        targets = MagicMock()

        assert metric.score(targets, outputs) == expected


# Test Threshold
class TestThreshold:
    @pytest.mark.parametrize(
        "threshold, greater_is_better",
        [
            (0.85, True),
            (10.0, False),
        ],
    )
    def test_to_mlflow(self, threshold: float, greater_is_better: bool) -> None:
        thresh = Threshold(
            threshold=threshold,
            greater_is_better=greater_is_better,
        )
        mlflow_thresh = thresh.to_mlflow()

        assert mlflow_thresh.threshold == threshold
        assert mlflow_thresh.greater_is_better == greater_is_better


if __name__ == "__main__":
    pytest.main(["-v", "test_metrics.py"])
