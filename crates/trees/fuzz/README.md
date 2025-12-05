# Fuzzing Trees with libFuzzer

This directory contains fuzzing targets for the trees crate using cargo-fuzz and libFuzzer.

## Setup

Install cargo-fuzz:

```bash
cargo install cargo-fuzz
```

## Running the Fuzzer

To fuzz tree operations:

```bash
cd crates/trees
cargo fuzz run fuzz_tree_operations
```

To run with a specific timeout (e.g., 60 seconds):

```bash
cargo fuzz run fuzz_tree_operations -- -max_total_time=60
```

To run with multiple jobs in parallel:

```bash
cargo fuzz run fuzz_tree_operations -- -jobs=4
```

## What It Tests

The `fuzz_tree_operations` target:
- Generates random sequences of insert and remove operations
- Executes operations on a Size-Balanced Tree
- Verifies tree invariants after each operation:
  - Tree size metadata matches actual node count
  - All inserted values can be found
  - Removed values cannot be found
  - Tree structure is consistent

## Crashes and Artifacts

If the fuzzer finds a crash, it will save the input in `fuzz/artifacts/fuzz_tree_operations/`.

To reproduce a crash:

```bash
cargo fuzz run fuzz_tree_operations fuzz/artifacts/fuzz_tree_operations/crash-<hash>
```

## Continuous Fuzzing

For continuous integration, you can run the fuzzer with a time limit:

```bash
cargo fuzz run fuzz_tree_operations -- -max_total_time=300
```

This will run for 5 minutes and exit, suitable for CI pipelines.
