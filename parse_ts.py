import tree_sitter_typescript as tsts
from tree_sitter import Language, Parser
import sys
import json

def get_node_text(node, source_bytes):
    return source_bytes[node.start_byte:node.end_byte].decode('utf-8')

def parse_ts(filepath):
    try:
        with open(filepath, 'rb') as f:
            source_bytes = f.read()

        TS_LANGUAGE = Language(tsts.language_typescript())
        parser = Parser(TS_LANGUAGE)
        tree = parser.parse(source_bytes)

        classes = []
        free_functions = []
        dependencies = []

        def traverse(node):
            if node.type in ['import_statement']:
                source_node = node.child_by_field_name('source')
                if source_node:
                    text = get_node_text(source_node, source_bytes)
                    dependencies.append(text.strip("'\""))

            elif node.type in ['class_declaration', 'interface_declaration']:
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = get_node_text(name_node, source_bytes)
                    kind = node.type.split('_')[0]
                    classes.append({
                        'name': name,
                        'kind': kind,
                        'doc': '',
                        'methods': [],
                        'fields': [],
                        'implements': []
                    })

            elif node.type == 'function_declaration':
                if node.parent and node.parent.type == 'program':
                    name_node = node.child_by_field_name('name')
                    if name_node:
                        name = get_node_text(name_node, source_bytes)
                        free_functions.append({
                            'name': name,
                            'is_pub': True,
                            'doc': '',
                            'args': [],
                            'ret_type': 'any'
                        })

            for child in node.children:
                traverse(child)

        traverse(tree.root_node)

        return {
            "classes": classes,
            "free_functions": free_functions,
            "dependencies": list(set(dependencies))
        }
    except Exception as e:
        print(f"Error parsing {filepath}: {e}", file=sys.stderr)
        return {"classes": [], "free_functions": [], "dependencies": []}

if __name__ == '__main__':
    if len(sys.argv) > 1:
        print(json.dumps(parse_ts(sys.argv[1])))
