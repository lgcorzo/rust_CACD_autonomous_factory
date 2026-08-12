import tree_sitter_java as tsjava
from tree_sitter import Language, Parser
import sys
import json
import os

def get_node_text(node, source_bytes):
    return source_bytes[node.start_byte:node.end_byte].decode('utf-8')

def parse_java(filepath):
    try:
        with open(filepath, 'rb') as f:
            source_bytes = f.read()

        lang = Language(tsjava.language())
        parser = Parser(lang)
        tree = parser.parse(source_bytes)

        classes = []
        free_functions = []
        dependencies = []

        def traverse(node):
            if node.type == 'import_declaration':
                text = get_node_text(node, source_bytes)
                dependencies.append(text.replace('import ', '').replace(';', '').strip())

            elif node.type in ['class_declaration', 'interface_declaration', 'enum_declaration']:
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = get_node_text(name_node, source_bytes)
                    kind = node.type.split('_')[0]

                    methods = []
                    fields = []
                    implements = []

                    interfaces_node = node.child_by_field_name('interfaces')
                    if interfaces_node:
                        for child in interfaces_node.children:
                            if child.type == 'type_list':
                                for t in child.children:
                                    if t.type == 'type_identifier':
                                        implements.append(get_node_text(t, source_bytes))

                    superclass_node = node.child_by_field_name('superclass')
                    if superclass_node:
                         t = superclass_node.child_by_field_name('type')
                         if t:
                              implements.append(get_node_text(t, source_bytes))

                    body_node = node.child_by_field_name('body')
                    if body_node:
                        for b_child in body_node.children:
                            if b_child.type in ['method_declaration', 'constructor_declaration']:
                                m_name_node = b_child.child_by_field_name('name')
                                if m_name_node:
                                    m_name = get_node_text(m_name_node, source_bytes)
                                    is_pub = False

                                    modifiers = b_child.child_by_field_name('modifiers')
                                    if modifiers:
                                        for mod in modifiers.children:
                                            if get_node_text(mod, source_bytes) == 'public':
                                                is_pub = True
                                                break
                                    elif kind == 'interface':
                                        is_pub = True

                                    is_constructor = b_child.type == 'constructor_declaration'

                                    args = []
                                    params = b_child.child_by_field_name('parameters')
                                    if params:
                                        for p in params.children:
                                            if p.type == 'formal_parameter':
                                                p_name_node = p.child_by_field_name('name')
                                                p_type_node = p.child_by_field_name('type')
                                                if p_name_node and p_type_node:
                                                    args.append({
                                                        "name": get_node_text(p_name_node, source_bytes),
                                                        "type": get_node_text(p_type_node, source_bytes)
                                                    })

                                    ret_type = "void"
                                    if not is_constructor:
                                        ret_node = b_child.child_by_field_name('type')
                                        if ret_node:
                                            ret_type = get_node_text(ret_node, source_bytes)

                                    methods.append({
                                        'name': m_name,
                                        'is_pub': is_pub,
                                        'is_constructor': is_constructor,
                                        'doc': '',
                                        'args': args,
                                        'ret_type': ret_type
                                    })
                            elif b_child.type == 'field_declaration':
                                f_type_node = b_child.child_by_field_name('type')
                                f_type = get_node_text(f_type_node, source_bytes) if f_type_node else 'any'
                                decls = b_child.child_by_field_name('declarator')
                                if decls:
                                     # handle only one for simplicity
                                     f_name_node = decls.child_by_field_name('name')
                                     if f_name_node:
                                         fields.append({
                                             'name': get_node_text(f_name_node, source_bytes),
                                             'type': f_type
                                         })

                    classes.append({
                        'name': name,
                        'kind': kind,
                        'doc': '',
                        'methods': methods,
                        'fields': fields,
                        'implements': implements
                    })

            for child in node.children:
                traverse(child)

        traverse(tree.root_node)

        for c in classes:
            c['methods'].sort(key=lambda x: x['name'])
            c['fields'].sort(key=lambda x: x['name'])
            c['implements'].sort()

        classes.sort(key=lambda x: x['name'])

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
        print(json.dumps(parse_java(sys.argv[1])))
