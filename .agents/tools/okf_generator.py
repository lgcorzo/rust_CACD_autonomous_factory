#!/usr/bin/env python3
import sys
import os
import subprocess
import re

def get_modified_files():
    try:
        try:
            output = subprocess.check_output(["git", "log", "-m", "-1", "--name-only", "--pretty=format:"]).decode("utf-8")
        except subprocess.CalledProcessError:
            output = subprocess.check_output(["git", "show", "--name-only", "--format="]).decode("utf-8")
        files = []
        for f in output.splitlines():
            f = f.strip()
            if not f:
                continue
            if not f.endswith(('.rs', '.py', '.ts', '.js')):
                continue

            # Ensure it's in a src/ or code/ folder, taking crates/ into account
            is_valid_dir = False
            parts = f.split('/')
            for p in parts[:-1]: # exclude the filename itself
                if p in ("src", "code"):
                    is_valid_dir = True
                    break

            if not is_valid_dir:
                continue

            files.append(f)
        return files
    except subprocess.CalledProcessError:
        return []

def get_git_hash():
    try:
        return subprocess.check_output(["git", "rev-parse", "--short", "HEAD"]).decode("utf-8").strip()
    except:
        return "unknown"

def generate_mermaid_class_diagram(content):
    matches = re.findall(r'(?:pub\s+)?\b(struct|enum|class|trait)\b\s+(\w+)', content)
    if not matches:
        return "classDiagram\n    class Empty"

    diagram = "classDiagram\n"
    for m_type, m_name in matches:
        if m_type == "trait":
            diagram += f"    class {m_name} {{\n        <<trait>>\n    }}\n"
        elif m_type == "enum":
            diagram += f"    class {m_name} {{\n        <<enumeration>>\n    }}\n"
        else:
            diagram += f"    class {m_name}\n"
    return diagram.strip()

def process_file(file_path):
    if not os.path.exists(file_path):
        return None

    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()

    file_name = os.path.basename(file_path)
    file_type = "module"
    if "agent" in file_path.lower():
        file_type = "class"

    class_diagram = generate_mermaid_class_diagram(content)
    git_hash = get_git_hash()

    wiki_name = file_path.replace("/", "_").replace("\\", "_")
    for ext in ['.rs', '.py', '.ts', '.js', '.md']:
        if wiki_name.endswith(ext):
            wiki_name = wiki_name[:-len(ext)]
            break

    wiki_file_path = f"wiki/{wiki_name}.md"

    if os.path.exists(wiki_file_path):
        with open(wiki_file_path, "r", encoding="utf-8") as f:
            old_md = f.read()

        # Update last_verified_commit
        old_md = re.sub(r'last_verified_commit: ".*?"', f'last_verified_commit: "{git_hash}"', old_md)

        # We also need to embed accurate class diagrams.
        # But we shouldn't rewrite unaffected components like Execution flow.
        if 'classDiagram' in old_md:
            # Replace old classDiagram block
            old_md = re.sub(r'classDiagram[\s\S]*?(?=```)', class_diagram + '\n', old_md)
        else:
            old_md += f"\n## Component Architecture\n\n```mermaid\n{class_diagram}\n```\n"

        # Ensure mandatory Source File relative path is present
        if "Source File:" not in old_md:
            header_pattern = rf"#(.*?)\n"
            match = re.search(header_pattern, old_md)
            if match:
                insert_pos = match.end()
                old_md = old_md[:insert_pos] + f"\nSource File: `{file_path}`\n" + old_md[insert_pos:]
            else:
                old_md = re.sub(r'(---[\s\S]*?---\n)', r'\1\nSource File: `' + file_path + '`\n\n', old_md)

        markdown = old_md
    else:
        markdown = f"""---
type: {file_type}
title: "{file_name}"
source_path: "{file_path}"
description: "Documentation for {file_path}"
tags: [rust, {file_type}]
last_verified_commit: "{git_hash}"
---

# {file_name}

Source File: `{file_path}`

## Component Architecture

```mermaid
{class_diagram}
```

## Execution Flow

```mermaid
flowchart TD
    Start --> End
```
"""

    os.makedirs("wiki", exist_ok=True)
    with open(wiki_file_path, "w", encoding="utf-8") as f:
        f.write(markdown)
    return wiki_name

def update_index(new_wiki_names):
    index_path = "wiki/index.md"
    if not os.path.exists(index_path):
        return

    with open(index_path, "r", encoding="utf-8") as f:
        content = f.read()

    lines = content.splitlines()

    changed = False
    for wiki_name in new_wiki_names:
        link1 = f"* [[{wiki_name}]]"
        link2 = f"- [[{wiki_name}]]"
        if link1 not in content and link2 not in content:
            lines.append(link1)
            changed = True

    if changed:
        with open(index_path, "w", encoding="utf-8") as f:
            f.write("\n".join(lines) + "\n")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        files = []
        for arg in sys.argv[1:]:
            if arg.endswith(('.rs', '.py', '.ts', '.js')) and (arg.startswith("src/") or arg.startswith("code/") or arg.startswith("crates/")):
                files.append(arg)
    else:
        files = get_modified_files()

    if not files:
        print("No source files modified in src/, code/, or crates/. Skipping documentation generation.")
        sys.exit(0)

    new_wiki_names = []
    for f in files:
        print(f"Processing {f}")
        wiki_name = process_file(f)
        if wiki_name:
            new_wiki_names.append(wiki_name)

    if new_wiki_names:
        update_index(new_wiki_names)
        print("Updated documentation.")
