import tree_sitter_go as tsgo
from tree_sitter import Language, Parser
import sys
import json

def get_node_text(node, source_bytes):
    return source_bytes[node.start_byte:node.end_byte].decode('utf-8')

def parse_go(filepath):
    try:
        with open(filepath, 'rb') as f:
            source_bytes = f.read()

        lang = Language(tsgo.language())
        parser = Parser(lang)
        tree = parser.parse(source_bytes)

        classes = []
        free_functions = []
        dependencies = []

        def traverse(node):
            if node.type == 'import_spec':
                path_node = node.child_by_field_name('path')
                if path_node:
                    dependencies.append(get_node_text(path_node, source_bytes).strip('"'))

            elif node.type == 'type_declaration':
                for type_spec in node.children:
                    if type_spec.type == 'type_spec':
                        name_node = type_spec.child_by_field_name('name')
                        type_node = type_spec.child_by_field_name('type')
                        if name_node and type_node:
                            name = get_node_text(name_node, source_bytes)

                            if type_node.type == 'struct_type':
                                kind = 'struct'
                            elif type_node.type == 'interface_type':
                                kind = 'interface'
                            else:
                                kind = 'type'

                            classes.append({
                                'name': name,
                                'kind': kind,
                                'doc': '',
                                'methods': [],
                                'fields': [],
                                'implements': []
                            })

            elif node.type == 'method_declaration':
                name_node = node.child_by_field_name('name')
                receiver_node = node.child_by_field_name('receiver')
                if name_node and receiver_node:
                    m_name = get_node_text(name_node, source_bytes)
                    is_pub = m_name[0].isupper() if m_name else False

                    target_class_name = None
                    for child in receiver_node.children:
                         if child.type == 'parameter_list':
                             for p in child.children:
                                 if p.type == 'parameter_declaration':
                                     t = p.child_by_field_name('type')
                                     if t:
                                         if t.type == 'pointer_type':
                                             target_class_name = get_node_text(t.children[1], source_bytes)
                                         else:
                                             target_class_name = get_node_text(t, source_bytes)
                                         break

                    args = []
                    params = node.child_by_field_name('parameters')
                    if params:
                        for p in params.children:
                            if p.type == 'parameter_declaration':
                                p_name = get_node_text(p.child_by_field_name('name'), source_bytes) if p.child_by_field_name('name') else ""
                                p_type = get_node_text(p.child_by_field_name('type'), source_bytes) if p.child_by_field_name('type') else ""
                                if p_name or p_type:
                                    args.append({"name": p_name, "type": p_type})

                    ret_type = ""
                    res_node = node.child_by_field_name('result')
                    if res_node:
                        ret_type = get_node_text(res_node, source_bytes)

                    for c in classes:
                        if c['name'] == target_class_name:
                            c['methods'].append({
                                'name': m_name,
                                'is_pub': is_pub,
                                'is_constructor': False,
                                'doc': '',
                                'args': args,
                                'ret_type': ret_type
                            })
                            break

            elif node.type == 'function_declaration':
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = get_node_text(name_node, source_bytes)
                    is_pub = name[0].isupper() if name else False

                    args = []
                    params = node.child_by_field_name('parameters')
                    if params:
                        for p in params.children:
                            if p.type == 'parameter_declaration':
                                p_name = get_node_text(p.child_by_field_name('name'), source_bytes) if p.child_by_field_name('name') else ""
                                p_type = get_node_text(p.child_by_field_name('type'), source_bytes) if p.child_by_field_name('type') else ""
                                if p_name or p_type:
                                    args.append({"name": p_name, "type": p_type})

                    ret_type = ""
                    res_node = node.child_by_field_name('result')
                    if res_node:
                        ret_type = get_node_text(res_node, source_bytes)

                    free_functions.append({
                        'name': name,
                        'is_pub': is_pub,
                        'doc': '',
                        'args': args,
                        'ret_type': ret_type
                    })

            for child in node.children:
                traverse(child)

        traverse(tree.root_node)

        for c in classes:
            c['methods'].sort(key=lambda x: x['name'])
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
