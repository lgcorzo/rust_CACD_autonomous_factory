"""High-level jobs of the project."""

# %% IMPORTS

from autogen_team.jobs.evaluations import EvaluationsJob
from autogen_team.jobs.explanations import ExplanationsJob
from autogen_team.jobs.inference import InferenceJob
from autogen_team.jobs.promotion import PromotionJob
from autogen_team.jobs.training import TrainingJob
from autogen_team.jobs.tuning import TuningJob

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
