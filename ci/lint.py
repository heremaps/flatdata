#!/usr/bin/env python3
import sys
from pylint.lint import Run

THRESHOLD = 6.00

results = Run(['flatdata-py'], do_exit=False)
score = results.linter.stats['global_note']

if score < THRESHOLD:
    print("pylint score below acceptable threshold of %.2f/10!" % THRESHOLD, file=sys.stderr)
    sys.exit(1)