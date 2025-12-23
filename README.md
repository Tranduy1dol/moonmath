# moonmath

[![Notebooks](https://github.com/Tranduy1dol/moonmath/actions/workflows/notebooks.yml/badge.svg)](https://github.com/Tranduy1dol/moonmath/actions/workflows/notebooks.yml)

Interactive Jupyter notebooks for learning the [Moonmath Manual](https://leastauthority.com/community-matters/moonmath-manual/) — a comprehensive guide to Zero-Knowledge Proofs.

## Features

- **Interactive Learning**: Executable Rust code cells for hands-on practice.
- **Chapter Coverage**: Exercises from Chapters 3-8 of the Moonmath Manual.
- **Cryptographic Libraries**: Uses [mathlib](https://github.com/Tranduy1dol/mathlib) and [curvelib](https://github.com/Tranduy1dol/curvelib) for implementations.
- **In-depth Notes**: Summaries and detailed explanations for each chapter.

## Installation

1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install evcxr Jupyter Kernel**:
   ```bash
   cargo install evcxr_jupyter
   evcxr_jupyter --install
   ```

3. **Start Jupyter**:
   ```bash
   jupyter notebook notebooks/
   ```

## Usage

Navigate to the `notebooks/` directory and open one of the chapters:

| Notebook | Topic | Description |
|----------|-------|-------------|
| [Chapter 3](notebooks/chapter_3_arithmetic.ipynb) | Arithmetic | Modulo, polynomials, extended GCD, CRT, Lagrange interpolation. |
| [Chapter 4](notebooks/chapter_4_algebra.ipynb) | Algebra | Groups, rings, fields, cyclic groups, pairings. |
| [Chapter 5](notebooks/chapter_5_elliptic_curves.ipynb) | Elliptic Curves | Curve equations, point addition, scalar multiplication. |
| [Chapter 6](notebooks/chapter_6_statements.ipynb) | Statements | R1CS, algebraic circuits, verification. |
| [Chapter 7](notebooks/chapter_7_circuit_compilers.ipynb) | Circuit Compilers | R1CS, PAPER language, primitive types. |
| [Chapter 8](notebooks/chapter_8_zero_knowledge_protocols.ipynb) | Zero Knowledge Protocols | Groth16 setup, prover, and verifier phases. |

### Example Code

Notebooks use `evcxr` to run Rust code directly:

```rust
// Load dependencies
:dep mathlib = { git = "https://github.com/Tranduy1dol/mathlib" }

// Use the library
use mathlib::{u1024, BigInt};
let a = u1024!(100u64);
println!("{:?}", a);
```

## Development

This project uses [just](https://github.com/casey/just) for task automation.

### Available Commands

```bash
just --list              # Show all available commands
just validate            # Run all notebook validations
just validate-json       # Validate notebook JSON structure
just validate-issues     # Check for common issues
just validate-metadata   # Verify kernel metadata consistency
just validate-rust       # Validate Rust syntax in code cells
just list-notebooks      # List all notebooks with summary
just install-deps        # Install Python dependencies
just clean-outputs       # Clear notebook outputs
```

### Setup

```bash
# Install just (if not already installed)
cargo install just

# Install Python dependencies
just install-deps

# Validate all notebooks
just validate
```

## Dependencies

- **[mathlib](https://github.com/Tranduy1dol/mathlib)**: High-performance mathematical library (BigInt, Fields, Polynomials).
- **[curvelib](https://github.com/Tranduy1dol/curvelib)**: Elliptic curve cryptography library.
- **Python packages**: See [requirements.txt](requirements.txt) for validation tools.

## License

This project is licensed under the MIT License.
