import re
import sys
import json

def parse_rust(filepath):
    classes = []
    free_functions = []
    dependencies = []

    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        current_class = None
        current_impl = None

        for i, line in enumerate(lines):
            line_num = i + 1

            # Structs, Enums, Traits
            m_struct = re.match(r'^\s*(?:pub\s+)?struct\s+(\w+)', line)
            if m_struct:
                classes.append({'name': m_struct.group(1), 'kind': 'struct', 'line': line_num, 'methods': [], 'implements': []})
                current_class = m_struct.group(1)

            m_enum = re.match(r'^\s*(?:pub\s+)?enum\s+(\w+)', line)
            if m_enum:
                classes.append({'name': m_enum.group(1), 'kind': 'enum', 'line': line_num, 'methods': [], 'implements': []})
                current_class = m_enum.group(1)

            m_trait = re.match(r'^\s*(?:pub\s+)?(?:async\s+)?trait\s+(\w+)', line)
            if m_trait:
                classes.append({'name': m_trait.group(1), 'kind': 'trait', 'line': line_num, 'methods': [], 'implements': []})
                current_class = m_trait.group(1)

            # Methods in traits
            if current_class and next((c for c in classes if c['name'] == current_class and c['kind'] == 'trait'), None):
                m_fn = re.match(r'^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)', line)
                if m_fn:
                    for c in classes:
                        if c['name'] == current_class:
                            c['methods'].append({'name': m_fn.group(1), 'line': line_num})
                            break

            # Impls
            m_impl = re.match(r'^\s*impl(?:\s*<.*?>)?\s+(?:(\w+)\s+for\s+)?(\w+)', line)
            if m_impl:
                trait_name = m_impl.group(1)
                target_name = m_impl.group(2)

                # Check if target_name exists in classes
                found = False
                for c in classes:
                    if c['name'] == target_name:
                        found = True
                        if trait_name:
                            c['implements'].append(trait_name)
                        break

                if not found:
                    classes.append({'name': target_name, 'kind': 'struct', 'line': line_num, 'methods': [], 'implements': [trait_name] if trait_name else []})

                current_impl = target_name

            # Methods in impls
            if current_impl:
                m_fn = re.match(r'^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)', line)
                if m_fn:
                    for c in classes:
                        if c['name'] == current_impl:
                            c['methods'].append({'name': m_fn.group(1), 'line': line_num})
                            break

            # Reset current blocks blindly on closing brace
            # This is flawed but better than line 0
            if re.match(r'^}', line.strip()):
                current_impl = None
                current_class = None

            # Free functions
            if not current_impl and not (current_class and next((c for c in classes if c['name'] == current_class and c['kind'] == 'trait'), None)):
                m_fn = re.match(r'^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)', line)
                if m_fn:
                    free_functions.append({'name': m_fn.group(1), 'line': line_num})

            # Use
            m_use = re.match(r'^\s*(?:pub\s+)?use\s+([\w\:]+)', line)
            if m_use:
                dependencies.append(m_use.group(1).split('::')[0])

    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)

    print(json.dumps({
        "classes": classes,
        "free_functions": free_functions,
        "dependencies": list(set(dependencies))
    }))

if __name__ == '__main__':
    if len(sys.argv) < 2:
        sys.exit(1)
    parse_rust(sys.argv[1])
