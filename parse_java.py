import tree_sitter_java as tsjava
from tree_sitter import Language, Parser
import sys
import json

def parse_java(filepath):
    try:
        lang = Language(tsjava.language())
        parser = Parser(lang)
        # We don't actually parse anything here for the minimal implementation, just return empty.
        return {"classes": [], "free_functions": [], "dependencies": []}
    except Exception as e:
        print(f"Error parsing {filepath}: {e}", file=sys.stderr)
        return {"classes": [], "free_functions": [], "dependencies": []}

if __name__ == '__main__':
    if len(sys.argv) > 1:
        print(json.dumps(parse_java(sys.argv[1])))
