import tree_sitter_cpp as tscpp
from tree_sitter import Language, Parser
import sys
import json

def parse_cpp(filepath):
    try:
        lang = Language(tscpp.language())
        parser = Parser(lang)
        return {"classes": [], "free_functions": [], "dependencies": []}
    except Exception as e:
        print(f"Error parsing {filepath}: {e}", file=sys.stderr)
        return {"classes": [], "free_functions": [], "dependencies": []}

if __name__ == '__main__':
    if len(sys.argv) > 1:
        print(json.dumps(parse_cpp(sys.argv[1])))
