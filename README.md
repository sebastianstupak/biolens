# BioLens

[![Build](https://github.com/sebastianstupak/biolens/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sebastianstupak/biolens/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/sebastianstupak/biolens/graph/badge.svg?token=V98LV8E5AJ)](https://codecov.io/gh/sebastianstupak/biolens)

BioLens is a command-line tool for bioinformatics analysis and visualization.

_NOTE: A small side project created in order to learn Rust and the domain of bioinformatics._

## Installation

```bash
# Install from source
cargo install --git https://github.com/sebastianstupak/biolens --features visualization
```

## Roadmap

- Validate file (FASTA, FASTQ, SAM/BAM, VCF, GFF, GTF)
- SAM/BAM coverage
- Sequence comparison
- Tool installation + Documentation generation

## Features

- TODO:

## Development

BioLens is built with Rust and uses a modular architecture:

- `core`: Main application and analysis logic
- `common`: Shared data structures
- `viz`: Visualization components (optional)

To build from source with all features:

```bash
cargo build --workspace --features visualization
```
