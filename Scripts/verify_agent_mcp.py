"""Verification script for Agent-MCP integration."""

import asyncio
from autogen_team.application.agents.planner_agent import PlannerAgent
from autogen_team.application.agents.coder_agent import CoderAgent
from autogen_team.application.agents.tester_agent import TesterAgent
from autogen_team.application.agents.reviewer_agent import ReviewerAgent


async def main() -> None:
    print("🚀 Starting Agent-MCP Integration Verification...")

    # 1. Verify PlannerAgent
    print("\n[1] Testing PlannerAgent...")
    planner = PlannerAgent()
    try:
        plan = await planner.create_plan(
            goal="Create a hello world python script", repository_path="."
        )
        print(f"✅ Planner Result: {plan.keys() if isinstance(plan, dict) else plan}")
    except Exception as e:
        print(f"❌ Planner Failed: {e}")

    # 2. Verify CoderAgent
    print("\n[2] Testing CoderAgent...")
    coder = CoderAgent()
    try:
        task = {
            "id": "task-test",
            "description": "Create a file named hello.py with print('Hello World')",
            "files": ["hello.py"],
        }
        coder_result = await coder.execute_task(task)
        print(f"✅ Coder Result: {coder_result.keys() if isinstance(coder_result, dict) else coder_result}")
    except Exception as e:
        print(f"❌ Coder Failed: {e}")

    # 3. Verify TesterAgent
    print("\n[3] Testing TesterAgent...")
    tester = TesterAgent()
    try:
        tester_result = await tester.run_tests()
        print(f"✅ Tester Result: {tester_result.keys() if isinstance(tester_result, dict) else tester_result}")
    except Exception as e:
        print(f"❌ Tester Failed: {e}")

    # 4. Verify ReviewerAgent
    print("\n[4] Testing ReviewerAgent...")
    reviewer = ReviewerAgent()
    try:
        # Pass a dummy diff string
        review_result = await reviewer.review_changes(
            mission_id="mission-test",
            file_changes=["diff --git a/hello.py b/hello.py\n+print('Hello')"],
        )
        print(f"✅ Reviewer Result: Approved={review_result.approved}, Comments={review_result.comments}")
    except Exception as e:
        print(f"❌ Reviewer Failed: {e}")


if __name__ == "__main__":
    asyncio.run(main())
