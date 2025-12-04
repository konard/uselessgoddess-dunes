#!/usr/bin/env python3
"""
Convert cargo bench output to custom JSON format with million links/second metric.

For tree benchmarks, we want to show throughput (M links/sec) rather than
time (ns/iter), as it's more meaningful and readable for understanding performance.
Values like "115.08 M links/sec" are much easier to read than "115076784 links/sec".
"""

import json
import re
import sys


def parse_bench_output(output_file):
    """Parse cargo bench output and extract benchmark results."""
    with open(output_file, 'r') as f:
        content = f.read()

    # Pattern to match old cargo bench format:
    # test sbt_insert_100 ... bench:       2,715.35 ns/iter (+/- 958.21)
    old_pattern = r'test\s+(\w+)\s+\.\.\.\s+bench:\s+([\d,]+(?:\.\d+)?)\s+ns/iter\s+\(\+/-\s+([\d,]+(?:\.\d+)?)\)'

    # Pattern to match criterion format:
    # sbt_insert_100          time:   [2.7153 µs 2.7634 µs 2.8164 µs]
    criterion_pattern = r'(\w+)\s+time:\s+\[([0-9.]+)\s+([µnm]?s)\s+([0-9.]+)\s+([µnm]?s)\s+([0-9.]+)\s+([µnm]?s)\]'

    results = []

    # Try criterion format first
    for match in re.finditer(criterion_pattern, content):
        bench_name = match.group(1)
        time_low = float(match.group(2))
        unit_low = match.group(3)
        time_avg = float(match.group(4))
        unit_avg = match.group(5)
        time_high = float(match.group(6))
        unit_high = match.group(7)

        # Convert to nanoseconds
        def to_ns(value, unit):
            if unit == 'ns':
                return value
            elif unit == 'µs' or unit == 'us':
                return value * 1000
            elif unit == 'ms':
                return value * 1_000_000
            elif unit == 's':
                return value * 1_000_000_000
            return value

        time_ns = to_ns(time_avg, unit_avg)
        time_ns_low = to_ns(time_low, unit_low)
        time_ns_high = to_ns(time_high, unit_high)
        variance = (time_ns_high - time_ns_low) / 2

        # Extract the number of operations from benchmark name
        # sbt_insert_100 -> 100 links
        # sbt_insert_1000 -> 1000 links
        # sbt_insert_10000 -> 10000 links
        # sbt_insert_search_100 -> 100 links (insert) + 100 links (search) = 200 operations
        # sbt_full_cycle_100 -> 100 links (insert) + 100 links (remove) = 200 operations

        num_match = re.search(r'_(\d+)$', bench_name)
        if not num_match:
            continue

        n = int(num_match.group(1))

        # Determine the number of operations based on benchmark type
        if 'insert_search' in bench_name:
            # Insert n elements, then search n elements
            num_operations = n + n
        elif 'full_cycle' in bench_name:
            # Insert n elements, then remove n elements
            num_operations = n + n
        else:
            # Just insert n elements
            num_operations = n

        # Calculate links/second (throughput)
        # time_ns is time for num_operations operations
        # links/second = num_operations / (time_ns / 1e9)
        time_seconds = time_ns / 1e9
        links_per_second = num_operations / time_seconds

        # Convert to million links per second for better readability
        million_links_per_second = links_per_second / 1_000_000

        # Calculate variance in links/second
        # If time varies by ± variance_ns, then throughput varies inversely
        time_low = (time_ns - variance) / 1e9
        time_high = (time_ns + variance) / 1e9
        links_per_second_high = num_operations / time_low if time_low > 0 else links_per_second
        links_per_second_low = num_operations / time_high if time_high > 0 else links_per_second

        # Use the average of the differences as the range (in millions)
        range_value = (links_per_second_high - links_per_second_low) / 2 / 1_000_000

        # Create a more descriptive name for the chart
        # Group benchmarks by operation type
        if 'insert_search' in bench_name:
            bench_type = 'Insert + Search'
        elif 'full_cycle' in bench_name:
            bench_type = 'Insert + Remove'
        elif 'insert' in bench_name:
            bench_type = 'Insert Only'
        else:
            bench_type = 'Unknown'

        chart_name = f'{bench_type} ({n} elements)'

        # Extract tree type from benchmark name (sbt_ or art_)
        tree_type = 'unknown'
        if bench_name.startswith('sbt_'):
            tree_type = 'SBT'
        elif bench_name.startswith('art_'):
            tree_type = 'ART'

        results.append({
            'name': chart_name,
            'unit': 'M links/sec',
            'value': round(million_links_per_second, 2),
            'range': f'± {round(range_value, 2)}',
            'extra': f'tree={tree_type} ops={num_operations} time={time_ns:.2f}ns'
        })

    # If criterion format didn't match, try old format
    if not results:
        for match in re.finditer(old_pattern, content):
            bench_name = match.group(1)
            time_ns = float(match.group(2).replace(',', ''))
            variance = float(match.group(3).replace(',', ''))

            # Extract the number of operations from benchmark name
            num_match = re.search(r'_(\d+)$', bench_name)
            if not num_match:
                continue

            n = int(num_match.group(1))

            # Determine the number of operations based on benchmark type
            if 'insert_search' in bench_name:
                num_operations = n + n
            elif 'full_cycle' in bench_name:
                num_operations = n + n
            else:
                num_operations = n

            # Calculate links/second (throughput)
            time_seconds = time_ns / 1e9
            links_per_second = num_operations / time_seconds

            # Calculate variance in links/second
            time_low = (time_ns - variance) / 1e9
            time_high = (time_ns + variance) / 1e9
            links_per_second_high = num_operations / time_low if time_low > 0 else links_per_second
            links_per_second_low = num_operations / time_high if time_high > 0 else links_per_second
            range_value = (links_per_second_high - links_per_second_low) / 2

            # Create a more descriptive name for the chart
            if 'insert_search' in bench_name:
                bench_type = 'Insert + Search'
            elif 'full_cycle' in bench_name:
                bench_type = 'Insert + Remove'
            elif 'insert' in bench_name:
                bench_type = 'Insert Only'
            else:
                bench_type = 'Unknown'

            chart_name = f'{bench_type} ({n} elements)'

            # Extract tree type from benchmark name (sbt_ or art_)
            tree_type = 'unknown'
            if bench_name.startswith('sbt_'):
                tree_type = 'SBT'
            elif bench_name.startswith('art_'):
                tree_type = 'ART'

            results.append({
                'name': chart_name,
                'unit': 'links/sec',
                'value': int(links_per_second),
                'range': str(int(range_value)),
                'extra': f'tree={tree_type} ops={num_operations} time={time_ns:.2f}ns'
            })

    return results


def main():
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <input_file> <output_file>")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    results = parse_bench_output(input_file)

    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)

    print(f"Converted {len(results)} benchmarks to {output_file}")
    print(f"Results: {json.dumps(results, indent=2)}")


if __name__ == '__main__':
    main()
