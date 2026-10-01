#!/usr/bin/env python3
"""
scripts/test_prepare_wiki_for_github.py

Unit tests for prepare_wiki_for_github.py ensuring:
- Unique slug generation without collisions
- Correct forward link rewriting (omitting .md and directories)
- Anchor preservation and external link preservation
- Full integrity of the repository's wiki/ directory
"""

import os
import shutil
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(__file__))

from prepare_wiki_for_github import (
    build_slug_map,
    transform_content_forward,
    export_wiki_forward,
    export_wiki_reverse,
)


class TestPrepareWikiForGithub(unittest.TestCase):
    def setUp(self):
        self.test_dir = tempfile.mkdtemp()

    def tearDown(self):
        shutil.rmtree(self.test_dir)

    def test_link_transformation_rules(self):
        slug_map = {
            "Home.md": "Home",
            "security/verification_triad.md": "verification_triad",
            "architecture/business_context.md": "business_context",
            "modules/core/lib.md": "core_lib",
            "modules/application/agents/rustant.md": "application_agents_rustant",
        }

        # 1. Root to section link
        content = "Link: [Verification Triad](security/verification_triad.md)"
        transformed, count = transform_content_forward(content, "Home.md", slug_map)
        self.assertEqual(count, 1)
        self.assertEqual(transformed, "Link: [Verification Triad](verification_triad)")

        # 2. Section to section link with ../
        content = "Link: [Context](../architecture/business_context.md)"
        transformed, count = transform_content_forward(content, "security/verification_triad.md", slug_map)
        self.assertEqual(count, 1)
        self.assertEqual(transformed, "Link: [Context](business_context)")

        # 3. Link with anchor
        content = "Link: [Gates](security/verification_triad.md#gate-specifications)"
        transformed, count = transform_content_forward(content, "Home.md", slug_map)
        self.assertEqual(count, 1)
        self.assertEqual(transformed, "Link: [Gates](verification_triad#gate-specifications)")

        # 4. Intra-page anchor should NOT change
        content = "Link: [Self Section](#gate-specifications)"
        transformed, count = transform_content_forward(content, "Home.md", slug_map)
        self.assertEqual(count, 0)
        self.assertEqual(transformed, "Link: [Self Section](#gate-specifications)")

        # 5. External link should NOT change
        content = "External: [GitHub](https://github.com/lgcorzo/repo)"
        transformed, count = transform_content_forward(content, "Home.md", slug_map)
        self.assertEqual(count, 0)
        self.assertEqual(transformed, "External: [GitHub](https://github.com/lgcorzo/repo)")

        # 6. Deep module link
        content = "Related: [Core Lib](../../core/lib.md)"
        transformed, count = transform_content_forward(content, "modules/application/agents/rustant.md", slug_map)
        self.assertEqual(count, 1)
        self.assertEqual(transformed, "Related: [Core Lib](core_lib)")

        # 7. Image tags should NOT be converted to page links
        content = "Image: ![Diagram](https://example.com/diag.png)"
        transformed, count = transform_content_forward(content, "Home.md", slug_map)
        self.assertEqual(count, 0)
        self.assertEqual(transformed, "Image: ![Diagram](https://example.com/diag.png)")

    def test_full_export_and_reverse_cycle(self):
        # Create mock wiki structure
        wiki_dir = os.path.join(self.test_dir, "wiki")
        os.makedirs(os.path.join(wiki_dir, "architecture"), exist_ok=True)
        os.makedirs(os.path.join(wiki_dir, "security"), exist_ok=True)

        home_content = "# Home\nSee [Context](architecture/business_context.md) and [Triad](security/verification_triad.md).\n"
        context_content = "# Context\nSee [Triad](../security/verification_triad.md).\n"
        triad_content = "# Triad\nBack to [Home](../Home.md).\n"

        with open(os.path.join(wiki_dir, "Home.md"), "w") as fp:
            fp.write(home_content)
        with open(os.path.join(wiki_dir, "architecture", "business_context.md"), "w") as fp:
            fp.write(context_content)
        with open(os.path.join(wiki_dir, "security", "verification_triad.md"), "w") as fp:
            fp.write(triad_content)

        # Forward export to temp flat dir
        export_dir = os.path.join(self.test_dir, "wiki_temp")
        export_wiki_forward(wiki_dir, export_dir)

        self.assertTrue(os.path.exists(os.path.join(export_dir, "Home.md")))
        self.assertTrue(os.path.exists(os.path.join(export_dir, "business_context.md")))
        self.assertTrue(os.path.exists(os.path.join(export_dir, "verification_triad.md")))

        with open(os.path.join(export_dir, "Home.md"), "r") as fp:
            exported_home = fp.read()
        self.assertIn("[Context](business_context)", exported_home)
        self.assertIn("[Triad](verification_triad)", exported_home)
        self.assertNotIn(".md", exported_home)

        # Test reverse sync
        reverse_dir = os.path.join(self.test_dir, "wiki_reverse")
        shutil.copytree(wiki_dir, reverse_dir)

        # Modify a file in export_dir (as if edited in GitHub Wiki UI)
        with open(os.path.join(export_dir, "business_context.md"), "w") as fp:
            fp.write("# Context Updated\nSee [Triad](verification_triad) again.\n")

        export_wiki_reverse(export_dir, reverse_dir)

        with open(os.path.join(reverse_dir, "architecture", "business_context.md"), "r") as fp:
            reversed_context = fp.read()
        self.assertIn("# Context Updated", reversed_context)
        self.assertIn("[Triad](../security/verification_triad.md)", reversed_context)

    def test_active_repo_wiki_integrity(self):
        """Verifies 100% resolution on the actual repository wiki/ directory."""
        repo_wiki = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "wiki"))
        if not os.path.exists(repo_wiki):
            self.skipTest("wiki/ directory not found relative to script")

        slug_map = build_slug_map(repo_wiki)
        self.assertEqual(len(slug_map), 100, f"Expected 100 wiki files, found {len(slug_map)}")

        export_dir = os.path.join(self.test_dir, "repo_export")
        export_wiki_forward(repo_wiki, export_dir)

        exported_files = [f for f in os.listdir(export_dir) if f.endswith(".md")]
        self.assertEqual(len(exported_files), 100)

        # Ensure _Sidebar.md exists and is populated
        sidebar_path = os.path.join(export_dir, "_Sidebar.md")
        self.assertTrue(os.path.exists(sidebar_path))
        with open(sidebar_path, "r") as fp:
            sidebar_content = fp.read()
        self.assertIn("[Verification Triad](verification_triad)", sidebar_content)
        self.assertIn("[Business Context](business_context)", sidebar_content)
        self.assertNotIn(".md)", sidebar_content)


if __name__ == "__main__":
    unittest.main()
