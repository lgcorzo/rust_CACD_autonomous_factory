import tree_sitter_cpp as tscpp
import tree_sitter_c as tsc
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

        if filepath.endswith('.c') or filepath.endswith('.h'):
             lang = Language(tsc.language())
        else:
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
                    dependencies.append(get_node_text(path_node, source_bytes).strip('<>"'))

            elif node.type in ['class_specifier', 'struct_specifier']:
                name_node = node.child_by_field_name('name')
                if name_node:
                    name = get_node_text(name_node, source_bytes)
                    kind = node.type.split('_')[0]

                    classes.append({
                        'name': name,
                        'kind': kind,
                        'doc': '',
                        'methods': [],
                        'fields': [],
                        'implements': []
                    })

            elif node.type == 'function_definition':
                name = None
                decl = node.child_by_field_name('declarator')

                # Dig to find function_declarator
                def find_func_decl(n):
                    if n.type == 'function_declarator':
                        return n
                    for c in n.children:
                        res = find_func_decl(c)
                        if res: return res
                    return None

                func_decl = find_func_decl(decl) if decl else None

                if func_decl:
                    name_node = func_decl.child_by_field_name('declarator')
                    if name_node:
                        name = get_node_text(name_node, source_bytes)
                        # Remove class scope if exists for free functions
                        if '::' not in name:
                            args = []
                            params = func_decl.child_by_field_name('parameters')
                            if params:
                                for p in params.children:
                                    if p.type == 'parameter_declaration':
                                        p_name_node = p.child_by_field_name('declarator')
                                        p_type_node = p.child_by_field_name('type')
                                        p_name = get_node_text(p_name_node, source_bytes) if p_name_node else ""
                                        p_type = get_node_text(p_type_node, source_bytes) if p_type_node else ""
                                        args.append({"name": p_name, "type": p_type})

                            ret_type = "void"
                            type_node = node.child_by_field_name('type')
                            if type_node:
                                ret_type = get_node_text(type_node, source_bytes)

                            free_functions.append({
                                'name': name,
                                'is_pub': True,
                                'doc': '',
                                'args': args,
                                'ret_type': ret_type
                            })

                # Note: Methods inside classes are harder to parse correctly without a full C++ semantic pass,
                # but we'll try to find inline method definitions in class_specifier


            for child in node.children:
                traverse(child)

        traverse(tree.root_node)

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
        print(json.dumps(parse_cpp(sys.argv[1])))
