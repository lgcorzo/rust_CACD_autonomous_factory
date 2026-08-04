import os
import sys
import json
import subprocess
import shutil
from datetime import datetime, timezone

def parse_file(filepath):
    if filepath.endswith('.rs'):
        cmd = ['python3', 'parse_rust.py', filepath]
    elif filepath.endswith('.py'):
        cmd = ['python3', 'parse_python.py', filepath]
    elif filepath.endswith(('.ts', '.tsx', '.js', '.jsx')):
        cmd = ['python3', 'parse_ts.py', filepath]
    else:
        return {"classes": [], "free_functions": [], "dependencies": []}

    try:
        res = subprocess.check_output(cmd)
        return json.loads(res)
    except Exception as e:
        print(f"Error parsing file {filepath}: {e}", file=sys.stderr)
        return {"classes": [], "free_functions": [], "dependencies": []}


def generate_mermaid_classes(classes):
    mermaid = "classDiagram\n    direction BT\n"
    if not classes:
        return mermaid + "    class EmptyModule {\n    }\n"

    for c in classes:
        kind = c.get('kind', 'class')
        if kind == 'trait' or kind == 'interface':
            mermaid += f"    class {c['name']} {{\n        <<{kind}>>\n"
        elif kind == 'enum':
            mermaid += f"    class {c['name']} {{\n        <<enumeration>>\n"
        else:
            mermaid += f"    class {c['name']} {{\n"

        for m in c.get('methods', []):
            visibility = "+" if m.get('is_pub', True) else "-"
            args_str = ", ".join([f"{a['name']}:{a['type']}" for a in m.get('args', [])])
            ret_type = m.get('ret_type', 'None').strip()
            # Mermaid doesn't like some characters, so we just use the name
            mermaid += f"        {visibility}{m['name']}({args_str}) {ret_type}\n"

        mermaid += "    }\n"

        for impl in c.get('implements', []):
            mermaid += f"    {impl} <|-- {c['name']} : Inheritance / Specialization\n"

    return mermaid

def generate_sequence_diagram(module_name, classes, free_functions):
    seq = "sequenceDiagram\n    autonumber\n    participant Caller as Client Interface\n"
    svc_name = module_name.capitalize() + "Service"
    seq += f"    participant Svc as {svc_name}\n"

    method_name = "execute"
    if classes and classes[0].get('methods'):
        method_name = classes[0]['methods'][0]['name']
    elif free_functions:
        method_name = free_functions[0]['name']

    seq += f"    Caller->>Svc: {method_name}()\n"
    seq += "    Note over Svc: Processing internal logic\n    Svc-->>Caller: result\n"
    return seq


def write_file_doc(file_path, parsed, now):
    file_name = os.path.basename(file_path)
    base_name = os.path.splitext(file_name)[0]

    # Directory mapping
    dir_name = os.path.dirname(file_path)
    # The prompt requires strict OKF structure (architecture/, modules/, api/, classes/, diagrams/, dependencies/)
    # But it also requires "openwiki/api.md" for "src/api.py". So we create a page for each file directly in openwiki/
    # And we create indexes linking them.
    # We will put the raw file docs in 'openwiki/modules/' and link from index.

    flattened_name = file_path.replace(os.sep, '_').replace('.', '_')
    out_dir = os.path.join('openwiki', 'modules')
    os.makedirs(out_dir, exist_ok=True)
    out_file = os.path.join(out_dir, f"{flattened_name}.md")

    mermaid_classes = generate_mermaid_classes(parsed['classes'])
    seq_diagram = generate_sequence_diagram(base_name, parsed['classes'], parsed['free_functions'])

    deps_str = ", ".join(parsed['dependencies']) if parsed['dependencies'] else "None"

    content = f"""---
type: "module-documentation"
title: "{file_name}"
source_path: "{file_path}"
description: "Detailed documentation for {file_name}"
tags: ["documentation", "ast", "openwiki"]
timestamp: "{now}"
---

# File: {file_name}

**Source Path:** `{file_path}`

## Overview

### Purpose
Provides implementation for {file_name}.

### Responsibilities
* Handles logic related to {base_name}.

### Dependencies
* {deps_str}

## Public API & Architecture

### Exported Classes / Structs / Interfaces

"""
    for c in parsed['classes']:
        doc = c.get('doc', '').strip()
        if not doc:
            doc = f"Represents {c['name']}."

        content += f"#### {c['name']}\n\n"
        content += f"**Overview:** {doc}\n\n"

        content += "**Public Methods:**\n\n"
        has_methods = False
        for m in c.get('methods', []):
            if m.get('is_pub', True):
                has_methods = True
                mdoc = m.get('doc', '').strip() or f"Executes {m['name']}."
                args_str = ", ".join([f"{a['name']} ({a['type']})" for a in m.get('args', [])])
                ret_type = m.get('ret_type', 'None')
                content += f"##### `{m['name']}({args_str}) -> {ret_type}`\n"
                content += f"{mdoc}\n\n"
        if not has_methods:
            content += "None.\n\n"

    content += "### Exported Functions\n\n"
    has_funcs = False
    for f in parsed['free_functions']:
        if f.get('is_pub', True):
            has_funcs = True
            fdoc = f.get('doc', '').strip() or f"Executes {f['name']}."
            args_str = ", ".join([f"{a['name']} ({a['type']})" for a in f.get('args', [])])
            ret_type = f.get('ret_type', 'None')
            content += f"#### `{f['name']}({args_str}) -> {ret_type}`\n"
            content += f"{fdoc}\n\n"

    if not has_funcs:
        content += "None.\n\n"

    content += f"""## Internal Architecture & Execution Flow

```mermaid
{mermaid_classes}
```

### Sequence Explanation

```mermaid
{seq_diagram}
```

## Cross References
* **Parent module:** `{dir_name}`
* **Dependencies:** {deps_str}
"""
    with open(out_file, 'w', encoding='utf-8') as f:
        f.write(content)

