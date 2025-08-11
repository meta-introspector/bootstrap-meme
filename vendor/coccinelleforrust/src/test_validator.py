#!/usr/bin/env python3

import os
import subprocess
import difflib
import re
from termcolor import colored

# Hardcoded destination path
destination_path = "cfr_conftests"

# Normalize diff text
def normalize_diff(diff_text):
    lines = diff_text.splitlines()
    # Keep only lines starting with '+' or '-' but not '+++' or '---'
    changes = [line for line in lines if re.match(r'^[+-](?![+-])', line)]
    return sorted(changes)  # Sort to ensure order doesn't matter

# Check if diffs are equal
def are_diffs_equal(diff1, diff2):
    norm_diff1 = normalize_diff(diff1)
    norm_diff2 = normalize_diff(diff2)
    return norm_diff1 == norm_diff2

# Function to validate CFR tests by comparing diffs
def validate_cfr_tests():
    if not os.path.exists(destination_path):
        print(colored(f"Error: Destination path '{destination_path}' does not exist.", "red"))
        return

    files = [f for f in os.listdir(destination_path) if f.startswith("test")]

    # Group files by integer suffix
    test_groups = {}
    for file in files:
        try:
            number = int(file[4:].split(".")[0])  # Extract the number after 'test'
            test_groups.setdefault(number, []).append(file)
        except (IndexError, ValueError):
            continue

    # Validate each test group
    for test_number, test_files in test_groups.items():
        output_file = os.path.join(destination_path, f"test{test_number}.txt")
        inputs = [os.path.join(destination_path, f) for f in test_files if not f.endswith(".txt")]

        if not os.path.exists(output_file):
            print(colored(f"Test {test_number}: Missing expected output file {output_file}", "yellow"))
            continue

        if len(inputs) != 2:
            print(colored(f"Test {test_number}: Expected 2 input files, found {len(inputs)}", "yellow"))
            continue

        # Ensure cocci file is first in the inputs
        if inputs[0].endswith("rs"):
            inputs.reverse()

        # Run CFR command
        try:
            result = subprocess.run(["cfr", "-c", *inputs], capture_output=True, text=True, check=True)
            temp_output = result.stdout
        except subprocess.CalledProcessError as e:
            print(colored(f"Test {test_number}: Error running CFR: {e}", "red"))
            continue

        # Compare normalized diffs with expected output
        with open(output_file, "r") as expected_file:
            expected_output = expected_file.read()

        if are_diffs_equal(expected_output, temp_output):
            print(colored(f"Test {test_number}: Passed (Diffs match)", "green"))
        else:
            print(colored(f"Test {test_number}: Failed (Diffs do not match)", "red"))

# Run the validation
validate_cfr_tests()
