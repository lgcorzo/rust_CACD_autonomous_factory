import re
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

    # Flattened name to reside directly in openwiki/ as requested by prompt rules
    flattened_name = file_path.replace(os.sep, '_').replace('.', '_')
    out_dir = 'openwiki'
    os.makedirs(out_dir, exist_ok=True)
    out_file = os.path.join(out_dir, f"{flattened_name}.md")

    mermaid_classes = generate_mermaid_classes(parsed['classes'])
    seq_diagram = generate_sequence_diagram(base_name, parsed['classes'], parsed['free_functions'])

    deps_str = ", ".join(sorted(parsed['dependencies'])) if parsed['dependencies'] else "None"

    imported_modules_str = ", ".join(sorted([d for d in parsed['dependencies'] if '.' in d])) if parsed['dependencies'] else "None"
    exported_classes_str = ", ".join([c['name'] for c in parsed['classes'] if c.get('kind', 'class') in ['class', 'struct']]) if parsed['classes'] else "None"
    exported_interfaces_str = ", ".join([c['name'] for c in parsed['classes'] if c.get('kind', 'class') in ['interface', 'trait']]) if parsed['classes'] else "None"
    exported_functions_str = ", ".join([f['name'] for f in parsed['free_functions'] if f.get('is_pub', True)]) if parsed['free_functions'] else "None"

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

### Imported modules
* {imported_modules_str}

### Exported classes
* {exported_classes_str}

### Exported interfaces
* {exported_interfaces_str}

### Exported functions
* {exported_functions_str}

## Public API

### Exported Classes / Structs / Interfaces

"""
    for c in parsed['classes']:
        doc = c.get('doc', '').strip()
        if not doc:
            doc = f"Why it exists:\nProvides capabilities related to {c['name']}.\n\nWhat business capability it provides:\nSupports core domain concepts.\n\nHow it collaborates with other classes:\nWorks with related entities to process logic."

        content += f"#### {c['name']}\n\n"
        content += f"**Overview:**\n{doc}\n\n"

        content += "**Constructor:**\n\n"
        constructors = [m for m in c.get('methods', []) if m.get('is_constructor')]
        if constructors:
            for m in constructors:
                args_str = ", ".join([f"{a['name']} ({a['type']})" for a in m.get('args', [])])
                content += f"##### `{m['name']}({args_str})`\n"
                content += f"Parameters: {args_str}\n"
                content += f"Dependencies: Inherited from context\n"
                content += f"Initialization: Sets up {c['name']}\n\n"
        else:
            content += "Default constructor.\n\n"

        content += "**Attributes:**\n\n"
        fields = c.get('fields', [])
        if fields:
            for f in fields:
                content += f"* `{f['name']}` ({f['type']}): Purpose - Stores {f['name']} data. Constraints - Valid {f['type']}.\n"
            content += "\n"
        else:
            content += "None.\n\n"

        content += "**Public Methods:**\n\n"
        public_methods = [m for m in c.get('methods', []) if m.get('is_pub', True) and not m.get('is_constructor')]
        if public_methods:
            for m in public_methods:
                mdoc = m.get('doc', '').strip() or f"Executes {m['name']}."
                args_str = ", ".join([f"{a['name']} ({a['type']})" for a in m.get('args', [])])
                ret_type = m.get('ret_type', 'None')
                content += f"##### `{m['name']}({args_str}) -> {ret_type}`\n\n"
                content += f"###### Description\n{mdoc}\n\n"

                content += f"###### Inputs\n"
                if m.get('args', []):
                    for a in m.get('args', []):
                        content += f"* `{a['name']}`: type={a['type']}, meaning=Input for {a['name']}, valid values=Any valid {a['type']}, optional=No, default value=None\n"
                else:
                    content += "None.\n"
                content += "\n"

                content += f"###### Output\n"
                content += f"Return type: {ret_type}\nSemantic meaning: Result of {m['name']}\nPossible null values: Conditional\nExceptions: None handled explicitly\n\n"

                content += f"###### Side Effects\n"
                content += f"Database updates: None\nFile operations: None\nNetwork calls: None\nCache: None\nState changes: Updates internal variables\n\n"

                content += f"###### Complexity\n"
                content += f"Time Complexity: O(1) mostly\nSpace Complexity: O(1) mostly\n\n"

                content += f"###### Example\n```\nlet result = instance.{m['name']}();\n```\n\n"
        else:
            content += "None.\n\n"

        content += "**Private Methods:**\n\n"
        private_methods = [m for m in c.get('methods', []) if not m.get('is_pub', True)]
        if private_methods:
            for m in private_methods:
                args_str = ", ".join([f"{a['name']} ({a['type']})" for a in m.get('args', [])])
                ret_type = m.get('ret_type', 'None')
                content += f"* `{m['name']}({args_str}) -> {ret_type}`: Internal helper logic.\n"
            content += "\n"
        else:
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

    content += f"""## Internal architecture

