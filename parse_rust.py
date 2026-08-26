import tree_sitter_rust as tsrust
from tree_sitter import Language, Parser
import sys
import json

def get_node_text(node, source_bytes):
    return source_bytes[node.start_byte:node.end_byte].decode('utf-8')

def parse_rust(filepath):
    try:
        with open(filepath, 'rb') as f:
            source_bytes = f.read()

        RUST_LANGUAGE = Language(tsrust.language())
        parser = Parser(RUST_LANGUAGE)
        tree = parser.parse(source_bytes)

        classes = []
        free_functions = []
        dependencies = []

        def traverse(node):
            if node.type == 'use_declaration':
                text = get_node_text(node, source_bytes)
                dependencies.append(text.replace('use ', '').replace(';', '').strip())

            elif node.type in ['struct_item', 'enum_item', 'trait_item']:
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = get_node_text(name_node, source_bytes)
                    doc_comments = []
                    prev_sibling = node.prev_sibling
                    while prev_sibling and prev_sibling.type in ['line_comment', 'attribute_item']:
                        if prev_sibling.type == 'line_comment':
                            doc_comments.append(get_node_text(prev_sibling, source_bytes))
                        prev_sibling = prev_sibling.prev_sibling

                    kind = node.type.split('_')[0]

                    fields = []
                    if kind == 'struct':
                        field_decl_list = node.child_by_field_name('body')
                        if field_decl_list and field_decl_list.type == 'field_declaration_list':
                            for f_child in field_decl_list.children:
                                if f_child.type == 'field_declaration':
                                    fname_node = f_child.child_by_field_name('name')
                                    ftype_node = f_child.child_by_field_name('type')
                                    if fname_node and ftype_node:
                                        fields.append({
                                            'name': get_node_text(fname_node, source_bytes),
                                            'type': get_node_text(ftype_node, source_bytes)
                                        })

                    methods = []
                    if kind == 'trait':
                        body = node.child_by_field_name('body')
                        if body and body.type == 'declaration_list':
                            for b_child in body.children:
                                if b_child.type in ['function_item', 'function_signature_item']:
                                    func_name_node = b_child.child_by_field_name('name')
                                    if func_name_node:
                                        func_name = get_node_text(func_name_node, source_bytes)
                                        is_pub = True

                                        func_doc = []
                                        ps = b_child.prev_sibling
                                        while ps and ps.type in ['line_comment', 'attribute_item']:
                                            if ps.type == 'line_comment':
                                                func_doc.append(get_node_text(ps, source_bytes))
                                            ps = ps.prev_sibling

                                        is_constructor = func_name == "new"

                                        args = []
                                        ret_type = "None"
                                        params_node = b_child.child_by_field_name('parameters')
                                        if params_node:
                                            for p in params_node.children:
                                                if p.type == 'parameter':
                                                    pat = p.child_by_field_name('pattern')
                                                    typ = p.child_by_field_name('type')
                                                    pat_text = get_node_text(pat, source_bytes) if pat else "unknown"
                                                    typ_text = get_node_text(typ, source_bytes) if typ else "Any"
                                                    args.append({"name": pat_text, "type": typ_text})
                                        return_type_node = b_child.child_by_field_name('return_type')
                                        if return_type_node:
                                            ret_type = get_node_text(return_type_node, source_bytes)
                                            if ret_type.startswith('->'):
                                                ret_type = ret_type[2:].strip()

                                        methods.append({
                                            'name': func_name,
                                            'is_pub': is_pub,
                                            'is_constructor': is_constructor,
                                            'doc': '\n'.join(reversed(func_doc)),
                                            'args': args,
                                            'ret_type': ret_type
                                        })

                    classes.append({
                        'name': name,
                        'kind': kind,
                        'doc': '\n'.join(reversed(doc_comments)),
                        'methods': methods,
                        'fields': fields,
                        'implements': []
                    })

            elif node.type == 'impl_item':
                type_node = node.child_by_field_name('type')
                trait_node = node.child_by_field_name('trait')
                if type_node:
                    target_name = get_node_text(type_node, source_bytes)
                    target_class = next((c for c in classes if c['name'] == target_name), None)
                    if not target_class:
                        target_class = {
                            'name': target_name,
                            'kind': 'struct',
                            'doc': '',
                            'methods': [],
                            'fields': [],
                            'implements': []
                        }
                        classes.append(target_class)

                    if trait_node:
                        trait_name = get_node_text(trait_node, source_bytes)
                        target_class['implements'].append(trait_name)

                    body = node.child_by_field_name('body')
                    if body:
                        for b_child in body.children:
                            if b_child.type == 'function_item':
                                func_name_node = b_child.child_by_field_name('name')
                                if func_name_node:
                                    func_name = get_node_text(func_name_node, source_bytes)
                                    is_pub = False
                                    for c in b_child.children:
                                        if c.type == 'visibility_modifier':
                                            is_pub = True

                                    func_doc = []
                                    ps = b_child.prev_sibling
                                    while ps and ps.type in ['line_comment', 'attribute_item']:
                                        if ps.type == 'line_comment':
                                            func_doc.append(get_node_text(ps, source_bytes))
                                        ps = ps.prev_sibling

                                    is_constructor = func_name == "new"

                                    args = []
                                    ret_type = "None"
                                    params_node = b_child.child_by_field_name('parameters')
                                    if params_node:
                                        for p in params_node.children:
                                            if p.type == 'parameter':
                                                pat = p.child_by_field_name('pattern')
                                                typ = p.child_by_field_name('type')
                                                pat_text = get_node_text(pat, source_bytes) if pat else "unknown"
                                                typ_text = get_node_text(typ, source_bytes) if typ else "Any"
                                                args.append({"name": pat_text, "type": typ_text})
                                    return_type_node = b_child.child_by_field_name('return_type')
                                    if return_type_node:
                                        ret_type = get_node_text(return_type_node, source_bytes)
                                        if ret_type.startswith('->'):
                                            ret_type = ret_type[2:].strip()


                                    target_class['methods'].append({
                                        'name': func_name,
                                        'is_pub': is_pub,
                                        'is_constructor': is_constructor,
                                        'doc': '\n'.join(reversed(func_doc)),
                                        'args': args,
                                        'ret_type': ret_type
                                    })

            elif node.type == 'function_item':
                if node.parent and node.parent.type == 'source_file':
                    name_node = node.child_by_field_name('name')
                    if name_node:
                        name = get_node_text(name_node, source_bytes)
                        is_pub = False
                        for c in node.children:
                            if c.type == 'visibility_modifier':
                                is_pub = True

                        doc_comments = []
                        ps = node.prev_sibling
                        while ps and ps.type in ['line_comment', 'attribute_item']:
                            if ps.type == 'line_comment':
                                doc_comments.append(get_node_text(ps, source_bytes))
                            ps = ps.prev_sibling

                        args = []
                        ret_type = "None"
                        params_node = node.child_by_field_name('parameters')
                        if params_node:
                            for p in params_node.children:
                                if p.type == 'parameter':
                                    pat = p.child_by_field_name('pattern')
                                    typ = p.child_by_field_name('type')
                                    pat_text = get_node_text(pat, source_bytes) if pat else "unknown"
                                    typ_text = get_node_text(typ, source_bytes) if typ else "Any"
                                    args.append({"name": pat_text, "type": typ_text})
                        return_type_node = node.child_by_field_name('return_type')
                        if return_type_node:
                            ret_type = get_node_text(return_type_node, source_bytes)
                            if ret_type.startswith('->'):
                                ret_type = ret_type[2:].strip()

                        free_functions.append({
                            'name': name,
                            'is_pub': is_pub,
                            'doc': '\n'.join(reversed(doc_comments)),
                            'args': args,
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
        print(json.dumps(parse_rust(sys.argv[1])))
