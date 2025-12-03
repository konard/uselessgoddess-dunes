#!/usr/bin/env python3
"""
Clean invalid (old) benchmark data from gh-pages.

This script removes entries with old formats:
1. Entries with tool="cargo" (ns/iter format)
2. Entries with tool="customBiggerIsBetter" but unit="links/sec" instead of "M links/sec"

Only keeps entries with:
- tool="customBiggerIsBetter"
- unit="M links/sec"
- range format with "±" prefix
"""

import json
import sys


def should_keep_entry(entry):
    """
    Determine if a benchmark entry should be kept.

    Keep only entries with the new format:
    - tool: "customBiggerIsBetter"
    - unit: "M links/sec" (million links per second)
    - range: with "±" prefix
    """
    tool = entry.get('tool', '')

    # Only keep customBiggerIsBetter entries
    if tool != 'customBiggerIsBetter':
        return False

    # Check if all benches have the correct unit format
    benches = entry.get('benches', [])
    if not benches:
        return False

    for bench in benches:
        unit = bench.get('unit', '')
        range_val = str(bench.get('range', ''))

        # Must have "M links/sec" unit and "±" in range
        if unit != 'M links/sec':
            return False
        if '±' not in range_val:
            return False

    return True


def clean_benchmark_data(input_file, output_file):
    """Clean benchmark data file by removing old format entries."""

    # Read the JavaScript file
    with open(input_file, 'r') as f:
        content = f.read()

    # Remove the JavaScript assignment to get pure JSON
    json_str = content.replace('window.BENCHMARK_DATA = ', '').strip()

    # Parse JSON
    data = json.loads(json_str)

    # Get original entries
    original_entries = data.get('entries', {}).get('Benchmark', [])
    original_count = len(original_entries)

    print(f"Original entries: {original_count}")

    # Filter entries
    cleaned_entries = []
    removed_entries = []

    for i, entry in enumerate(original_entries):
        commit_id = entry.get('commit', {}).get('id', 'unknown')[:7]
        tool = entry.get('tool', 'unknown')

        if should_keep_entry(entry):
            cleaned_entries.append(entry)
            print(f"  ✓ Keeping entry {i+1}: {commit_id} (tool={tool})")
        else:
            removed_entries.append(entry)
            bench_count = len(entry.get('benches', []))
            units = set(b.get('unit', '') for b in entry.get('benches', []))
            print(f"  ✗ Removing entry {i+1}: {commit_id} (tool={tool}, units={units}, {bench_count} benches)")

    # Update data
    data['entries']['Benchmark'] = cleaned_entries

    print(f"\nSummary:")
    print(f"  Kept: {len(cleaned_entries)} entries")
    print(f"  Removed: {len(removed_entries)} entries")

    # Write back to file with JavaScript assignment
    output_content = 'window.BENCHMARK_DATA = ' + json.dumps(data, indent=2) + '\n'

    with open(output_file, 'w') as f:
        f.write(output_content)

    print(f"\nCleaned data written to: {output_file}")

    return len(cleaned_entries), len(removed_entries)


def main():
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <input_file> <output_file>")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    kept, removed = clean_benchmark_data(input_file, output_file)

    if removed > 0:
        print(f"\n✓ Successfully cleaned {removed} old format entries")
        sys.exit(0)
    else:
        print(f"\n✓ No old format entries found, data is already clean")
        sys.exit(0)


if __name__ == '__main__':
    main()
