# Rust Lab – Bug Spotting & Testing

> **Educational Rust project** featuring intentional bug exercises, comprehensive testing strategies, and progressive learning modules covering ownership, concurrency, error handling, and performance optimization.

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://img.shields.io/badge/CI-GitHub%20Actions-blue)](.github/workflows)

---

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Development](#development)
- [Testing](#testing)
- [CI/CD](#cicd)
- [Documentation](#documentation)
- [Roadmap](#roadmap)
- [License](#license)

---

## Overview

Rust Lab is a work-in-progress educational repository designed to demonstrate advanced Rust skills through intentional bug spotting exercises, comprehensive testing, and debugging practice. Each module contains progressive exercises that demonstrate different aspects of the Rust programming language.

The project is structured to help developers understand Rust's unique features through hands-on practice with intentionally buggy code, followed by corrected implementations and comprehensive test suites.

### Key Features

- **Intentional Bug Exercises**: Each category includes exercises with deliberate bugs for debugging practice
- **Corrected Implementations**: Fixed versions demonstrate proper Rust patterns and solutions
- **Comprehensive Testing**: Unit tests, integration tests, and property-based testing examples
- **Multiple Rust Topics**: Coverage of ownership, borrowing, error handling, concurrency, memory management, and performance
- **CI/CD Integration**: Automated testing across multiple Rust versions (stable, beta, nightly)
- **Documentation**: Detailed explanations of bugs, solutions, and Rust concepts

---

## Project Structure

```
rust_lab/
├── exercises/                    # Exercise modules organized by category
│   ├── ownership_borrowing/      # Ownership & Borrowing exercises
│   │   ├── ownership_basics.rs           # Exercise with intentional bugs
│   │   └── ownership_basics_fixed.rs     # Corrected implementation
│   ├── error_handling/           # Error Handling & Recovery
│   │   ├── error_handling_basics.rs
│   │   └── error_handling_basics_fixed.rs
│   ├── concurrency/              # Concurrency & Async Programming
│   │   ├── concurrency_basics.rs
│   │   └── concurrency_basics_fixed.rs
│   ├── memory_management/       # Memory Management Patterns
│   │   ├── memory_management.rs
│   │   └── memory_management_fixed.rs
│   └── performance/              # Performance & Optimization
│       ├── performance_optimization.rs
│       └── performance_optimization_fixed.rs
│
├── tests/                        # Integration tests
│   ├── ownership_tests.rs
│   ├── error_handling_tests.rs
│   ├── concurrency_tests.rs
│   ├── memory_management_tests.rs
│   └── performance_tests.rs
│
├── docs/                         # Documentation and explanations
│   └── ownership_explanation.md
│
├── scripts/                      # Automation scripts
│   └── run_exercise.sh           # Exercise execution helper
│
├── src/                          # Main source code
│   └── main.rs
│
├── .github/
│   └── workflows/                # CI/CD pipelines
│       ├── rust-ci.yml           # Main CI workflow
│       └── exercise-validation.yml
│
└── Cargo.toml                    # Project configuration
```

---

## Getting Started

### Prerequisites

- **Rust**: Rust 1.70+ (stable, beta, or nightly)
- **Cargo**: Included with Rust installation
- **Git**: For version control

### Installation

```bash
# Clone the repository
git clone git@github.com:Franklin-Osede/Rust-Lab.git
cd Rust-Lab

# Build the project
cargo build

# Run all tests
cargo test
```

### Running Exercises

**Using Cargo directly:**

```bash
# Run an exercise with intentional bugs
cargo run --bin ownership_basics

# Run the corrected version
cargo run --bin ownership_basics_fixed

# Run error handling exercises
cargo run --bin error_handling_basics
cargo run --bin error_handling_basics_fixed

# Run concurrency exercises
cargo run --bin concurrency_basics
cargo run --bin concurrency_basics_fixed

# Run performance exercises
cargo run --bin performance_optimization
cargo run --bin performance_optimization_fixed

# Run memory management exercises
cargo run --bin memory_management
cargo run --bin memory_management_fixed
```

**Using the helper script:**

```bash
# Make script executable (first time only)
chmod +x scripts/run_exercise.sh

# List available exercises
./scripts/run_exercise.sh list

# Run a specific exercise
./scripts/run_exercise.sh run ownership_basics

# Run all tests
./scripts/run_exercise.sh test

# Build the project
./scripts/run_exercise.sh build

# Clean build artifacts
./scripts/run_exercise.sh clean

# Generate documentation
./scripts/run_exercise.sh doc
```

---

## Development

### Exercise Categories

**Ownership & Borrowing**
- Demonstrates Rust's ownership system
- Common pitfalls with move semantics
- Borrowing rules and lifetime management
- Solutions using references, cloning, and smart pointers

**Error Handling**
- Result and Option types
- Error propagation patterns
- Panic recovery strategies
- Custom error types and error chains

**Concurrency**
- Thread management and synchronization
- Channels for message passing
- Shared state with Arc and Mutex
- Async/await patterns (planned)

**Memory Management**
- Stack vs heap allocation
- Smart pointers (Box, Rc, Arc)
- Memory safety patterns
- Resource cleanup and RAII

**Performance Optimization**
- Zero-cost abstractions
- Benchmarking with Criterion
- Optimization techniques
- Profiling and analysis

### Adding New Exercises

1. Create exercise files in the appropriate category directory:
   - `exercises/{category}/{exercise_name}.rs` (with bugs)
   - `exercises/{category}/{exercise_name}_fixed.rs` (corrected)

2. Add binary targets to `Cargo.toml`:
```toml
[[bin]]
name = "exercise_name"
path = "exercises/{category}/{exercise_name}.rs"

[[bin]]
name = "exercise_name_fixed"
path = "exercises/{category}/{exercise_name}_fixed.rs"
```

3. Create corresponding tests in `tests/{category}_tests.rs`

4. Update documentation in `docs/` if needed

---

## Testing

### Test Structure

The project includes comprehensive test suites for each exercise category:

- **Unit Tests**: Test individual functions and methods
- **Integration Tests**: Test complete exercise workflows
- **Property-Based Tests**: Using Proptest for randomized testing (optional feature)

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific category
cargo test ownership
cargo test error_handling
cargo test concurrency

# Run tests with output
cargo test -- --nocapture

# Run tests with specific filter
cargo test test_name_pattern

# Run tests with benchmarks (requires benchmarks feature)
cargo test --features benchmarks
```

### Test Coverage

Coverage reports are generated in CI/CD pipelines. To generate locally:

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

---

## CI/CD

### GitHub Actions Workflows

**Main CI Pipeline** (`.github/workflows/rust-ci.yml`)

- **Multi-version Testing**: Tests on stable, beta, and nightly Rust
- **Code Quality**: Runs rustfmt and clippy checks
- **Documentation**: Generates and deploys documentation
- **Benchmarks**: Performance benchmarks on main branch
- **Coverage Reports**: Code coverage analysis

**Exercise Validation** (`.github/workflows/exercise-validation.yml`)

- Validates that exercises compile correctly
- Ensures fixed versions run without errors
- Verifies test suites pass

### Workflow Features

- Automatic testing on push and pull requests
- Multi-version Rust compatibility checks
- Code formatting verification
- Linting with clippy
- Documentation generation
- Coverage reporting
- Manual workflow dispatch support

---

## Documentation

### Generated Documentation

```bash
# Generate and open documentation
cargo doc --open

# Generate documentation for all dependencies
cargo doc --all

# Generate documentation without dependencies
cargo doc --no-deps
```

### Available Documentation

- **Ownership Explanation**: `docs/ownership_explanation.md` - Detailed explanation of ownership and borrowing concepts
- **Code Documentation**: Inline documentation in exercise files
- **API Documentation**: Generated via `cargo doc`

---

## Roadmap

### Completed

- [x] Basic exercise structure for ownership and borrowing
- [x] Error handling exercises
- [x] Concurrency basics exercises
- [x] Memory management exercises
- [x] Performance optimization exercises
- [x] Integration test suites
- [x] CI/CD pipeline setup
- [x] Helper scripts for exercise execution
- [x] Basic documentation structure

### In Progress

- [ ] Advanced concurrency patterns (async/await)
- [ ] Additional exercise variations
- [ ] Enhanced documentation
- [ ] More comprehensive test coverage
- [ ] Benchmark suite expansion

### Planned

- [ ] Advanced ownership patterns (lifetimes, HRTB)
- [ ] Unsafe Rust exercises
- [ ] Macro system exercises
- [ ] WebAssembly integration examples
- [ ] Embedded systems examples
- [ ] Advanced error handling patterns
- [ ] Performance profiling guides
- [ ] Interactive learning modules

---

## Contributing

This is a work-in-progress educational repository. Contributions, suggestions, and improvements are welcome. Please ensure that:

- All exercises compile and run correctly
- Tests pass for both buggy and fixed versions
- Code follows Rust style guidelines (rustfmt)
- No clippy warnings (unless intentional for educational purposes)
- Documentation is updated accordingly

---

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

## Support

For questions, issues, or contributions:

- **Issues**: Open an issue on GitHub
- **Documentation**: Check the `docs/` directory
- **Exercises**: Review exercise files and their fixed counterparts
- **Tests**: Examine test files for expected behavior

---

**Note**: This project is a work in progress. Some exercises may be incomplete or subject to change as the project evolves.
