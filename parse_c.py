import tree_sitter_c as tsc
from tree_sitter import Language, Parser
import sys
import json
import os

def get_node_text(node, source_bytes):
    return source_bytes[node.start_byte:node.end_byte].decode('utf-8')

def parse_c(filepath):
    try:
        with open(filepath, 'rb') as f:
            source_bytes = f.read()

        lang = Language(tsc.language())
        parser = Parser(lang)
        tree = parser.parse(source_bytes)

        classes = [] # structs in C
        free_functions = []
        dependencies = []

        def traverse(node):
            if node.type == 'preproc_include':
                path_node = node.child_by_field_name('path')
                if path_node:
                    dependencies.append(get_node_text(path_node, source_bytes))

            elif node.type == 'struct_specifier':
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = get_node_text(name_node, source_bytes)

                    fields = []
                    body_node = node.child_by_field_name('body')
                    if body_node:
                        for child in body_node.children:
                            if child.type == 'field_declaration':
                                type_node = child.child_by_field_name('type')
                                decl_node = child.child_by_field_name('declarator')
                                if type_node and decl_node:
                                    if decl_node.type == 'identifier' or decl_node.type == 'pointer_declarator' or decl_node.type == 'array_declarator':
                                        fields.append({
                                            'name': get_node_text(decl_node, source_bytes),
                                            'type': get_node_text(type_node, source_bytes)
                                        })

                    classes.append({
                        'name': name,
                        'kind': 'struct',
                        'doc': '',
                        'methods': [], # C structs don't have methods
                        'fields': fields,
                        'implements': []
                    })
            elif node.type == 'function_definition':
                decl_node = node.child_by_field_name('declarator')
                type_node = node.child_by_field_name('type')
                if decl_node and type_node:
                    # Find function_declarator inside declarator
                    func_decl = None
                    if decl_node.type == 'function_declarator':
                        func_decl = decl_node
                    elif decl_node.type == 'pointer_declarator':
                        for d_child in decl_node.children:
                            if d_child.type == 'function_declarator':
                                func_decl = d_child

                    if func_decl:
                        name_node = func_decl.child_by_field_name('declarator')
                        if name_node:
                            name = get_node_text(name_node, source_bytes)
                            args = []
                            params_node = func_decl.child_by_field_name('parameters')
                            if params_node:
                                for p in params_node.children:
                                    if p.type == 'parameter_declaration':
                                        p_type_node = p.child_by_field_name('type')
                                        p_decl_node = p.child_by_field_name('declarator')
                                        if p_type_node and p_decl_node:
                                            args.append({
                                                "name": get_node_text(p_decl_node, source_bytes),
                                                "type": get_node_text(p_type_node, source_bytes)
                                            })

                            free_functions.append({
                                'name': name,
                                'is_pub': True, # C functions are global by default unless static (simplification)
                                'doc': '',
                                'args': args,
                                'ret_type': get_node_text(type_node, source_bytes)
                            })

            for child in node.children:
                traverse(child)

        traverse(tree.root_node)

        for c in classes:
            c['fields'].sort(key=lambda x: x['name'])

        classes.sort(key=lambda x: x['name'])
        free_functions.sort(key=lambda x: x['name'])

        return {
            "classes": classes,
            "free_functions": free_functions,
            "dependencies": sorted(list(set(dependencies)))
        }
    except Exception as e:
        print(f"Error parsing {filepath}: {e}", file=sys.stderr)
        return {"classes": [], "free_functions": [], "dependencies": []}

if __name__ == '__main__':
    if len(sys.argv) > 1:
        print(json.dumps(parse_c(sys.argv[1])))
