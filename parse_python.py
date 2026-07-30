import ast
import sys
import json

def parse_python_file(filepath):
    classes = []
    free_functions = []
    dependencies = []

    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        tree = ast.parse(content, filename=filepath)

        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                for alias in node.names:
                    dependencies.append(alias.name)
            elif isinstance(node, ast.ImportFrom):
                if node.module:
                    dependencies.append(node.module)

            elif isinstance(node, ast.ClassDef):
                methods = []
                for child in node.body:
                    if isinstance(child, ast.FunctionDef):
                        methods.append({
                            'name': child.name,
                            'line': child.lineno
                        })

                implements = []
                for base in node.bases:
                    if isinstance(base, ast.Name):
                        implements.append(base.id)
                    elif isinstance(base, ast.Attribute):
                        implements.append(base.attr)

                classes.append({
                    'name': node.name,
                    'kind': 'class',
                    'line': node.lineno,
                    'methods': methods,
                    'implements': implements
                })

            # Module-level functions
            elif isinstance(node, ast.FunctionDef):
                # Check if it's not a method (very simplified logic: if parent is Module)
                free_functions.append({
                    'name': node.name,
                    'line': node.lineno
                })

        # Filter out methods from free_functions (since ast.walk traverses everything)
        class_lines = [(c['line'], getattr(ast.get_source_segment(content, next((n for n in ast.walk(tree) if getattr(n, 'lineno', -1) == c['line'] and isinstance(n, ast.ClassDef)), None)), 'count', lambda x: 0)('\n') + c['line']) for c in classes]
        # actually, this is too hard to filter easily. Let's just do a first-level pass instead of ast.walk for classes and free functions
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)

    return classes, free_functions, list(set(dependencies))

def parse_top_level(filepath):
    classes = []
    free_functions = []
    dependencies = set()

    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        tree = ast.parse(content, filename=filepath)

        for node in tree.body:
            if isinstance(node, ast.Import):
                for alias in node.names:
                    dependencies.add(alias.name)
            elif isinstance(node, ast.ImportFrom):
                if node.module:
                    dependencies.add(node.module)

            elif isinstance(node, ast.ClassDef):
                methods = []
                for child in node.body:
                    if isinstance(child, ast.FunctionDef):
                        methods.append({
                            'name': child.name,
                            'line': child.lineno
                        })

                implements = []
                for base in node.bases:
                    if isinstance(base, ast.Name):
                        implements.append(base.id)
                    elif isinstance(base, ast.Attribute):
                        implements.append(base.attr)

                classes.append({
                    'name': node.name,
                    'kind': 'class',
                    'line': node.lineno,
                    'methods': methods,
                    'implements': implements
                })

            elif isinstance(node, ast.FunctionDef):
                free_functions.append({
                    'name': node.name,
                    'line': node.lineno
                })
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)

    print(json.dumps({
        "classes": classes,
        "free_functions": free_functions,
        "dependencies": list(dependencies)
    }))

if __name__ == '__main__':
    if len(sys.argv) < 2:
        sys.exit(1)
    parse_top_level(sys.argv[1])
