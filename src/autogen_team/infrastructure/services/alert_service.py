"""Alert Service - System notifications."""

from __future__ import annotations

import pydantic as pdt
from plyer import notification

from .logger_service import Service


class AlertsService(Service):
    """Service for sending notifications."""

    enable: bool = True
    app_name: str = "autogen_team"
    timeout: int | None = None

    def start(self) -> None:
        pass

    def notify(self, title: str, message: str) -> None:
        """Send a notification to the system."""
        if self.enable:
            try:
                notification.notify(title=title, message=message, app_name=self.app_name, timeout=self.timeout)
            except Exception:
                print(f"[{self.app_name}] {title}: {message} (Notification ignored: No usable implementation)")
        else:
            print(f"[{self.app_name}] {title}: {message}")
