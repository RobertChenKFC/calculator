#!/bin/bash

# check-line-length.sh
# Written with assistance from Claude (Anthropic)
#
# Checks all Rust source files under src/ for lines exceeding the max_width
# limit defined in rustfmt.toml. Lines that are only over the limit because
# of a URL are exempt. Lines containing the comment "ignore-linelength" are
# skipped entirely. Unicode characters are counted as codepoints (matching
# rustfmt's own counting), not bytes. Exits with code 1 if any violations
# are found, making it suitable for use in a pre-commit hook or CI pipeline.

LIMIT=$(grep '^max_width' rustfmt.toml | awk -F'=' '{print $2}' | tr -d ' ')
LIMIT=${LIMIT:-80}

found=0
while IFS= read -r file; do
  grep -q "ignore-linelength" "$file" && continue
  python3 - "$file" "$LIMIT" <<'EOF'
import sys, re

path, limit = sys.argv[1], int(sys.argv[2])
url_re = re.compile(r'https?://\S+')
found = False

with open(path, encoding='utf-8') as f:
    for lineno, line in enumerate(f, 1):
        line = line.rstrip('\n')
        if len(line) > limit:
            stripped = url_re.sub('', line)
            if len(stripped) <= limit:
                continue
            print(f"{path}:{lineno}: line too long ({len(line)} chars)")
            found = True

sys.exit(1 if found else 0)
EOF
  [ $? -ne 0 ] && found=1
done < <(find src -name "*.rs")

exit $found