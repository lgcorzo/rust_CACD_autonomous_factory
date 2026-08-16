import re
import os
import sys
import json
import subprocess
import shutil
from datetime import datetime, timezone

def generate_ai_description(entity_name, file_name, entity_type, original_doc=""):
    api_key = os.environ.get("OPENAI_API_KEY")
    if not api_key:
        return original_doc if original_doc else "No description provided."

    try:
        from openai import OpenAI
        client = OpenAI(api_key=api_key)
        prompt = f"Write a clear, concise natural language description for the {entity_type} named '{entity_name}' in the file '{file_name}'. Explain its purpose and intent based on its name."

        response = client.chat.completions.create(
            model="gpt-4o-mini",
            messages=[
                {"role": "system", "content": "You are a technical documentation assistant. Provide only the description, without quotes or conversational filler."},
                {"role": "user", "content": prompt}
            ],
            max_tokens=150
        )
        return response.choices[0].message.content.strip()
    except Exception as e:
        print(f"Error generating AI description: {e}", file=sys.stderr)
        return original_doc if original_doc else "No description provided."


def parse_file(filepath):
    if filepath.endswith('.rs'):
        cmd = ['python3', 'parse_rust.py', filepath]
    elif filepath.endswith('.py'):
        cmd = ['python3', 'parse_python.py', filepath]
    elif filepath.endswith(('.ts', '.tsx', '.js', '.jsx')):
        cmd = ['python3', 'parse_ts.py', filepath]
    elif filepath.endswith('.java'):
        cmd = ['python3', 'parse_java.py', filepath]
    elif filepath.endswith('.cs'):
        cmd = ['python3', 'parse_csharp.py', filepath]
    elif filepath.endswith('.c'):
        cmd = ['python3', 'parse_c.py', filepath]
    elif filepath.endswith('.cpp'):
        cmd = ['python3', 'parse_cpp.py', filepath]
    elif filepath.endswith('.go'):
        cmd = ['python3', 'parse_go.py', filepath]
    else:
        return {"classes": [], "free_functions": [], "dependencies": []}

    try:
        res = subprocess.check_output(cmd)
        return json.loads(res)
    except Exception as e:
        print(f"Error parsing file {filepath}: {e}", file=sys.stderr)
        return {"classes": [], "free_functions": [], "dependencies": []}


def generate_plantuml_classes(classes):
    puml = "@startuml\n"
    if not classes:
        return puml + "class EmptyModule {\n}\n@enduml\n"

    for c in classes:
        kind = c.get('kind', 'class')
        if kind == 'trait' or kind == 'interface':
            puml += f"interface {c['name']} {{\n"
        elif kind == 'enum':
            puml += f"enum {c['name']} {{\n"
        else:
            puml += f"class {c['name']} {{\n"

        for m in c.get('methods', []):
            visibility = "+" if m.get('is_pub', True) else "-"
            args_str = ", ".join([f"{a['name']}:{a['type']}" for a in m.get('args', [])])
            ret_type = m.get('ret_type', 'None').strip()
            puml += f"    {visibility}{m['name']}({args_str}) : {ret_type}\n"

        puml += "}\n"

        for impl in c.get('implements', []):
            puml += f"{impl} <|-- {c['name']} : extends/implements\n"

    puml += "@enduml\n"
    return puml

def generate_plantuml_sequence(module_name, classes, free_functions):
    seq = "@startuml\nautonumber\nparticipant \"Client Interface\" as Caller\n"
    svc_name = module_name.capitalize() + "Service"
    seq += f"participant \"{svc_name}\" as Svc\n"

    method_name = "execute"
    if classes and classes[0].get('methods'):
        method_name = classes[0]['methods'][0]['name']
    elif free_functions:
        method_name = free_functions[0]['name']

    seq += f"Caller -> Svc: {method_name}()\n"
    seq += "note right of Svc: Processing internal logic\nSvc --> Caller: result\n"
    seq += "@enduml\n"
    return seq


