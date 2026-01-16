# TUI-Based Spreadsheet Editor (Rust)

A **terminal-based spreadsheet application** written in **Rust**, built as part of the **COP290 Rust Lab**. The project focuses on a clean separation between the spreadsheet engine and multiple frontends, including a minimal command-only interface and a Vim-like TUI extension.

For full design details, implementation rationale, and feature discussion, **refer to**:

> `./report/report.pdf`

---

## Repository Structure

```
.
├── Cargo.toml            # Workspace root
├── spreadsheet_core/     # Core spreadsheet engine (logic, parsing, evaluation)
├── cli/                  # Command-only ASCII interface
├── ext/                  # Vim-like interactive TUI extension
├── report/               # Project report (PDF)
├── README.md
└── Makefile
```

---

## Crates Overview

### `spreadsheet_core`

* Core data structures and spreadsheet semantics
* Formula lexer, parser (LALRPOP), and evaluator
* Dependency tracking, undo/redo logic
* CSV import/export support

### `cli`

* **Purely command-driven interface**
* ASCII-based spreadsheet view
* No cursor navigation or UI modes
* Intended as a minimal frontend over the core engine

### `ext`

* **Interactive terminal UI (Vim-like)**
* Cursor-based navigation and modes
* Built using `crossterm` and `ratatui`
* Implements most user-facing interaction features

---

## Key Features (Summary)

* Multiple sheets with dynamic resizing
* Complex formulas with conditionals and functions
* Copy–paste (cell and range), autofill (AP/GP)
* CSV import/export
* Undo/redo (assignment-based)
* ASCII graphing support

> Detailed command syntax, UI behavior, and extension discussion are documented in the report.

---

## Build & Run

This repository includes a **Makefile** for common workflows (used by the autograder).

### Build (Autograder)

```bash
make build
```

Builds the release binary for the **command-only CLI** (`spreadsheet`).

### Run Extensions (TUI)

```bash
make ext1
```

Runs the **Vim-like TUI extension** (`ext`) with preset arguments.

### Tests

```bash
make test
```

Runs tests for the CLI package.

### Coverage

```bash
make coverage
```

Generates coverage using `cargo-tarpaulin` (UI code excluded).

### Documentation

```bash
make docs
```

Builds API documentation for the CLI crate.

---

## Contributors

* Arjun Sammi  (https://github.com/ExactHarmony917)
* Popat Nihal Alkesh (https://github.com/Pianissimo-3115)
* Viraaj Narolia (https://github.com/viraaz11)

---

## Notes

This README is intentionally concise. Architectural decisions, data structures, supported commands, implemented extensions, and limitations are explained in detail in the accompanying **project report**.

