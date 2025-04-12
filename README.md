# BioLens

[![Build](https://github.com/sebastianstupak/biolens/actions/workflows/build.yml/badge.svg)](https://github.com/sebastianstupak/biolens/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/sebastianstupak/biolens/graph/badge.svg?token=V98LV8E5AJ)](https://codecov.io/gh/sebastianstupak/biolens)

BioLens is a command-line tool for bioinformatics analysis and visualization.

## Features

- TODO

## Installation

```bash
# Install from source
cargo install --git https://github.com/sebastianstupak/biolens --features visualization
```

## Usage

### BAM coverage analysis

biolens bam-coverage -i sample.bam
biolens bamcov -i sample.bam -r chr1:1000-2000 -v

### Sequence comparison

## Development

BioLens is built with Rust and uses a modular architecture:

- `core`: Main application and analysis logic
- `common`: Shared data structures
- `viz`: Visualization components (optional)

To build from source with all features:

```bash
# Install from source
cargo build --workspace --features visualization
```
