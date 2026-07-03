import json
import sys

for line in sys.stdin:
    print("RAW:", line)
    try:
        obj = json.loads(line)
        print("Parsed:", obj)
    except Exception as e:
        print("JSON error:", e)