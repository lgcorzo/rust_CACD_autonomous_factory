import tree_sitter_c_sharp as tscsharp
from tree_sitter import Language, Parser
import sys
import json
import os

def get_node_text(node, source_bytes):
    return source_bytes[node.start_byte:node.end_byte].decode('utf-8')

def parse_csharp(filepath):
    try:
        with open(filepath, 'rb') as f:
            source_bytes = f.read()

        lang = Language(tscsharp.language())
        parser = Parser(lang)
        tree = parser.parse(source_bytes)

        classes = []
        free_functions = []
        dependencies = []

        def traverse(node):
            if node.type == 'using_directive':
                name_node = node.child_by_field_name('name')
                if name_node:
                    dependencies.append(get_node_text(name_node, source_bytes))

            elif node.type == 'class_declaration' or node.type == 'interface_declaration' or node.type == 'struct_declaration':
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = get_node_text(name_node, source_bytes)
                    kind = 'class'
                    if node.type == 'interface_declaration': kind = 'interface'
                    elif node.type == 'struct_declaration': kind = 'struct'

                    methods = []
                    fields = []
                    implements = []

                    bases_node = node.child_by_field_name('bases')
                    if bases_node:
                        for child in bases_node.children:
                            if child.type == 'base_list':
                                for t in child.children:
                                    if t.type == 'identifier' or t.type == 'generic_name':
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
                                    for child_node in child.children:
                                        if child_node.type == 'modifier':
                                            if get_node_text(child_node, source_bytes) == 'public':
                                                is_pub = True

                                    args = []
                                    params_node = child.child_by_field_name('parameters')
                                    if params_node:
                                        for p in params_node.children:
                                            if p.type == 'parameter':
                                                p_name_node = p.child_by_field_name('name')
                                                p_type_node = p.child_by_field_name('type')
                                                if p_name_node:
                                                    args.append({
                                                        "name": get_node_text(p_name_node, source_bytes),
                                                        "type": get_node_text(p_type_node, source_bytes) if p_type_node else "object"
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
                                    is_pub = False

                                    for child_node in child.children:
                                        if child_node.type == 'modifier':
                                            if get_node_text(child_node, source_bytes) == 'public':
                                                is_pub = True

                                    args = []
                                    params_node = child.child_by_field_name('parameters')
                                    if params_node:
                                        for p in params_node.children:
                                            if p.type == 'parameter':
                                                p_name_node = p.child_by_field_name('name')
                                                p_type_node = p.child_by_field_name('type')
                                                if p_name_node:
                                                    args.append({
                                                        "name": get_node_text(p_name_node, source_bytes),
                                                        "type": get_node_text(p_type_node, source_bytes) if p_type_node else "object"
                                                    })
                                    methods.append({
                                        'name': m_name,
                                        'is_pub': is_pub,
                                        'is_constructor': True,
                                        'doc': '',
                                        'args': args,
                                        'ret_type': name
                                    })
                            elif child.type == 'field_declaration' or child.type == 'property_declaration':
                                type_node = child.child_by_field_name('type')
                                name_node = child.child_by_field_name('name')
                                if not name_node and child.type == 'field_declaration':
                                    decl_node = child.child_by_field_name('declaration')
                                    if decl_node:
                                        for d_child in decl_node.children:
                                            if d_child.type == 'variable_declarator':
                                                name_node = d_child.child_by_field_name('name')

                                if type_node and name_node:
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
            "free_functions": [], # C# doesn't usually have free functions (pre C# 9 top-level)
            "dependencies": sorted(list(set(dependencies)))
        }
    except Exception as e:
        print(f"Error parsing {filepath}: {e}", file=sys.stderr)
        return {"classes": [], "free_functions": [], "dependencies": []}

if __name__ == '__main__':
    if len(sys.argv) > 1:
        print(json.dumps(parse_csharp(sys.argv[1])))
