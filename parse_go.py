import tree_sitter_go as tsgo
from tree_sitter import Language, Parser
import sys
import json
import os

def get_node_text(node, source_bytes):
    return source_bytes[node.start_byte:node.end_byte].decode('utf-8')

def parse_go(filepath):
    try:
        with open(filepath, 'rb') as f:
            source_bytes = f.read()

        lang = Language(tsgo.language())
        parser = Parser(lang)
        tree = parser.parse(source_bytes)

        classes = [] # structs and interfaces in Go
        free_functions = []
        dependencies = []

        def traverse(node):
            if node.type == 'import_spec':
                path_node = node.child_by_field_name('path')
                if path_node:
                    dependencies.append(get_node_text(path_node, source_bytes).strip('"'))

            elif node.type == 'type_spec':
                name_node = node.child_by_field_name('name')
                type_node = node.child_by_field_name('type')

                if name_node and type_node:
                    name = get_node_text(name_node, source_bytes)
                    kind = 'struct'
                    if type_node.type == 'interface_type': kind = 'interface'
                    elif type_node.type == 'struct_type': kind = 'struct'
                    else: return # only document structs and interfaces as classes

                    fields = []
                    methods = [] # Interfaces have methods defined inline

                    if kind == 'struct':
                        field_decl_list = type_node.child_by_field_name('field_declaration_list')
                        if field_decl_list:
                            for f_child in field_decl_list.children:
                                if f_child.type == 'field_declaration':
                                    f_type_node = f_child.child_by_field_name('type')
                                    f_name_node = None

                                    # Handle named fields and embedded fields
                                    for sub in f_child.children:
                                        if sub.type == 'field_identifier':
                                            f_name_node = sub
                                            break

                                    if f_type_node:
                                        fields.append({
                                            'name': get_node_text(f_name_node, source_bytes) if f_name_node else get_node_text(f_type_node, source_bytes),
                                            'type': get_node_text(f_type_node, source_bytes)
                                        })
                    elif kind == 'interface':
                        method_decl_list = type_node
                        for child in method_decl_list.children:
                            if child.type == 'method_spec':
                                m_name_node = child.child_by_field_name('name')
                                m_params_node = child.child_by_field_name('parameters')
                                m_result_node = child.child_by_field_name('result')

                                if m_name_node:
                                    m_name = get_node_text(m_name_node, source_bytes)
                                    args = []
                                    if m_params_node:
                                        for p in m_params_node.children:
                                            if p.type == 'parameter_declaration':
                                                p_type = p.child_by_field_name('type')
                                                p_name = None
                                                for sub in p.children:
                                                    if sub.type == 'identifier':
                                                        p_name = sub
                                                        break
                                                if p_type:
                                                    args.append({
                                                        "name": get_node_text(p_name, source_bytes) if p_name else "",
                                                        "type": get_node_text(p_type, source_bytes)
                                                    })

                                    methods.append({
                                        'name': m_name,
                                        'is_pub': m_name[0].isupper(),
                                        'is_constructor': False,
                                        'doc': '',
                                        'args': args,
                                        'ret_type': get_node_text(m_result_node, source_bytes) if m_result_node else "void"
                                    })

                    classes.append({
                        'name': name,
                        'kind': kind,
                        'doc': '',
                        'methods': methods,
                        'fields': fields,
                        'implements': [] # Go interfaces are implicit, hard to determine statically without full type check
                    })

            elif node.type == 'method_declaration':
                receiver_node = node.child_by_field_name('receiver')
                name_node = node.child_by_field_name('name')
                params_node = node.child_by_field_name('parameters')
                result_node = node.child_by_field_name('result')

                if receiver_node and name_node:
                    name = get_node_text(name_node, source_bytes)

                    # Find the target struct name
                    target_struct_name = None
                    for child in receiver_node.children:
                        if child.type == 'parameter_list':
                            for p in child.children:
                                if p.type == 'parameter_declaration':
                                    t_node = p.child_by_field_name('type')
                                    if t_node:
                                        if t_node.type == 'pointer_type':
                                            for ptr_child in t_node.children:
                                                if ptr_child.type == 'type_identifier':
                                                    target_struct_name = get_node_text(ptr_child, source_bytes)
                                        elif t_node.type == 'type_identifier':
                                            target_struct_name = get_node_text(t_node, source_bytes)

                    if target_struct_name:
                        target_class = next((c for c in classes if c['name'] == target_struct_name), None)
                        if not target_class:
                            target_class = {
                                'name': target_struct_name,
                                'kind': 'struct',
                                'doc': '',
                                'methods': [],
                                'fields': [],
                                'implements': []
                            }
                            classes.append(target_class)

                        args = []
                        if params_node:
                            for p in params_node.children:
                                if p.type == 'parameter_declaration':
                                    p_type = p.child_by_field_name('type')
                                    p_name = None
                                    for sub in p.children:
                                        if sub.type == 'identifier':
                                            p_name = sub
                                            break
                                    if p_type:
                                        args.append({
                                            "name": get_node_text(p_name, source_bytes) if p_name else "",
                                            "type": get_node_text(p_type, source_bytes)
                                        })

                        target_class['methods'].append({
                            'name': name,
                            'is_pub': name[0].isupper(),
                            'is_constructor': False, # Go doesn't have true constructors
                            'doc': '',
                            'args': args,
                            'ret_type': get_node_text(result_node, source_bytes) if result_node else "void"
                        })

            elif node.type == 'function_declaration':
                name_node = node.child_by_field_name('name')
                params_node = node.child_by_field_name('parameters')
                result_node = node.child_by_field_name('result')

                if name_node:
                    name = get_node_text(name_node, source_bytes)
                    args = []
                    if params_node:
                        for p in params_node.children:
                            if p.type == 'parameter_declaration':
                                p_type = p.child_by_field_name('type')
                                p_name = None
                                for sub in p.children:
                                    if sub.type == 'identifier':
                                        p_name = sub
                                        break
                                if p_type:
                                    args.append({
                                        "name": get_node_text(p_name, source_bytes) if p_name else "",
                                        "type": get_node_text(p_type, source_bytes)
                                    })

                    free_functions.append({
                        'name': name,
                        'is_pub': name[0].isupper(),
                        'doc': '',
                        'args': args,
                        'ret_type': get_node_text(result_node, source_bytes) if result_node else "void"
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
        print(json.dumps(parse_go(sys.argv[1])))
