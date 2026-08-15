import tree_sitter_cpp as tscpp
from tree_sitter import Language, Parser
import sys
import json
import os

def get_node_text(node, source_bytes):
    return source_bytes[node.start_byte:node.end_byte].decode('utf-8')

def parse_cpp(filepath):
    try:
        with open(filepath, 'rb') as f:
            source_bytes = f.read()

        lang = Language(tscpp.language())
        parser = Parser(lang)
        tree = parser.parse(source_bytes)

        classes = []
        free_functions = []
        dependencies = []

        def traverse(node):
            if node.type == 'preproc_include':
                path_node = node.child_by_field_name('path')
                if path_node:
                    dependencies.append(get_node_text(path_node, source_bytes))

            elif node.type == 'class_specifier' or node.type == 'struct_specifier':
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = get_node_text(name_node, source_bytes)
                    kind = 'class' if node.type == 'class_specifier' else 'struct'

                    methods = []
                    fields = []
                    implements = []

                    # Very basic base class detection (tree-sitter-cpp handles this in base_class_clause)
                    for child in node.children:
                        if child.type == 'base_class_clause':
                            for b_child in child.children:
                                if b_child.type == 'type_identifier':
                                    implements.append(get_node_text(b_child, source_bytes))

                    body_node = node.child_by_field_name('body')
                    if body_node:
                        current_visibility = 'private' if kind == 'class' else 'public'
                        for child in body_node.children:
                            if child.type == 'access_specifier':
                                current_visibility = get_node_text(child, source_bytes).replace(':', '').strip()

                            elif child.type == 'field_declaration':
                                type_node = child.child_by_field_name('type')
                                decl_node = child.child_by_field_name('declarator')
                                if type_node and decl_node:
                                    if decl_node.type == 'field_identifier' or decl_node.type == 'identifier' or decl_node.type == 'pointer_declarator' or decl_node.type == 'array_declarator':
                                        fields.append({
                                            'name': get_node_text(decl_node, source_bytes),
                                            'type': get_node_text(type_node, source_bytes)
                                        })
                            elif child.type == 'function_definition' or child.type == 'declaration':
                                type_node = child.child_by_field_name('type')
                                decl_node = child.child_by_field_name('declarator')
                                if decl_node:
                                    func_decl = None
                                    if decl_node.type == 'function_declarator':
                                        func_decl = decl_node
                                    elif decl_node.type == 'pointer_declarator' or decl_node.type == 'reference_declarator':
                                        for d_child in decl_node.children:
                                            if d_child.type == 'function_declarator':
                                                func_decl = d_child

                                    if func_decl:
                                        func_name_node = func_decl.child_by_field_name('declarator')
                                        if func_name_node:
                                            func_name = get_node_text(func_name_node, source_bytes)
                                            is_constructor = (func_name == name)

                                            args = []
                                            params_node = func_decl.child_by_field_name('parameters')
                                            if params_node:
                                                for p in params_node.children:
                                                    if p.type == 'parameter_declaration' or p.type == 'optional_parameter_declaration':
                                                        p_type_node = p.child_by_field_name('type')
                                                        p_decl_node = p.child_by_field_name('declarator')
                                                        if p_type_node and p_decl_node:
                                                            args.append({
                                                                "name": get_node_text(p_decl_node, source_bytes),
                                                                "type": get_node_text(p_type_node, source_bytes)
                                                            })

                                            ret_type = get_node_text(type_node, source_bytes) if type_node else ("void" if not is_constructor else name)

                                            methods.append({
                                                'name': func_name,
                                                'is_pub': current_visibility == 'public',
                                                'is_constructor': is_constructor,
                                                'doc': '',
                                                'args': args,
                                                'ret_type': ret_type
                                            })

                    classes.append({
                        'name': name,
                        'kind': kind,
                        'doc': '',
                        'methods': methods,
                        'fields': fields,
                        'implements': implements
                    })

            elif node.type == 'function_definition':
                if node.parent and node.parent.type == 'translation_unit':
                    decl_node = node.child_by_field_name('declarator')
                    type_node = node.child_by_field_name('type')
                    if decl_node and type_node:
                        func_decl = None
                        if decl_node.type == 'function_declarator':
                            func_decl = decl_node
                        elif decl_node.type == 'pointer_declarator' or decl_node.type == 'reference_declarator':
                            for d_child in decl_node.children:
                                if d_child.type == 'function_declarator':
                                    func_decl = d_child

                        if func_decl:
                            name_node = func_decl.child_by_field_name('declarator')
                            if name_node and name_node.type == 'identifier': # Ignore Method Definitions outside class body
                                name = get_node_text(name_node, source_bytes)
                                args = []
                                params_node = func_decl.child_by_field_name('parameters')
                                if params_node:
                                    for p in params_node.children:
                                        if p.type == 'parameter_declaration' or p.type == 'optional_parameter_declaration':
                                            p_type_node = p.child_by_field_name('type')
                                            p_decl_node = p.child_by_field_name('declarator')
                                            if p_type_node and p_decl_node:
                                                args.append({
                                                    "name": get_node_text(p_decl_node, source_bytes),
                                                    "type": get_node_text(p_type_node, source_bytes)
                                                })

                                free_functions.append({
                                    'name': name,
                                    'is_pub': True,
                                    'doc': '',
                                    'args': args,
                                    'ret_type': get_node_text(type_node, source_bytes)
                                })
            for child in node.children:
                traverse(child)

        traverse(tree.root_node)

        for c in classes:
            c['methods'].sort(key=lambda x: x['name'])
            c['fields'].sort(key=lambda x: x['name'])
            c['implements'].sort()

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
        print(json.dumps(parse_cpp(sys.argv[1])))
