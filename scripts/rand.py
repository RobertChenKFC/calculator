#!/usr/bin/env python3

import random
import argparse

parser = argparse.ArgumentParser(
    description="Generate a random decimal number of specified length",
)
parser.add_argument("-l", "--length", default=16, type=int)
args = parser.parse_args()

while True:
    l = [random.randint(0, 9) for _ in range(args.length)]
    if l[0] == 0:
        continue
    idx = random.randint(0, args.length - 1)
    l = l[:idx] + ["."] + l[idx:]
    print("".join(str(x) for x in l))
    break