import tree_sitter_c_sharp as tscs
from tree_sitter import Language, Parser
import sys
import json

def get_node_text(node, source_bytes):
    return source_bytes[node.start_byte:node.end_byte].decode('utf-8')

def parse_cs(filepath):
    try:
        with open(filepath, 'rb') as f:
            source_bytes = f.read()

        lang = Language(tscs.language())
        parser = Parser(lang)
        tree = parser.parse(source_bytes)

        classes = []
        dependencies = []

        def traverse(node):
            if node.type == 'using_directive':
                name_node = node.child_by_field_name('name')
                if name_node:
                    dependencies.append(get_node_text(name_node, source_bytes))

            elif node.type in ['class_declaration', 'interface_declaration', 'struct_declaration', 'enum_declaration']:
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = get_node_text(name_node, source_bytes)
                    kind = node.type.split('_')[0]

                    methods = []
                    fields = []
                    implements = []

                    bases_node = node.child_by_field_name('bases')
                    if bases_node:
                        for child in bases_node.children:
                            if child.type == 'base_list':
                                for t in child.children:
                                    if t.type == 'identifier':
                                        implements.append(get_node_text(t, source_bytes))

                    body_node = node.child_by_field_name('body')
                    if body_node:
                        for b_child in body_node.children:
                            if b_child.type in ['method_declaration', 'constructor_declaration']:
                                m_name_node = b_child.child_by_field_name('name')
                                if m_name_node:
                                    m_name = get_node_text(m_name_node, source_bytes)
                                    is_pub = False

                                    # Look for modifiers
                                    for child in b_child.children:
                                         if child.type == 'modifier':
                                             if get_node_text(child, source_bytes) == 'public':
                                                 is_pub = True
                                                 break
                                    if kind == 'interface':
                                        is_pub = True

                                    is_constructor = b_child.type == 'constructor_declaration'

                                    args = []
                                    params = b_child.child_by_field_name('parameters')
                                    if params:
                                        for p in params.children:
                                            if p.type == 'parameter':
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
                                for decl in b_child.children:
                                    if decl.type == 'variable_declarator':
                                        f_name_node = decl.child_by_field_name('name')
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
            "free_functions": [],
            "dependencies": sorted(list(set(dependencies)))
        }
    except Exception as e:
        print(f"Error parsing {filepath}: {e}", file=sys.stderr)
        return {"classes": [], "free_functions": [], "dependencies": []}

if __name__ == '__main__':
    if len(sys.argv) > 1:
        print(json.dumps(parse_cs(sys.argv[1])))
