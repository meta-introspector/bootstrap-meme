#!/usr/bin/env python3

import os
import shutil
import sys
import subprocess

# Hardcoded destination path
destination_path = "./cfr_conftests"

# Function to get the next available integer for filenames
def get_next_integer(base_path):
    existing_files = [f for f in os.listdir(base_path) if f.startswith("test")]
    numbers = []
    for f in existing_files:
        try:
            number = int(f[4:].split(".")[0])  # Extract the number after 'test'
            numbers.append(number)
        except (IndexError, ValueError):
            continue
    return max(numbers, default=0) + 1

# Main function to copy files and run CFR
def copy_files_safely(file1, file2):
    # Ensure the destination path exists
    if not os.path.exists(destination_path):
        os.makedirs(destination_path)

    # Check if files exist
    for file in [file1, file2]:
        if not os.path.exists(file):
            print(f"Error: File '{file}' does not exist.")
            return

    # Extract file extensions
    _, ext1 = os.path.splitext(file1)
    _, ext2 = os.path.splitext(file2)
    ext1 = ext1.lstrip('.')
    ext2 = ext2.lstrip('.')

    # Get the next integer for filenames
    next_int = get_next_integer(destination_path)

    # Define new filenames
    new_file1_path = os.path.join(destination_path, f"test{next_int}.{ext1}")
    new_file2_path = os.path.join(destination_path, f"test{next_int}.{ext2}")
    output_file_path = os.path.join(destination_path, f"test{next_int}.txt")

    # Copy the files
    shutil.copy(file1, new_file1_path)
    shutil.copy(file2, new_file2_path)

    print(f"Copied '{file1}' to '{new_file1_path}'")
    print(f"Copied '{file2}' to '{new_file2_path}'")

    # Run CFR and store output
    try:
        with open(output_file_path, "w") as output_file:
            subprocess.run(["cfr", "-c", file1, file2], stdout=output_file, check=True)
        print(f"CFR output saved to '{output_file_path}'")
    except subprocess.CalledProcessError as e:
        print(f"Error running CFR: {e}")

# Check for correct usage
if len(sys.argv) != 3:
    print("Usage: ./safe_copy.py <file1> <file2>")
    sys.exit(1)

# Input file names from command-line arguments
file1 = sys.argv[1]
file2 = sys.argv[2]

# Call the function
copy_files_safely(file1, file2)
