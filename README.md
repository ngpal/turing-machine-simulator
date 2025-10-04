# Turing Machine Simulator (Rust)

A simple command-line Turing machine simulator written in Rust.\
It reads a transition table from a text file, asks for an input string, and simulates the machine until it halts.

## Table of Contents

- [Build & Run](#build-run)
- [Input File Format](#input-file-format)
- [Features](#features)
- [Implementation Notes](#implementation-notes)
- [Roadmap](#roadmap)

## Build & Run

Build and run with cargo

```bash
cargo run -- <input file>
```

## Input File Format

- The first line lists the final (accepting) states, comma-separated.
- Each subsequent line defines a transition in the format:

```
(current_state, read_symbol) -> (next_state, write_symbol, direction)
```

- direction can be either `<` (move left) or `>` (move right)
- `_` represents a blank cell

## Features

- Parses Turing machine transition definitions from a plain text file
- Uses a hash map for the transition table for fast lookups
- Interactive simulation with live tape updates
- Accept/reject halts based on final states

## Implementation Notes

- Tape length = 256 cells with wraparound
- All uninitialized tape cells are `_`
- Simulator starts in state `q0` and halts when no valid transition exists
- If the current state is in the list of final states, the machine accepts; otherwise, it halts with an error

## Roadmap

- [x] Parse input files and run the machine
- [ ] Instantaneous description
- [ ] Turing machine variants
