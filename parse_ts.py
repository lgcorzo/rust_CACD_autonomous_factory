import tree_sitter_typescript as tsts
import tree_sitter_javascript as tsjs
from tree_sitter import Language, Parser
import sys
import json
import os

def get_node_text(node, source_bytes):
    return source_bytes[node.start_byte:node.end_byte].decode('utf-8')

def parse_ts(filepath):
    try:
        with open(filepath, 'rb') as f:
            source_bytes = f.read()

        ext = os.path.splitext(filepath)[1].lower()
        if ext == '.js' or ext == '.jsx':
            lang = Language(tsjs.language())
        elif ext == '.tsx':
            lang = Language(tsts.language_tsx())
        else:
            lang = Language(tsts.language_typescript())

        parser = Parser(lang)
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

                    methods = []
                    fields = []

                    body_node = node.child_by_field_name('body')
                    if body_node:
                        for b_child in body_node.children:
                            if b_child.type == 'method_definition' or b_child.type == 'method_signature':
                                m_name_node = b_child.child_by_field_name('name')
                                if m_name_node:
                                    m_name = get_node_text(m_name_node, source_bytes)
                                    is_pub = True
                                    for mod in b_child.children:
                                        if mod.type == 'accessibility_modifier':
                                            if get_node_text(mod, source_bytes) in ['private', 'protected']:
                                                is_pub = False

                                    is_constructor = m_name == "constructor"
                                    if is_constructor:
                                        is_pub = True

                                    args = []
                                    params = b_child.child_by_field_name('parameters')
                                    if params:
                                        for p in params.children:
                                            if p.type in ['required_parameter', 'optional_parameter']:
                                                p_name_node = p.child_by_field_name('pattern') or p
                                                p_name = get_node_text(p_name_node, source_bytes)
                                                args.append({"name": p_name, "type": "any"})

                                    ret_type = "any"
                                    ret_node = b_child.child_by_field_name('return_type')
                                    if ret_node:
                                        ret_type = get_node_text(ret_node, source_bytes)
                                        if ret_type.startswith(':'):
                                            ret_type = ret_type[1:].strip()

                                    methods.append({
                                        'name': m_name,
                                        'is_pub': is_pub,
                                        'is_constructor': is_constructor,
                                        'doc': '',
                                        'args': args,
                                        'ret_type': ret_type
                                    })
                            elif b_child.type == 'property_signature' or b_child.type == 'public_field_definition':
                                f_name_node = b_child.child_by_field_name('name')
                                f_type_node = b_child.child_by_field_name('type')
                                if f_name_node:
                                    f_name = get_node_text(f_name_node, source_bytes)
                                    f_type = "any"
                                    if f_type_node:
                                        f_type = get_node_text(f_type_node, source_bytes)
                                        if f_type.startswith(':'):
                                            f_type = f_type[1:].strip()
                                    fields.append({
                                        'name': f_name,
                                        'type': f_type
                                    })

                    classes.append({
                        'name': name,
                        'kind': kind,
                        'doc': '',
                        'methods': methods,
                        'fields': fields,
                        'implements': []
                    })

            elif node.type == 'function_declaration' or node.type == 'function_signature':
                if node.parent and node.parent.type == 'program':
                    name_node = node.child_by_field_name('name')
                    if name_node:
                        name = get_node_text(name_node, source_bytes)
                        ret_type = "any"
                        ret_node = node.child_by_field_name('return_type')
                        if ret_node:
                            ret_type = get_node_text(ret_node, source_bytes)
                            if ret_type.startswith(':'):
                                ret_type = ret_type[1:].strip()
                        free_functions.append({
                            'name': name,
                            'is_pub': True,
                            'doc': '',
                            'args': [],
                            'ret_type': ret_type
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
        print(json.dumps(parse_ts(sys.argv[1])))