def write_file_doc(file_path, parsed, now):
    file_name = os.path.basename(file_path)
    base_name = os.path.splitext(file_name)[0]

    # Directory mapping
    dir_name = os.path.dirname(file_path)

    # Flattened name to reside directly in openwiki/ as requested by prompt rules
    name_without_ext = os.path.splitext(file_path)[0]
    flattened_name = name_without_ext.replace(os.sep, '_').replace('-', '_')
    out_dir = 'openwiki'
    os.makedirs(out_dir, exist_ok=True)
    out_file = os.path.join(out_dir, f"{flattened_name}.md")

    parsed['classes'].sort(key=lambda x: x['name'])
    parsed['free_functions'].sort(key=lambda x: x['name'])
    plantuml_classes = generate_plantuml_classes(parsed['classes'])
    seq_diagram = generate_plantuml_sequence(base_name, parsed['classes'], parsed['free_functions'])

    deps_str = ", ".join(sorted(parsed['dependencies'])) if parsed['dependencies'] else "None"

    imported = sorted([d for d in parsed['dependencies'] if '.' in d])
    imported_modules_str = ", ".join(imported) if imported else "None"

    exported_classes = [c['name'] for c in parsed['classes'] if c.get('kind', 'class') in ['class', 'struct']]
    exported_classes_str = ", ".join(exported_classes) if exported_classes else "None"

    exported_interfaces = [c['name'] for c in parsed['classes'] if c.get('kind', 'class') in ['interface', 'trait']]
    exported_interfaces_str = ", ".join(exported_interfaces) if exported_interfaces else "None"

    exported_functions = [f['name'] for f in parsed['free_functions'] if f.get('is_pub', True)]
    exported_functions_str = ", ".join(exported_functions) if exported_functions else "None"

    git_hash = get_git_hash()

    content = f"""---
type: "module-documentation"
title: "{file_name}"
source_path: "{file_path}"
description: "Detailed documentation for {file_name}"
tags: ["documentation", "ast", "openwiki"]
last_verified_commit: "{git_hash}"
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
            doc = generate_ai_description(c['name'], file_name, "class", doc)

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
                mdoc = m.get('doc', '').strip()
                if not mdoc:
                    mdoc = generate_ai_description(m['name'], file_name, "method", mdoc)
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
            fdoc = f.get('doc', '').strip()
            if not fdoc:
                fdoc = generate_ai_description(f['name'], file_name, "function", fdoc)
            args_str = ", ".join([f"{a['name']} ({a['type']})" for a in f.get('args', [])])
            ret_type = f.get('ret_type', 'None')
            content += f"#### `{f['name']}({args_str}) -> {ret_type}`\n"
            content += f"{fdoc}\n\n"

    if not has_funcs:
        content += "None.\n\n"

    content += f"""## Internal architecture

```plantuml
{plantuml_classes}
```

## Execution flow & Sequence explanation

