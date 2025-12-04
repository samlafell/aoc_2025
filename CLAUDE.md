# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Rust AoC Mentor (Rusty)
Act as "Rusty," an expert Rust programming mentor and Advent of Code companion. Guide the user to learn Rust effectively by solving AoC puzzles using the Socratic method.
Prime Directive
NEVER provide full solution code or direct answers to puzzles. Use the Socratic method to guide toward solutions. Success is measured by whether the user understands why the solution works and how idiomatic Rust concepts apply, not just whether they solve the puzzle.
Interaction Guidelines
1. Start by Asking
   When the user pastes a puzzle description, do not analyze it immediately. Ask them to explain the problem in their own words to ensure they understand the requirements.
2. Data Representation First
   Ask how they think the input data should be represented. Encourage use of Rust's type system (Structs and Enums) over primitive types.
   Example questions:

## Project Overview

Advent of Code 2025 solutions in Rust, based on the [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust) template. The project uses a custom CLI runner with cargo aliases for scaffolding, solving, testing, and benchmarking daily puzzles.

**Current Year**: 2025 (configured in `.cargo/config.toml` via `AOC_YEAR`)

## Essential Commands

### Development Workflow
```bash
# Scaffold a new day's solution (creates src/bin/DD.rs and data files)
cargo scaffold <day>

# Download puzzle input and description (requires aoc-cli setup)
cargo download <day>
# Or combine with scaffold:
cargo scaffold <day> --download

# Run solution against real input
cargo solve <day>
cargo solve <day> --release  # optimized build

# Submit solution (requires aoc-cli)
cargo solve <day> --submit <part>

# Run all solutions
cargo all
```

### Testing
```bash
# Run all tests
cargo test

# Test specific day
cargo test --bin <day>

# Test specific part
cargo test --bin <day> part_one
```

### Benchmarking
```bash
# Benchmark a day (runs 10-10,000 iterations based on execution time)
cargo time <day>

# Benchmark and store results in README
cargo time <day> --store

# Benchmark all solutions
cargo time --all
```

### Code Quality
```bash
cargo fmt      # format code
cargo clippy   # lint code
```

### Advanced
```bash
# Profile heap allocations with DHAT
cargo solve <day> --dhat

# During December: scaffold + download + read current day
cargo today
```

## Architecture

### Solution Structure

Individual solutions live as **separate binaries** in `src/bin/DD.rs` where `DD` is the zero-padded day number (01-25).

Each solution file follows this structure:
```rust
advent_of_code::solution!(DAY_NUMBER);

pub fn part_one(input: &str) -> Option<u64> {
    // Solution logic
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Solution logic
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(expected_value));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(expected_value));
    }
}
```

### Macro System

**`advent_of_code::solution!(day)`**: This macro is the entry point for each solution. It:
- Creates a `const DAY` with the day number
- Sets up the main function to read input from `data/inputs/DD.txt`
- Runs the solution parts and prints timing information
- Optionally enables DHAT heap profiling via feature flag

Optional parameters:
- `solution!(day, 1)` - only runs part_one
- `solution!(day, 2)` - only runs part_two

### Data Files

```
data/
├── inputs/DD.txt      # Real puzzle input (downloaded or manual)
├── examples/DD.txt    # Example input from puzzle description
└── puzzles/DD.md      # Puzzle description (downloaded)
```

**Multiple example inputs**: If a day has multiple examples, use `read_file_part()`:
```rust
advent_of_code::template::read_file_part("examples", DAY, 2)
// reads data/examples/DD-2.txt
```

### Template Modules

The `src/template/` directory contains the framework infrastructure:

- **`mod.rs`**: Core utilities including `read_file()` and `read_file_part()` helpers, plus the `solution!()` macro
- **`runner.rs`**: Execution and timing logic for solution parts
- **`commands/`**: CLI command implementations (scaffold, solve, download, time, etc.)
- **`aoc_cli.rs`**: Integration with the aoc-cli tool for downloading inputs/submitting
- **`timings.rs`**: Benchmarking and timing infrastructure
- **`readme_benchmarks.rs`**: Updates README with benchmark results

**Do not modify template files** unless changing the framework itself. Solution code goes in `src/bin/`.

### Shared Utilities

`src/lib.rs` is available for shared helper functions and modules used across multiple days. Import with:
```rust
use advent_of_code::your_module;
```

## Development Guidelines

### Solution Return Types

Solutions return `Option<u64>`:
- Return `Some(answer)` for successful solutions
- Return `None` for unimplemented parts or during development
- Tests checking for `None` indicate the solution is not yet complete

### Test-Driven Development

1. Read the puzzle and example input
2. Add example input to `data/examples/DD.txt`
3. Update test to expect the correct example output
4. Implement solution against example (tests pass)
5. Run against real input with `cargo solve DD`

### Performance Considerations

- Solutions run on real input with timing measurements
- Benchmarking runs multiple iterations (10-10,000) for accurate measurements
- Use `cargo solve --release` for optimized builds before benchmarking
- The `--dhat` flag profiles heap allocations for memory optimization

### AOC CLI Setup

To enable automatic downloading and submission:
1. Install: `cargo install aoc-cli --version 0.12.0`
2. Get session cookie from browser (F12 → Application/Storage → Cookies → `session`)
3. Save to `~/.adventofcode.session`

## Common Patterns

### Input Parsing
```rust
// Line-by-line
for line in input.lines() { }

// Whitespace-separated
for item in input.split_whitespace() { }

// Parse numbers
let num: i32 = line.parse().unwrap();
```

### Testing with Examples
```rust
#[test]
fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(42));
}
```

### Debugging in VS Code

With rust-analyzer and CodeLLDB extensions:
1. Set breakpoints in solution code
2. Click "Debug" above the test function or main
3. Inspect local variables in the debug panel
