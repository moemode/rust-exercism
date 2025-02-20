# Exercism Solutions - Rust Track

This repository contains my solutions to various [Exercism](https://exercism.org) exercises in the Rust track.

## Completed Exercises

| # | Exercise | Solution | Topics | Status |
|---|----------|----------|---------|--------|
| 1 | [Hello World](./rust/hello-world) | [lib.rs](./rust/hello-world/src/lib.rs) | Basic setup, Unit testing | ✅ |
| 2 | [Robot Name](./rust/robot-name) | [lib.rs](./rust/robot-name/src/lib.rs) | Random generation, String manipulation, Mutability | ✅ |
| 3 | [Simple Linked List](./rust/simple-linked-list) | [lib.rs](./rust/simple-linked-list/src/lib.rs) | Data structures, Generics, Memory management | ✅ |
| 4 | [PaaS I/O](./rust/paasio) | [lib.rs](./rust/paasio/src/lib.rs) | I/O traits, Read/Write implementation, Metrics tracking | ✅ |
| 5 | [Fizzy](./rust/fizzy) | [lib.rs](./rust/fizzy/src/lib.rs) | Iterator adaptation, Generics, Trait bounds | ✅ |
| 6 | [Triangle](./rust/triangle) | [lib.rs](./rust/triangle/src/lib.rs) | Pattern matching, Geometry | ✅ |
| 7 | [Robot Simulator](./rust/robot-simulator) | [lib.rs](./rust/robot-simulator/src/lib.rs) | State machines, Enums | ✅ |
| 8 | [Accumulate](./rust/accumulate) | [lib.rs](./rust/accumulate/src/lib.rs) | Higher-order functions, Iterators | ✅ |
| 9 | [Word Count](./rust/word-count) | [lib.rs](./rust/word-count/src/lib.rs) | String manipulation, HashMap | ✅ |
| 10 | [Grep](./rust/grep) | [lib.rs](./rust/grep/src/lib.rs) | File I/O, Regular expressions | ✅ |
| 11 | [Luhn](./rust/luhn) | [lib.rs](./rust/luhn/src/lib.rs) | String parsing, Algorithms | ✅ |
| 12 | [Luhn From](./rust/luhn-from) | [lib.rs](./rust/luhn-from/src/lib.rs) | Traits, From/Into | ✅ |
| 13 | [Luhn Trait](./rust/luhn-trait) | [lib.rs](./rust/luhn-trait/src/lib.rs) | Trait implementation | ✅ |
| 14 | [Clock](./rust/clock) | [lib.rs](./rust/clock/src/lib.rs) | Arithmetic, Display trait | ✅ |
| 15 | [Space Age](./rust/space-age) | [lib.rs](./rust/space-age/src/lib.rs) | Traits, Float arithmetic | ✅ |
| 16 | [Sublist](./rust/sublist) | [lib.rs](./rust/sublist/src/lib.rs) | Generics, List operations | ✅ |
| 17 | [Binary Search](./rust/binary-search) | [lib.rs](./rust/binary-search/src/lib.rs) | Algorithms, Arrays | ✅ |
| 18 | [ETL](./rust/etl) | [lib.rs](./rust/etl/src/lib.rs) | Data transformation, Collections | ✅ |
| 19 | [Grade School](./rust/grade-school) | [lib.rs](./rust/grade-school/src/lib.rs) | HashMaps, Sorting | ✅ |
| 20 | [Hamming](./rust/hamming) | [lib.rs](./rust/hamming/src/lib.rs) | Iterator, Error handling | ✅ |
| 21 | [Isogram](./rust/isogram) | [lib.rs](./rust/isogram/src/lib.rs) | HashSet, Chars | ✅ |
| 22 | [Nucleotide Count](./rust/nucleotide-count) | [lib.rs](./rust/nucleotide-count/src/lib.rs) | HashMap, Error handling | ✅ |
| 23 | [Macros](./rust/macros) | [lib.rs](./rust/macros/src/lib.rs) | Macro rules, Metaprogramming | ✅ |
| 24 | [Parallel Letter Frequency](./rust/parallel-letter-frequency) | [lib.rs](./rust/parallel-letter-frequency/src/lib.rs) | Concurrency, Threading | 🔄 |
| 25 | [Xorcism](./rust/xorcism) | [lib.rs](./rust/xorcism/src/lib.rs) | Iterators, Bit manipulation | 🔄 |
| 26 | [React](./rust/react) | [lib.rs](./rust/react/src/lib.rs) | Observer pattern, Cell types | 🔄 |
| 27 | [Circular Buffer](./rust/circular-buffer) | [lib.rs](./rust/circular-buffer/src/lib.rs) | Ring buffer, Error handling | 🔄 |
| 28 | [Forth](./rust/forth) | [lib.rs](./rust/forth/src/lib.rs) | Interpreters, Stack-based | 🔄 |
| 29 | [Doubly Linked List](./rust/doubly-linked-list) | [lib.rs](./rust/doubly-linked-list/src/lib.rs) | Unsafe Rust, Pointers | 🔄 |

## Running Tests

```bash
cd rust/<exercise-name>
cargo test -- --include-ignored
```

## Helper Utilities

### next.sh - Exercise Download Helper

A convenience script that automates downloading the next exercise:
- Scans this README.md to find the first exercise marked as in-progress (🔄)
- Checks if the exercise already exists locally
- Downloads the exercise using exercism CLI if not present
- Uses the configured exercism workspace path

Usage:
```bash
./next.sh
```