#!/usr/bin/env python3

import argparse
import get_critical_path
import os


def parse_args():
    parser = argparse.ArgumentParser(
        description="Automatically generate the verilog file containing the "
                    "propagation delay for the modules."
    )
    parser.add_argument("module_file", help="Path to verilog module file")
    return parser.parse_args()


def get_generated_file_path(module_file):
    path, ext = os.path.splitext(module_file)
    return f"{path}_delay{ext}"


def generate_verilog_file(module_file, module_result):
    generated_file = get_generated_file_path(module_file)
    generated_filename = os.path.split(generated_file)[1]
    include_guard = generated_filename.upper().replace(".", "_")
    file_contents = f"""
`ifndef {include_guard}
`define {include_guard}
`define DELAY {module_result.delay // 10}.{module_result.delay % 10}
`endif
    """
    with open(generated_file, "w") as outfile:
        outfile.write(file_contents)


def main():
    args = parse_args()
    module_result = get_critical_path.get_module_results(args.module_file)
    generate_verilog_file(args.module_file, module_result)


if __name__ == "__main__":
    main()