```plantuml
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
        start_marker_flow = "## Execution flow & Sequence explanation\n"
        end_marker_flow_1 = "## Examples\n"
        end_marker_flow_2 = "## Cross References\n"

        start_idx_flow = old_content.find(start_marker_flow)
        if start_idx_flow != -1:
            start_content_flow = start_idx_flow + len(start_marker_flow)
            end_idx_flow = old_content.find(end_marker_flow_1, start_content_flow)
            if end_idx_flow == -1:
                end_idx_flow = old_content.find(end_marker_flow_2, start_content_flow)
            if end_idx_flow != -1:
                existing_execution_flow = old_content[start_content_flow:end_idx_flow]

        # Extract Examples
        start_marker_examples = "## Examples\n"
        end_marker_examples = "## Cross References\n"

        start_idx_examples = old_content.find(start_marker_examples)
        if start_idx_examples != -1:
            start_content_examples = start_idx_examples + len(start_marker_examples)
            end_idx_examples = old_content.find(end_marker_examples, start_content_examples)
            if end_idx_examples != -1:
                existing_examples = old_content[start_content_examples:end_idx_examples]

    if existing_execution_flow:
        marker1 = "## Execution flow & Sequence explanation\n"
        marker2 = "\n## Examples\n"
        start_idx = content.find(marker1)
        end_idx = content.find(marker2, start_idx)
        if start_idx != -1 and end_idx != -1:
            content = content[:start_idx] + marker1 + existing_execution_flow + marker2 + content[end_idx + len(marker2):]

    if existing_examples:
        marker1 = "## Examples\n"
        marker2 = "\n## Cross References\n"
        start_idx = content.find(marker1)
        end_idx = content.find(marker2, start_idx)
        if start_idx != -1 and end_idx != -1:
            content = content[:start_idx] + marker1 + existing_examples + marker2 + content[end_idx + len(marker2):]


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

def get_git_hash():
    try:
        return subprocess.check_output(["git", "rev-parse", "--short", "HEAD"], stderr=subprocess.DEVNULL).decode("utf-8").strip()
    except Exception:
        return "unknown"

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
                if f.endswith(('.rs', '.py', '.ts', '.js', '.tsx', '.jsx', '.java', '.cs', '.c', '.cpp', '.go')):
                    files_to_process.append(os.path.normpath(os.path.join(clean_root, f)))
    else:
        setup_okf_structure()
        try:
            # Fallback for diffing correctly in git
            try:
                output = subprocess.check_output(["git", "diff", "HEAD~1", "--name-only"], stderr=subprocess.DEVNULL).decode("utf-8")
            except subprocess.CalledProcessError:
                try:
                    output = subprocess.check_output(["git", "log", "-m", "-1", "--name-only", "--pretty=format:"], stderr=subprocess.DEVNULL).decode("utf-8")
                except subprocess.CalledProcessError:
                    output = subprocess.check_output(["git", "show", "--name-only", "--format="], stderr=subprocess.DEVNULL).decode("utf-8")
            files_to_process = []
            deleted_files = []
            for f in output.splitlines():
                f = f.strip()
                if not f: continue
                if f.endswith(('.rs', '.py', '.ts', '.js', '.tsx', '.jsx', '.java', '.cs', '.c', '.cpp', '.go')):
                    if os.path.exists(f):
                        files_to_process.append(f)
                    else:
                        deleted_files.append(f)

            # Remove orphaned markdown files
            for f in deleted_files:
                name_without_ext = os.path.splitext(f)[0]
                flattened_name = name_without_ext.replace(os.sep, '_').replace('-', '_')
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
    validate_links()

def validate_links():
    files = []
    for root, _, filenames in os.walk('openwiki'):
        for filename in filenames:
            if filename.endswith('.md'):
                files.append(os.path.join(root, filename))

    all_pages = set()
    for f in files:
        basename = os.path.basename(f)
        if basename.endswith('.md'):
            all_pages.add(basename[:-3])

    print("Validating links...")
    broken_links = 0
    orphan_pages = all_pages.copy()

    # Exclude root indexes from orphan check
    orphan_pages.discard("index")
    orphan_pages.discard("SUMMARY")

    for f in files:
        with open(f, 'r', encoding='utf-8') as file:
            content = file.read()
            # Simple regex to find wiki links [[link]] or markdown links [text](link)
            # This is just for validation, not text parsing of the source file
            wiki_links = re.findall(r'\[\[(.*?)\]\]', content)
            md_links = re.findall(r'\[.*?\]\((.*?)\)', content)

            for link in wiki_links:
                if link not in all_pages:
                    print(f"Warning: Broken wiki link [[{link}]] in {f}")
                    broken_links += 1
                orphan_pages.discard(link)

            for link in md_links:
                # Exclude external links and standard anchor links
                if not link.startswith('http') and not link.startswith('#'):
                    link_clean = link
                    # Handle relative paths from markdown links
                    if '/' in link:
                        link_clean = link.split('/')[-1]
                    if link_clean in all_pages:
                        orphan_pages.discard(link_clean)
                    elif not link.endswith('.md'): # it might be linking to a page that isn't found
                        pass # just simple validation

    if broken_links == 0:
        print("No broken links found.")
    else:
        print(f"Found {broken_links} broken links.")

    if not orphan_pages:
        print("No orphan pages found.")
    else:
        print(f"Found {len(orphan_pages)} orphan pages:")
        for page in list(orphan_pages)[:5]: # show up to 5
            print(f"  - {page}")
        if len(orphan_pages) > 5:
            print(f"  ... and {len(orphan_pages) - 5} more.")

def generate_indexes(now):
    summary_content = "# SUMMARY\n\n"
    index_content = "---\ntitle: OpenWiki Index\n---\n\n# OpenWiki Root Index\n\n## Auto-Generated Module Architecture Links\n\n"

    for root, dirs, files in os.walk("openwiki"):
        dirs.sort()
        files.sort()

        rel_root = os.path.relpath(root, "openwiki")
        if rel_root == ".":
            for f in files:
                if f.endswith(".md") and f not in ["SUMMARY.md", "index.md"]:
                    path = f
                    summary_content += f"* [{f[:-3]}]({path[:-3]})\n"
                    index_content += f"* [[{path[:-3]}]]\n"
        else:
            summary_content += f"\n## {rel_root}\n\n"
            for f in files:
                if f.endswith(".md"):
                    path = os.path.join(rel_root, f).replace("\\", "/")
                    summary_content += f"* [{f[:-3]}]({path[:-3]})\n"
                    index_content += f"* [[{path[:-3]}]]\n"

    with open("openwiki/SUMMARY.md", "w") as f:
        f.write(summary_content)

    index_file = "openwiki/index.md"
    if os.path.exists(index_file):
        with open(index_file, "r") as f:
            old_index = f.read()

        if "## Auto-Generated Module Architecture Links" in old_index:
            parts = old_index.split("## Auto-Generated Module Architecture Links")
            prefix = parts[0]
            # Try to find if there's any section after the auto generated one
            suffix = ""
            if len(parts) > 1:
                subparts = parts[1].split("\n## ", 1)
                if len(subparts) > 1:
                    suffix = "\n## " + subparts[1]
            new_links = index_content.split("## Auto-Generated Module Architecture Links")[1]
            final_index_content = prefix + "## Auto-Generated Module Architecture Links" + new_links.rstrip() + "\n" + suffix
        else:
            new_links = index_content.split("## Auto-Generated Module Architecture Links")[1]
            final_index_content = old_index + "\n## Auto-Generated Module Architecture Links" + new_links

        with open(index_file, "w") as f:
            f.write(final_index_content)
    else:
        with open(index_file, "w") as f:
            f.write(index_content)

if __name__ == '__main__':
    main()