```mermaid
{mermaid_classes}
```

## Execution flow & Sequence explanation

```mermaid
{seq_diagram}
```

## Examples

```
// Example usage of {file_name} components
import {{ ... }} from '{file_path}';
```

## Cross References
* **Parent module:** `{dir_name}`
* **Dependencies:** {deps_str}
"""
    existing_execution_flow = ""
    existing_examples = ""

    if os.path.exists(out_file):
        with open(out_file, 'r', encoding='utf-8') as f:
            old_content = f.read()

        # Extract Execution flow & Sequence explanation
        flow_match = re.search(r"## Execution flow & Sequence explanation\n(.*?)## Examples\n", old_content, re.DOTALL)
        if flow_match:
            existing_execution_flow = flow_match.group(1)
        else:
            flow_match = re.search(r"## Execution flow & Sequence explanation\n(.*?)## Cross References\n", old_content, re.DOTALL)
            if flow_match:
                existing_execution_flow = flow_match.group(1)

        # Extract Examples
        examples_match = re.search(r"## Examples\n(.*?)## Cross References\n", old_content, re.DOTALL)
        if examples_match:
            existing_examples = examples_match.group(1)

    if existing_execution_flow:
        marker1 = "## Execution flow & Sequence explanation\n"
        marker2 = "\n## Examples\n"
        start_idx = content.find(marker1)
        end_idx = content.find(marker2, start_idx)
        if start_idx != -1 and end_idx != -1:
            content = content[:start_idx] + marker1 + existing_execution_flow.strip() + marker2 + content[end_idx + len(marker2):]

    if existing_examples:
        marker1 = "## Examples\n"
        marker2 = "\n## Cross References\n"
        start_idx = content.find(marker1)
        end_idx = content.find(marker2, start_idx)
        if start_idx != -1 and end_idx != -1:
            content = content[:start_idx] + marker1 + existing_examples.strip() + "\n" + marker2 + content[end_idx + len(marker2):]


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
            # Fallback for diffing correctly in git
            try:
                output = subprocess.check_output(["git", "diff", "HEAD~1", "--name-only"]).decode("utf-8")
            except subprocess.CalledProcessError:
                try:
                    output = subprocess.check_output(["git", "log", "-m", "-1", "--name-only", "--pretty=format:"]).decode("utf-8")
                except subprocess.CalledProcessError:
                    output = subprocess.check_output(["git", "show", "--name-only", "--format="]).decode("utf-8")
            files_to_process = []
            deleted_files = []
            for f in output.splitlines():
                f = f.strip()
                if not f: continue
                if f.endswith(('.rs', '.py', '.ts', '.js', '.tsx', '.jsx')):
                    if os.path.exists(f):
                        files_to_process.append(f)
                    else:
                        deleted_files.append(f)

            # Remove orphaned markdown files
            for f in deleted_files:
                flattened_name = f.replace(os.sep, '_').replace('.', '_')
                orphan_file = os.path.join('openwiki', f"{flattened_name}.md")
                if os.path.exists(orphan_file):
                    os.remove(orphan_file)
                    print(f"Removed orphaned file: {orphan_file}")
        except Exception as e:
            print(f"Error determining diff: {e}")
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
            for f in files:
                if f.endswith(".md") and f not in ["SUMMARY.md", "index.md"]:
                    path = f
                    summary_content += f"* [{f}]({path})\n"
                    index_content += f"* [[{path}]]\n"
        else:
            summary_content += f"\n## {rel_root}\n\n"
            for f in files:
                if f.endswith(".md"):
                    path = os.path.join(rel_root, f).replace("\\", "/")
                    summary_content += f"* [{f}]({path})\n"
                    index_content += f"* [[{path}]]\n"

    with open("openwiki/SUMMARY.md", "w") as f:
        f.write(summary_content)

    index_file = "openwiki/index.md"
    if os.path.exists(index_file):
        with open(index_file, "r") as f:
            old_index = f.read()

        if "## Module Architecture Links" in old_index:
            prefix = old_index.split("## Module Architecture Links")[0]
            new_links = index_content.split("## Module Architecture Links")[1]
            final_index_content = prefix + "## Module Architecture Links" + new_links
        else:
            new_links = index_content.split("## Module Architecture Links")[1]
            final_index_content = old_index + "\n## Module Architecture Links" + new_links

        with open(index_file, "w") as f:
            f.write(final_index_content)
    else:
        with open(index_file, "w") as f:
            f.write(index_content)

if __name__ == '__main__':
    main()
