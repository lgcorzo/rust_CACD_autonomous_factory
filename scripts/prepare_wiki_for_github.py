#!/usr/bin/env python3
"""
scripts/prepare_wiki_for_github.py

Prepares documentation from the modular repository directory (e.g. `wiki/`)
for GitHub Wiki (`.wiki.git`), ensuring all pages and links render natively
as rich HTML in the GitHub Wiki web interface rather than raw Markdown text.

Key transformations:
1. Flat Wiki Export:
   GitHub Wiki uses a flat namespace. Modular files in subdirectories are mapped
   to unique, collision-free wiki page slugs:
   - Root files (Home.md, _Sidebar.md, GLOSSARY.md, index.md, README.md) -> Root filenames
   - Section files (architecture/*, security/*, operations/*, quality/*) -> <file_basename>.md
   - Module files (modules/<crate>/<subpath>.md) -> <crate>_<subpath>.md
2. Link Transformation:
   Internal relative Markdown links (e.g. `[Triad](security/verification_triad.md)`)
   are converted to flat Wiki page references without `.md` extensions:
   e.g. `[Triad](verification_triad)`. This prevents GitHub Wiki from routing
   links to raw.githubusercontent.com.
3. Bidirectional Support:
   Supports `--reverse` mode to sync changes made via GitHub Wiki UI back to the
   hierarchical `wiki/` directory.
"""

import argparse
import glob
import os
import re
import sys
from typing import Dict, Tuple

# Markdown link regex (ignoring image links starting with '!')
LINK_REGEX = re.compile(r"(?<!!)\[([^\]]+)\]\(([^)]+)\)")


def build_slug_map(source_dir: str) -> Dict[str, str]:
    """
    Builds a bidirectional map: relative path in source_dir -> wiki slug.
    Returns dict mapping normalized source relative path -> wiki slug.
    """
    slug_map: Dict[str, str] = {}
    pattern = os.path.join(source_dir, "**", "*.md")
    files = glob.glob(pattern, recursive=True)

    for f in sorted(files):
        rel = os.path.normpath(os.path.relpath(f, source_dir))
        parts = rel.split(os.sep)

        if len(parts) == 1:
            slug = os.path.splitext(parts[0])[0]
        elif parts[0] in ("architecture", "security", "operations", "quality"):
            slug = os.path.splitext(parts[1])[0]
        elif parts[0] == "modules":
            mod_parts = parts[1:]
            mod_parts[-1] = os.path.splitext(mod_parts[-1])[0]
            slug = "_".join(mod_parts)
        else:
            clean_parts = list(parts)
            clean_parts[-1] = os.path.splitext(clean_parts[-1])[0]
            slug = "_".join(clean_parts)

        if slug in slug_map.values():
            colliding = [k for k, v in slug_map.items() if v == slug]
            raise ValueError(f"Slug collision detected: '{slug}' for '{rel}' and '{colliding[0]}'")

        slug_map[rel] = slug

    return slug_map


def transform_content_forward(content: str, source_rel: str, slug_map: Dict[str, str]) -> Tuple[str, int]:
    """
    Transforms internal Markdown links from relative file paths to flat wiki slugs.
    """
    source_dir = os.path.dirname(source_rel)
    replaced_count = 0

    def replace_link(match: re.Match) -> str:
        nonlocal replaced_count
        label = match.group(1)
        raw_target = match.group(2).strip()

        # Preserve external links and intra-page anchors
        if raw_target.startswith(("http://", "https://", "mailto:", "#")):
            return match.group(0)

        # Handle optional markdown link title: [label](target "title")
        parts = raw_target.split(None, 1)
        target_url = parts[0]
        title_suffix = f" {parts[1]}" if len(parts) > 1 else ""

        target_path = target_url.split("#")[0]
        target_anchor = ("#" + target_url.split("#")[1]) if "#" in target_url else ""

        # Resolve path relative to the current file's directory
        resolved = os.path.normpath(os.path.join(source_dir, target_path))

        # Check with or without .md extension
        if resolved not in slug_map and (resolved + ".md") in slug_map:
            resolved = resolved + ".md"

        if resolved in slug_map:
            slug = slug_map[resolved]
            replaced_count += 1
            return f"[{label}]({slug}{target_anchor}{title_suffix})"

        # If unresolved, leave untouched but warn
        print(f"Warning: unresolved internal link in {source_rel}: {raw_target} (resolved: {resolved})", file=sys.stderr)
        return match.group(0)

    transformed = LINK_REGEX.sub(replace_link, content)
    return transformed, replaced_count


