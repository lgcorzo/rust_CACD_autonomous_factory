import ast
import sys
import json

def parse_python_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        tree = ast.parse(content, filename=filepath)

        classes = []
        free_functions = []
        dependencies = set()

        for node in ast.iter_child_nodes(tree):
            if isinstance(node, ast.Import):
                for alias in node.names:
                    dependencies.add(alias.name)
            elif isinstance(node, ast.ImportFrom):
                if node.module:
                    dependencies.add(node.module)
            elif isinstance(node, ast.ClassDef):
                methods = []
                fields = []
                for child in ast.iter_child_nodes(node):
                    if isinstance(child, ast.AnnAssign):
                        if isinstance(child.target, ast.Name):
                            fields.append({
                                'name': child.target.id,
                                'type': ast.unparse(child.annotation)
                            })
                    elif isinstance(child, ast.Assign):
                        for target in child.targets:
                            if isinstance(target, ast.Name):
                                fields.append({
                                    'name': target.id,
                                    'type': 'Any'
                                })
                    elif isinstance(child, ast.FunctionDef) or isinstance(child, ast.AsyncFunctionDef):
                        doc = ast.get_docstring(child) or ""
                        is_constructor = child.name == "__init__"

                        if is_constructor:
                            is_pub = True
                        else:
                            is_pub = not child.name.startswith('_') or (child.name.startswith('__') and child.name.endswith('__'))

                        args = []
                        for arg in child.args.args:
                            arg_type = ast.unparse(arg.annotation) if arg.annotation else "Any"
                            args.append({"name": arg.arg, "type": arg_type})

                        ret_type = ast.unparse(child.returns) if child.returns else "None"

                        methods.append({
                            'name': child.name,
                            'is_pub': is_pub,
                            'is_constructor': is_constructor,
                            'doc': doc,
                            'args': args,
                            'ret_type': ret_type
                        })

                        if is_constructor:
                            for stmt in child.body:
                                if isinstance(stmt, ast.Assign):
                                    for target in stmt.targets:
                                        if isinstance(target, ast.Attribute) and isinstance(target.value, ast.Name) and target.value.id == 'self':
                                            fields.append({
                                                'name': target.attr,
                                                'type': 'Any'
                                            })
                                elif isinstance(stmt, ast.AnnAssign):
                                    if isinstance(stmt.target, ast.Attribute) and isinstance(stmt.target.value, ast.Name) and stmt.target.value.id == 'self':
                                        fields.append({
                                            'name': stmt.target.attr,
                                            'type': ast.unparse(stmt.annotation)
                                        })

                unique_fields = []
                seen_fields = set()
                for f in fields:
                    if f['name'] not in seen_fields:
                        seen_fields.add(f['name'])
                        unique_fields.append(f)

                implements = []
                for base in node.bases:
                    if isinstance(base, ast.Name):
                        implements.append(base.id)
                    elif isinstance(base, ast.Attribute):
                        implements.append(base.attr)

                doc = ast.get_docstring(node) or ""
                classes.append({
                    'name': node.name,
                    'kind': 'class',
                    'doc': doc,
                    'methods': methods,
                    'implements': implements,
                    'fields': unique_fields
                })

            elif isinstance(node, ast.FunctionDef) or isinstance(node, ast.AsyncFunctionDef):
                doc = ast.get_docstring(node) or ""
                is_pub = not node.name.startswith('_')
                args = []
                for arg in node.args.args:
                    arg_type = ast.unparse(arg.annotation) if arg.annotation else "Any"
                    args.append({"name": arg.arg, "type": arg_type})

                ret_type = ast.unparse(node.returns) if node.returns else "None"

                free_functions.append({
                    'name': node.name,
                    'is_pub': is_pub,
                    'doc': doc,
                    'args': args,
                    'ret_type': ret_type
                })

        return {
            "classes": classes,
            "free_functions": free_functions,
            "dependencies": list(dependencies)
        }
    except Exception as e:
        print(f"Error parsing {filepath}: {e}", file=sys.stderr)
        return {"classes": [], "free_functions": [], "dependencies": []}

if __name__ == '__main__':
    if len(sys.argv) > 1:
        print(json.dumps(parse_python_file(sys.argv[1])))
