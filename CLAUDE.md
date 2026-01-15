# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Invenio is an inventory and discovery tool built in Rust for the Layer Migration Platform. It performs system discovery across multiple operating systems (Linux, Windows, Unix, AIX) and cloud platforms (AWS, GCP, OCI), generating comprehensive inventory reports.

## Build and Development Commands

```bash
# Build the project
cargo build

# Build with verbose output
cargo build --verbose

# Build optimized release binary
cargo build --release

# Run the program (generates invenio.md report)
cargo run

# Run tests
cargo test

# Run tests with verbose output
cargo test --verbose

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Architecture Overview

### Single Binary Application
The application is structured as a single-file Rust binary (`src/main.rs`) that:
- Detects the host operating system at runtime using `cfg!` macros
- Executes OS-specific discovery functions based on detection
- Generates a markdown report (`invenio.md`) with system inventory

### OS Detection Flow
1. `get_os_type()` determines the runtime OS (windows, linux, macos, unix, aix)
2. `main()` dispatches to the appropriate discovery function
3. Discovery functions execute OS-specific commands and collect output
4. Report is written to `invenio.md` in the current directory

### Discovery Functions by Platform

**On-Premises Systems:**
- `discover_linux()` - Uses `lscpu`, `free`, `lsblk`, `df`, `ip addr`, `/etc/os-release`
- `discover_windows()` - Uses PowerShell WMI queries (`Get-WmiObject`, `Get-NetAdapter`)
- `discover_unix()` - Similar to Linux, uses `uname -a` for OS info
- `discover_aix()` - Uses AIX-specific commands (`lsattr`, `lsvg`, `oslevel`, `netstat`)

**Cloud Platforms:**
- `discover_aws()` - Uses AWS CLI (`aws ec2 describe-instances`)
- `discover_gcp()` - Uses gcloud CLI (`gcloud compute instances list`)
- `discover_oci()` - Uses OCI CLI (`oci compute instance list`)

Note: Cloud discovery functions check CLI availability via `check_command_availability()` before execution.

### Information Collected
Each OS discovery function gathers:
- CPU/processor information
- Memory/RAM details
- Storage/disk information
- Filesystem and mount points
- Network interfaces and configuration
- OS version and build information

## Dependencies

Key external dependencies (from `Cargo.toml`):
- `rusoto_core`, `rusoto_credential`, `rusoto_ec2` (0.47) - AWS SDK integration
- `serde`, `serde_json` (1.0) - Serialization framework
- `walkdir` (2.3) - Directory traversal
- `sysinfo` (0.25) - System information gathering

## CI/CD

GitHub Actions workflow (`.github/workflows/rust.yml`):
- Triggers on push/PR to `main` branch
- Runs on `ubuntu-latest`
- Executes `cargo build --verbose` and `cargo test --verbose`

## Output

The program generates `invenio.md` containing discovered system information formatted as markdown with sections for each category (CPU, Memory, Storage, Network, OS).
