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
                for child in node.children:
                    if child.type == 'scoped_identifier' or child.type == 'identifier':
                        dependencies.append(get_node_text(child, source_bytes))

            elif node.type == 'class_declaration' or node.type == 'interface_declaration':
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = get_node_text(name_node, source_bytes)
                    kind = 'class' if node.type == 'class_declaration' else 'interface'

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

                    body_node = node.child_by_field_name('body')
                    if body_node:
                        for child in body_node.children:
                            if child.type == 'method_declaration':
                                m_name_node = child.child_by_field_name('name')
                                if m_name_node:
                                    m_name = get_node_text(m_name_node, source_bytes)
                                    is_pub = False

                                    # Very basic modifier check
                                    modifiers_node = child.child_by_field_name('modifiers')
                                    if modifiers_node:
                                        for mod in modifiers_node.children:
                                            if get_node_text(mod, source_bytes) == 'public':
                                                is_pub = True

                                    args = []
                                    params_node = child.child_by_field_name('parameters')
                                    if params_node:
                                        for p in params_node.children:
                                            if p.type == 'formal_parameter':
                                                p_name_node = p.child_by_field_name('name')
                                                p_type_node = p.child_by_field_name('type')
                                                if p_name_node:
                                                    args.append({
                                                        "name": get_node_text(p_name_node, source_bytes),
                                                        "type": get_node_text(p_type_node, source_bytes) if p_type_node else "Object"
                                                    })

                                    ret_type_node = child.child_by_field_name('type')
                                    ret_type = get_node_text(ret_type_node, source_bytes) if ret_type_node else "void"

                                    methods.append({
                                        'name': m_name,
                                        'is_pub': is_pub,
                                        'is_constructor': False,
                                        'doc': '',
                                        'args': args,
                                        'ret_type': ret_type
                                    })
                            elif child.type == 'constructor_declaration':
                                m_name_node = child.child_by_field_name('name')
                                if m_name_node:
                                    m_name = get_node_text(m_name_node, source_bytes)
                                    args = []
                                    params_node = child.child_by_field_name('parameters')
                                    if params_node:
                                        for p in params_node.children:
                                            if p.type == 'formal_parameter':
                                                p_name_node = p.child_by_field_name('name')
                                                p_type_node = p.child_by_field_name('type')
                                                if p_name_node:
                                                    args.append({
                                                        "name": get_node_text(p_name_node, source_bytes),
                                                        "type": get_node_text(p_type_node, source_bytes) if p_type_node else "Object"
                                                    })
                                    methods.append({
                                        'name': m_name,
                                        'is_pub': True,
                                        'is_constructor': True,
                                        'doc': '',
                                        'args': args,
                                        'ret_type': name
                                    })
                            elif child.type == 'field_declaration':
                                type_node = child.child_by_field_name('type')
                                decl_node = child.child_by_field_name('declarator')
                                if decl_node and type_node:
                                    # Handle variable_declarator
                                    if decl_node.type == 'variable_declarator':
                                        name_node = decl_node.child_by_field_name('name')
                                        if name_node:
                                            fields.append({
                                                'name': get_node_text(name_node, source_bytes),
                                                'type': get_node_text(type_node, source_bytes)
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
            "free_functions": [], # Java doesn't have true free functions
            "dependencies": sorted(list(set(dependencies)))
        }
    except Exception as e:
        print(f"Error parsing {filepath}: {e}", file=sys.stderr)
        return {"classes": [], "free_functions": [], "dependencies": []}

if __name__ == '__main__':
    if len(sys.argv) > 1:
        print(json.dumps(parse_java(sys.argv[1])))