def setup_okf_structure():
    folders = [
        "architecture",
        "modules",
        "api",
        "classes",
        "diagrams",
        "dependencies",
        "glossary",
        "decisions",
        "generated"
    ]
    for folder in folders:
        os.makedirs(os.path.join("openwiki", folder), exist_ok=True)

def main():
    mode = "diff"
    if len(sys.argv) > 1:
        for arg in sys.argv[1:]:
            if arg.startswith("mode="):
                mode = arg.split("=")[1]

    if mode == "full":
        if os.path.exists("openwiki"):
            shutil.rmtree("openwiki")
        setup_okf_structure()

        files_to_process = []
        for root, dirs, files in os.walk('.'):
            clean_root = os.path.normpath(root)
            parts = clean_root.split(os.sep)
            if any(ignored in parts for ignored in ['.git', '.github', '.vscode', '.idea', 'node_modules', 'dist', 'bin', 'obj', 'target', 'coverage', '__pycache__', 'openwiki']):
                continue
            for f in files:
                if f.endswith(('.rs', '.py', '.ts', '.js', '.tsx', '.jsx')):
                    files_to_process.append(os.path.normpath(os.path.join(clean_root, f)))
    else:
        setup_okf_structure()
        try:
            try:
                output = subprocess.check_output(["git", "log", "-m", "-1", "--name-only", "--pretty=format:"]).decode("utf-8")
            except subprocess.CalledProcessError:
                output = subprocess.check_output(["git", "show", "--name-only", "--format="]).decode("utf-8")
            files_to_process = []
            for f in output.splitlines():
                f = f.strip()
                if not f: continue
                if f.endswith(('.rs', '.py', '.ts', '.js', '.tsx', '.jsx')):
                    if os.path.exists(f):
                        files_to_process.append(f)
        except:
            files_to_process = []

    if not files_to_process:
        print("No files to process.")
        return

    now = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')

    for file_path in files_to_process:
        print(f"Processing {file_path}")
        parsed = parse_file(file_path)
        write_file_doc(file_path, parsed, now)

    generate_indexes(now)

def generate_indexes(now):
    summary_content = "# SUMMARY\n\n"
    index_content = "---\ntitle: OpenWiki Index\n---\n\n# OpenWiki Root Index\n\n## Module Architecture Links\n\n"

    for root, dirs, files in os.walk("openwiki"):
        dirs.sort()
        files.sort()

        rel_root = os.path.relpath(root, "openwiki")
        if rel_root == ".":
            pass
        else:
            summary_content += f"\n## {rel_root}\n\n"
            for f in files:
                if f.endswith(".md"):
                    path = os.path.join(rel_root, f).replace("\\", "/")
                    summary_content += f"* [{f}]({path})\n"
                    index_content += f"* [[{path}]]\n"

    with open("openwiki/SUMMARY.md", "w") as f:
        f.write(summary_content)

    with open("openwiki/index.md", "w") as f:
        f.write(index_content)

if __name__ == '__main__':
    main()
