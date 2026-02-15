from typing import List, Optional, Dict, Any
from pydantic import BaseModel, Field


class MissionStart(BaseModel):
    """Event payload to start a new autonomous mission."""

    mission_id: str = Field(..., description="Unique identifier for the mission")
    goal: str = Field(..., description="High-level goal description")
    repository_path: str = Field(..., description="Path to the repository to modify")
    context: Optional[Dict[str, Any]] = Field(
        default_factory=dict, description="Additional context"
    )


class TaskAssignment(BaseModel):
    """Payload for assigning a task to a Coder Agent."""

    task_id: str
    mission_id: str
    description: str
    relevant_files: List[str]
    constraints: Optional[str] = None


class TaskResult(BaseModel):
    """Result from a Coder Agent execution."""

    task_id: str
    mission_id: str
    status: str = Field(..., pattern="^(completed|failed)$")
    diff: Optional[str] = None
    file_changes: List[str] = Field(default_factory=list)
    error_message: Optional[str] = None


class ReviewResult(BaseModel):
    """Result from a Reviewer Agent."""

    mission_id: str
    approved: bool
    comments: List[str]
    suggested_changes: Optional[str] = None
