# Invenio

**Enterprise-Grade System Discovery & Inventory Tool for the Layer Migration Platform**

[![Rust](https://github.com/michaelcolletti/Invenio/actions/workflows/rust.yml/badge.svg)](https://github.com/michaelcolletti/Invenio/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

---

## Overview

Invenio is a high-performance, cross-platform system discovery tool written in Rust. It automatically detects the host operating system and generates comprehensive hardware and software inventory reports in Markdown format. Designed as a core component of the **Layer Migration Platform**, Invenio enables infrastructure teams to capture detailed system specifications for migration planning, compliance auditing, and capacity analysis.

### Key Features

- **Multi-Platform Support** - Native discovery for Linux, Windows, macOS, Unix/BSD, and AIX
- **Cloud Integration** - Inventory discovery for AWS EC2, Google Cloud Platform, and Oracle Cloud Infrastructure
- **Zero Dependencies at Runtime** - Single static binary with no external runtime requirements
- **Structured Output** - Generates clean, collapsible Markdown reports with HTML details/summary tags
- **Extensible Architecture** - Easy to add support for new operating systems and cloud platforms
- **Docker Ready** - Multi-stage Dockerfile for containerized deployments

---

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later
- OS-specific CLI tools (automatically detected and handled gracefully)

### Installation

```bash
# Clone the repository
git clone https://github.com/michaelcolletti/Invenio.git
cd Invenio

# Build the release binary
cargo build --release

# Run discovery
./target/release/invenio
```

### Output

Invenio generates an `invenio.md` file in the current directory containing:

```markdown
# macOS Systems

<details open>
<summary>

### Hardware Overview

</summary>

Hardware:

    Hardware Overview:
      Model Name: MacBook Pro
      Chip: Apple M2 Pro
      Total Number of Cores: 12
      Memory: 32 GB
      ...

</details>

<!-- Additional collapsible sections for CPU, Memory, Storage, Network, etc. -->
```

---

## Supported Platforms

### On-Premises Systems

| Platform | Status | Discovery Method |
|----------|--------|------------------|
| **Linux** | Stable | `lscpu`, `free`, `lsblk`, `df`, `ip addr`, `/etc/os-release` |
| **Windows** | Stable | PowerShell WMI queries (`Get-WmiObject`, `Get-NetAdapter`) |
| **macOS** | Stable | `system_profiler`, `sysctl`, `diskutil`, `pmset` |
| **Unix/BSD** | Stable | `lscpu`, `free`, `lsblk`, `df`, `ip addr`, `uname` |
| **AIX** | Stable | `lsattr`, `lsvg`, `oslevel`, `netstat` |

### Cloud Platforms

| Platform | Status | CLI Requirement |
|----------|--------|-----------------|
| **AWS** | Stable | AWS CLI (`aws ec2 describe-instances`) |
| **GCP** | Stable | gcloud CLI (`gcloud compute instances list`) |
| **OCI** | Stable | OCI CLI (`oci compute instance list`) |

> Cloud discovery automatically checks for CLI availability before execution.

---

## Information Collected

Invenio gathers comprehensive system information across multiple categories:

### Hardware
- CPU architecture, cores, threads, cache sizes, frequencies
- Physical memory capacity and configuration
- Storage devices (NVMe, SATA, block devices)
- Disk partitions and volume groups

### Software
- Operating system version and build information
- Kernel version and architecture
- Filesystem mounts and usage statistics

### Network
- Network interfaces and MAC addresses
- IP configuration (IPv4/IPv6)
- Routing tables and protocol statistics
- WiFi, Ethernet, and Bluetooth information (where applicable)

### Peripherals (macOS)
- USB devices
- Thunderbolt/USB-C ports
- PCI devices
- Audio and camera hardware
- Display and graphics information

### Power & Thermal (macOS)
- Battery status and health
- Power management settings
- Thermal sensors (requires elevated privileges)

---

## Usage

### Basic Usage

```bash
# Generate inventory report
cargo run

# Or use the compiled binary
./target/release/invenio

# View the generated report
cat invenio.md
```

### Docker Usage

```bash
# Build the Docker image
docker build -t invenio .

# Run in a container
docker run --rm -v $(pwd):/output invenio

# The report will be generated in the mounted volume
```

### Cloud Discovery

For cloud platform discovery, ensure the respective CLI tools are installed and configured:

```bash
# AWS - requires configured credentials
aws configure

# GCP - requires authentication
gcloud auth login

# OCI - requires configured profile
oci setup config
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                          main()                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                   get_os_type()                          │    │
│  │   Uses Rust cfg! macros for compile-time OS detection    │    │
│  └─────────────────────┬───────────────────────────────────┘    │
│                        │                                         │
│                        ▼                                         │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │              OS-Specific Discovery                       │    │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐       │    │
│  │  │ Linux   │ │ Windows │ │ macOS   │ │ AIX     │       │    │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘       │    │
│  └─────────────────────┬───────────────────────────────────┘    │
│                        │                                         │
│                        ▼                                         │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │              Output: invenio.md                          │    │
│  │   Structured Markdown with collapsible HTML sections     │    │
│  └─────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Single Binary** - All functionality compiled into one executable
2. **Compile-Time OS Detection** - Uses Rust's `cfg!` macros for zero-cost abstraction
3. **Graceful Degradation** - Commands that fail or aren't available are handled without crashing
4. **Structured Output** - Consistent Markdown format with GitHub-compatible HTML

---

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Check for errors without building
cargo check
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test --verbose
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Type checking
cargo check
```

### Adding New OS Support

See [SKILLS.md](SKILLS.md) for a comprehensive guide on:
- Adding new operating system support
- Updating existing OS discovery functions
- OS-specific command references
- Best practices and troubleshooting

---

## CI/CD

Invenio uses GitHub Actions for continuous integration:

- **Trigger**: Push or PR to `main` branch
- **Environment**: Ubuntu latest
- **Steps**: Build and test with verbose output

```yaml
# .github/workflows/rust.yml
on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]
```

---

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `rusoto_core` | 0.47 | AWS SDK core functionality |
| `rusoto_credential` | 0.47 | AWS credential management |
| `rusoto_ec2` | 0.47 | AWS EC2 API integration |
| `serde` | 1.0 | Serialization framework |
| `serde_json` | 1.0 | JSON parsing and generation |
| `walkdir` | 2.3 | Directory traversal |
| `sysinfo` | 0.25 | Cross-platform system information |

---

## Project Structure

```
Invenio/
├── src/
│   └── main.rs          # Main application code
├── .github/
│   └── workflows/
│       └── rust.yml     # CI/CD configuration
├── Cargo.toml           # Rust dependencies
├── Cargo.lock           # Locked dependency versions
├── Dockerfile           # Multi-stage Docker build
├── CLAUDE.md            # AI assistant configuration
├── SKILLS.md            # OS integration guide
├── README.md            # This file
└── invenio.md           # Generated output (after running)
```

---

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/add-os-support`)
3. Make your changes following the [SKILLS.md](SKILLS.md) guidelines
4. Run tests (`cargo test`)
5. Format code (`cargo fmt`)
6. Submit a pull request

### Contribution Guidelines

- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Add appropriate error handling for new commands
- Use collapsible sections for verbose output
- Update documentation when adding new features
- Include sample output in PR descriptions

---

## Roadmap

- [ ] JSON output format option
- [ ] Remote system discovery via SSH
- [ ] Kubernetes cluster discovery
- [ ] VMware vSphere integration
- [ ] Azure cloud platform support
- [ ] Configuration file support
- [ ] Differential inventory (change detection)
- [ ] Web UI for report visualization

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Built for the **Layer Migration Platform**
- Inspired by enterprise discovery tools like BMC Discovery, ServiceNow Discovery, and AWS Application Discovery Service
- Powered by [Rust](https://www.rust-lang.org/) for performance and safety

---

<p align="center">
  <strong>Invenio</strong> - Know Your Infrastructure
</p>
