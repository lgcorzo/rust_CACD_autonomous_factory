"""High-level jobs of the project."""

# %% IMPORTS

from .evaluations import EvaluationsJob
from .explanations import ExplanationsJob
from .inference import InferenceJob
from .promotion import PromotionJob
from .training import TrainingJob
from .tuning import TuningJob

# %% TYPES

JobKind = TuningJob | TrainingJob | PromotionJob | InferenceJob | EvaluationsJob | ExplanationsJob

# %% EXPORTS

__all__ = [
    "TuningJob",
    "TrainingJob",
    "PromotionJob",
    "InferenceJob",
    "EvaluationsJob",
    "ExplanationsJob",
    "JobKind",
]
