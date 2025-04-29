# BioLens

[![Build](https://github.com/sebastianstupak/biolens/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sebastianstupak/biolens/actions/workflows/ci.yml)
[![Documentation](https://github.com/sebastianstupak/biolens/actions/workflows/cd-documentation.yml/badge.svg)](https://github.com/sebastianstupak/biolens/actions/workflows/cd-documentation.yml)
[![Codecov](https://codecov.io/gh/sebastianstupak/biolens/graph/badge.svg?token=V98LV8E5AJ)](https://codecov.io/gh/sebastianstupak/biolens)

BioLens is a command-line tool for bioinformatics analysis and visualization.

_NOTE: A small side project created in order to learn Rust and the domain of bioinformatics._

## Installation

### Easy Install

BioLens is available in two versions:

- **Standard version**: Smaller, includes basic functionality
- **Visualization version**: Larger, includes visualization features

The installer will prompt you to choose which version to install, or you can specify it directly with the `VISUALIZATION` parameter.

#### Unix (Linux/macOS)

```bash
# Interactive installation (will prompt for version choice)
curl -sSL https://raw.githubusercontent.com/sebastianstupak/biolens/main/install.sh | sh

# Standard version (no visualization)
VISUALIZATION=false curl -sSL https://raw.githubusercontent.com/sebastianstupak/biolens/main/install.sh | sh

# Visualization version
VISUALIZATION=true curl -sSL https://raw.githubusercontent.com/sebastianstupak/biolens/main/install.sh | sh
```

#### Windows (PowerShell)

```powershell
# Interactive installation (will prompt for version choice)
irm https://raw.githubusercontent.com/sebastianstupak/biolens/main/install.ps1 | iex

# Standard version (no visualization)
$env:VISUALIZATION="false"; irm https://raw.githubusercontent.com/sebastianstupak/biolens/main/install.ps1 | iex

# Visualization version
$env:VISUALIZATION="true"; irm https://raw.githubusercontent.com/sebastianstupak/biolens/main/install.ps1 | iex
```

### Install Specific Version

#### Unix (Linux/macOS)

```bash
# Specific version (interactive)
VERSION=0.0.1 curl -sSL https://raw.githubusercontent.com/sebastianstupak/biolens/main/install.sh | sh

# Specific version with visualization
VERSION=0.0.1 VISUALIZATION=true curl -sSL https://raw.githubusercontent.com/sebastianstupak/biolens/main/install.sh | sh
```

#### Windows (PowerShell)

```powershell
# Specific version (interactive)
$env:VERSION="0.0.1"; irm https://raw.githubusercontent.com/sebastianstupak/biolens/main/install.ps1 | iex

# Specific version with visualization
$env:VERSION="0.0.1"; $env:VISUALIZATION="true"; irm https://raw.githubusercontent.com/sebastianstupak/biolens/main/install.ps1 | iex
```

### Manual Installation

1. Download the latest release for your platform from [GitHub Releases](https://github.com/sebastianstupak/biolens/releases)
2. Extract the archive
3. Move the binary to a location in your PATH

### From Source

```bash
cargo install --git https://github.com/sebastianstupak/biolens --features visualization
```

## Roadmap

- Validate file (FASTA, FASTQ, SAM/BAM, VCF, GFF, GTF)
- SAM/BAM coverage
- Sequence comparison

## Features

- TODO:

## Development

BioLens is built with Rust and uses a modular architecture:

- `core`: Main application and analysis logic
- `common`: Shared data structures
- `viz`: Visualization components (optional)
- `web-docs`: Documentation

To build from source with all features:

```bash
cargo build --workspace --features visualization
```