def export_wiki_forward(source_dir: str, dest_dir: str, dry_run: bool = False) -> None:
    """
    Exports all markdown files from hierarchical source_dir to flat dest_dir
    with transformed wiki links.
    """
    slug_map = build_slug_map(source_dir)
    os.makedirs(dest_dir, exist_ok=True)

    total_files = len(slug_map)
    total_links_transformed = 0

    print(f"Exporting {total_files} wiki files from '{source_dir}' to '{dest_dir}'...")

    for rel_path, slug in slug_map.items():
        src_path = os.path.join(source_dir, rel_path)
        with open(src_path, "r", encoding="utf-8") as fp:
            content = fp.read()

        transformed, count = transform_content_forward(content, rel_path, slug_map)
        total_links_transformed += count

        # Destination filename:
        # Note: _Sidebar.md retains its leading underscore for GitHub Wiki sidebar detection
        dest_filename = f"{slug}.md"
        dest_path = os.path.join(dest_dir, dest_filename)

        if not dry_run:
            with open(dest_path, "w", encoding="utf-8") as fp:
                fp.write(transformed)

    print(f"Successfully processed {total_files} files.")
    print(f"Transformed {total_links_transformed} internal links to flat wiki slugs.")


def export_wiki_reverse(wiki_temp_dir: str, repo_wiki_dir: str, dry_run: bool = False) -> None:
    """
    Syncs changes made on GitHub Wiki (flat namespace) back to the repo's
    hierarchical wiki directory.
    """
    forward_slug_map = build_slug_map(repo_wiki_dir)
    reverse_slug_map = {slug: rel for rel, slug in forward_slug_map.items()}

    print(f"Syncing modified wiki files from '{wiki_temp_dir}' back to '{repo_wiki_dir}'...")
    updated_files = 0

    for filename in os.listdir(wiki_temp_dir):
        if not filename.endswith(".md"):
            continue
        slug = os.path.splitext(filename)[0]
        if slug not in reverse_slug_map:
            print(f"Warning: New or unmapped page '{slug}' found in wiki. Storing in root.", file=sys.stderr)
            target_rel = filename
        else:
            target_rel = reverse_slug_map[slug]

        src_file = os.path.join(wiki_temp_dir, filename)
        dest_file = os.path.join(repo_wiki_dir, target_rel)

        with open(src_file, "r", encoding="utf-8") as fp:
            content = fp.read()

        # Re-map slug links back to relative paths for local browsing
        target_dir = os.path.dirname(target_rel)

        def re_relativize(match: re.Match) -> str:
            label = match.group(1)
            raw_target = match.group(2).strip()

            if raw_target.startswith(("http://", "https://", "mailto:", "#")):
                return match.group(0)

            parts = raw_target.split(None, 1)
            target_url = parts[0]
            title_suffix = f" {parts[1]}" if len(parts) > 1 else ""

            target_slug = target_url.split("#")[0]
            target_anchor = ("#" + target_url.split("#")[1]) if "#" in target_url else ""

            if target_slug in reverse_slug_map:
                dest_rel_file = reverse_slug_map[target_slug]
                rel_back = os.path.relpath(dest_rel_file, target_dir)
                return f"[{label}]({rel_back}{target_anchor}{title_suffix})"

            return match.group(0)

        relativized = LINK_REGEX.sub(re_relativize, content)

        if not dry_run:
            os.makedirs(os.path.dirname(dest_file), exist_ok=True)
            with open(dest_file, "w", encoding="utf-8") as fp:
                fp.write(relativized)
        updated_files += 1

    print(f"Reverse sync complete: {updated_files} files processed.")


def main():
    parser = argparse.ArgumentParser(description="Prepare modular wiki docs for GitHub Wiki flat export.")
    parser.add_argument("--source", default="wiki", help="Source wiki directory (default: wiki)")
    parser.add_argument("--dest", default="wiki_temp", help="Destination directory (default: wiki_temp)")
    parser.add_argument("--reverse", action="store_true", help="Sync back from flat wiki to hierarchical repo wiki")
    parser.add_argument("--dry-run", action="store_true", help="Preview transformations without writing files")

    args = parser.parse_args()

    if args.reverse:
        export_wiki_reverse(args.source, args.dest, dry_run=args.dry_run)
    else:
        export_wiki_forward(args.source, args.dest, dry_run=args.dry_run)


if __name__ == "__main__":
    main()
