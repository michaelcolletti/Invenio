# SKILLS.md

This document provides guidance for integrating new and updated operating systems into Invenio for the Layer Migration Platform.

## Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [Adding a New Operating System](#adding-a-new-operating-system)
3. [Updating an Existing Operating System](#updating-an-existing-operating-system)
4. [OS-Specific Command Reference](#os-specific-command-reference)
5. [Testing Your Integration](#testing-your-integration)
6. [Best Practices](#best-practices)
7. [Troubleshooting](#troubleshooting)

## Architecture Overview

Invenio uses a three-part architecture for OS discovery:

```
┌─────────────────┐
│  get_os_type()  │  ← Detects OS at compile time (cfg! macros)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     main()      │  ← Dispatches to appropriate discovery function
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ discover_xxx()  │  ← Executes OS-specific commands
└─────────────────┘
```

### Key Components

1. **OS Detection (`get_os_type()`)**: Uses Rust `cfg!` macros to detect the target OS at compile time
2. **Dispatch Logic (`main()`)**: Routes to the appropriate discovery function based on detected OS
3. **Discovery Functions**: OS-specific functions that gather hardware/software inventory

## Adding a New Operating System

### Step 1: Add OS Detection

Edit `src/main.rs` and update the `get_os_type()` function:

```rust
fn get_os_type() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "linux") {
        "linux".to_string()
    // ADD YOUR NEW OS HERE
    } else if cfg!(target_os = "your_new_os") {
        "your_new_os".to_string()
    } else {
        "unknown".to_string()
    }
}
```

**Supported `target_os` values**: `windows`, `linux`, `macos`, `freebsd`, `openbsd`, `netbsd`, `solaris`, `aix`, `android`, `ios`, `dragonfly`, `haiku`, `illumos`, etc.

### Step 2: Create Discovery Function

Create a new discovery function following this template:

```rust
fn discover_your_new_os() -> String {
    let mut report = String::new();

    // Hardware Overview (expanded by default)
    report.push_str("<details open>\n<summary>\n\n### Hardware Overview\n\n</summary>\n\n");
    let output = Command::new("your_hardware_command")
        .arg("--options")
        .output()
        .expect("Failed to execute hardware command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // CPU Information (expanded by default)
    report.push_str("\n<details open>\n<summary>\n\n### CPU Information\n\n</summary>\n\n");
    let output = Command::new("your_cpu_command")
        .output()
        .expect("Failed to execute CPU command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Memory Information (expanded by default)
    report.push_str("\n<details open>\n<summary>\n\n### Memory Information\n\n</summary>\n\n");
    let output = Command::new("your_memory_command")
        .output()
        .expect("Failed to execute memory command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Storage Information (expanded by default)
    report.push_str("\n<details open>\n<summary>\n\n### Storage Information\n\n</summary>\n\n");
    let output = Command::new("your_storage_command")
        .output()
        .expect("Failed to execute storage command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Network Information (collapsed by default)
    report.push_str("\n<details>\n<summary>\n\n### Network Information\n\n</summary>\n\n");
    let output = Command::new("your_network_command")
        .output()
        .expect("Failed to execute network command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Add more sections as needed...

    report
}
```

### Step 3: Add Dispatch Logic

Update the `main()` function to include your new OS:

```rust
fn main() {
    let os_type = get_os_type();
    let mut report = String::new();

    match os_type.as_str() {
        "windows" => {
            report.push_str("# Windows Systems\n");
            report.push_str(&discover_windows());
        }
        // ADD YOUR NEW OS HERE
        "your_new_os" => {
            report.push_str("# Your New OS Systems\n");
            report.push_str(&discover_your_new_os());
        }
        _ => {
            report.push_str("Unsupported OS\n");
        }
    }

    // Write report to file
    let mut file = File::create("invenio.md").expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");
}
```

### Step 4: Test Your Integration

See [Testing Your Integration](#testing-your-integration) section below.

## Updating an Existing Operating System

### Adding New Discovery Sections

To add new hardware/software discovery to an existing OS:

1. **Locate the discovery function** (e.g., `discover_linux()`, `discover_macos()`)

2. **Add your new section** following the collapsible pattern:

```rust
// For EXPANDED sections (hardware, CPU, memory, storage):
report.push_str("\n<details open>\n<summary>\n\n### Your New Section\n\n</summary>\n\n");
let output = Command::new("your_command")
    .arg("--option")
    .output()
    .expect("Failed to execute command");
report.push_str(&String::from_utf8_lossy(&output.stdout));
report.push_str("\n</details>\n");

// For COLLAPSED sections (network, peripherals, etc.):
report.push_str("\n<details>\n<summary>\n\n### Your New Section\n\n</summary>\n\n");
// ... same as above
```

3. **Use proper error handling** for commands that may not be available:

```rust
let output = Command::new("optional_command")
    .output();

if let Ok(output) = output {
    if output.status.success() {
        report.push_str(&String::from_utf8_lossy(&output.stdout));
    } else {
        report.push_str("Command failed or not available\n");
    }
} else {
    report.push_str("Command not found\n");
}
```

### Updating Command Arguments

When OS commands change between versions:

1. **Check command availability first**:

```rust
fn check_command_availability(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
```

2. **Provide fallback commands**:

```rust
// Try modern command first, fall back to legacy
let output = Command::new("new_command").output();
if output.is_err() {
    let output = Command::new("old_command").output()
        .expect("Neither new nor old command available");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
} else {
    report.push_str(&String::from_utf8_lossy(&output.unwrap().stdout));
}
```

## OS-Specific Command Reference

### Linux Commands

| Category | Command | Description |
|----------|---------|-------------|
| CPU | `lscpu` | CPU architecture info |
| Memory | `free -h` | Memory usage |
| Storage | `lsblk` | Block devices |
| Filesystem | `df -h` | Disk space usage |
| Network | `ip addr` | Network interfaces |
| OS Info | `cat /etc/os-release` | Distribution info |
| Kernel | `uname -a` | Kernel version |

### macOS Commands

| Category | Command | Description |
|----------|---------|-------------|
| Hardware | `system_profiler SPHardwareDataType` | Hardware overview |
| CPU | `sysctl -n machdep.cpu.*` | CPU details |
| Memory | `system_profiler SPMemoryDataType` | RAM information |
| Storage | `system_profiler SPStorageDataType` | Storage devices |
| NVMe | `system_profiler SPNVMeDataType` | NVMe devices |
| Disks | `diskutil list` | Disk layout |
| Network | `system_profiler SPNetworkDataType` | Network overview |
| WiFi | `system_profiler SPAirPortDataType` | WiFi details |
| Power | `pmset -g` | Power settings |
| Battery | `pmset -g batt` | Battery status |
| Thermal | `sudo powermetrics --samplers thermal -n 1` | Temperature sensors |

### Windows Commands (PowerShell)

| Category | Command | Description |
|----------|---------|-------------|
| CPU | `Get-WmiObject Win32_Processor` | CPU information |
| Memory | `Get-WmiObject Win32_PhysicalMemory` | RAM details |
| Storage | `Get-WmiObject Win32_DiskDrive` | Disk drives |
| Filesystem | `Get-WmiObject Win32_LogicalDisk` | Logical volumes |
| Network | `Get-NetAdapter` | Network adapters |
| OS Info | `Get-ComputerInfo` | System information |

### Unix/BSD Commands

| Category | Command | Description |
|----------|---------|-------------|
| CPU | `lscpu` or `sysctl hw.model` | CPU info |
| Memory | `free -h` or `sysctl hw.physmem` | Memory info |
| Storage | `lsblk` or `geom disk list` | Disk info |
| Network | `ifconfig` | Network interfaces |
| OS Info | `uname -a` | System info |

### AIX Commands

| Category | Command | Description |
|----------|---------|-------------|
| CPU | `lsattr -El proc0` | Processor attributes |
| Memory | `lsattr -El sys0` | System memory |
| Storage | `lsvg` | Volume groups |
| Filesystem | `df -k` | Filesystem usage |
| Network | `netstat -in` | Network interfaces |
| OS Info | `oslevel` | AIX version |

## Testing Your Integration

### Step 1: Compile and Check

```bash
# Check for compilation errors
cargo check

# Build the project
cargo build

# Build release version
cargo build --release
```

### Step 2: Run Discovery

```bash
# Run the discovery tool
cargo run

# Check the generated report
ls -lh invenio.md
head -n 100 invenio.md
```

### Step 3: Validate Output

Check that your report includes:

1. ✅ All expected hardware sections
2. ✅ Properly formatted markdown with collapsible sections
3. ✅ No error messages from failed commands
4. ✅ Reasonable data in each section
5. ✅ Proper HTML details/summary tags

```bash
# Count collapsible sections
grep -c '<details' invenio.md
grep -c '</details>' invenio.md
# These should match!

# Check for expanded sections (should have 'open' attribute)
grep -c '<details open>' invenio.md

# Verify headers
grep '###' invenio.md
```

### Step 4: Cross-Platform Testing

If adding support for a new OS, test on actual hardware/VM:

```bash
# On target OS
git clone <repo>
cd Invenio
cargo build
cargo run

# Verify invenio.md was created and contains expected data
cat invenio.md
```

## Best Practices

### 1. Consistent Section Naming

Use clear, descriptive section names:
- ✅ "CPU Information", "Memory Information", "Storage Devices"
- ❌ "CPU Stuff", "RAM", "Disks"

### 2. Expanded vs Collapsed Sections

**Expand by default** (`<details open>`):
- Hardware Overview
- CPU/Processor Information
- Memory/RAM Information
- Storage Devices

**Collapse by default** (`<details>`):
- Network details
- Peripherals (USB, Bluetooth, etc.)
- Power/Battery information
- OS/Kernel details
- Verbose output (logs, registries, etc.)

### 3. Error Handling

Always handle commands that may not be available:

```rust
// DON'T do this (will panic if command not found):
let output = Command::new("rare_command")
    .output()
    .expect("Failed to execute command");

// DO this instead:
let output = Command::new("rare_command").output();
match output {
    Ok(out) if out.status.success() => {
        report.push_str(&String::from_utf8_lossy(&out.stdout));
    }
    _ => {
        report.push_str("Command not available or failed\n");
    }
}
```

### 4. Command Availability Checks

For optional or platform-specific commands:

```rust
if check_command_availability("special_tool") {
    // Run the command
} else {
    report.push_str("Special tool not installed\n");
}
```

### 5. Privilege Escalation

Some commands require elevated privileges. Handle gracefully:

```rust
report.push_str("\n<details>\n<summary>\n\n### Thermal Information (requires root)\n\n</summary>\n\n");
let output = Command::new("sudo")
    .arg("-n")  // Non-interactive
    .arg("powermetrics")
    .arg("--samplers")
    .arg("thermal")
    .output();

if let Ok(output) = output {
    if output.status.success() {
        report.push_str(&String::from_utf8_lossy(&output.stdout));
    } else {
        report.push_str("Requires root privileges. Run with sudo or configure passwordless sudo.\n");
    }
} else {
    report.push_str("Command not available\n");
}
```

### 6. Data Format Consistency

Keep output format consistent across OS implementations:

```rust
// Good: Structured with labels
report.push_str(&format!("CPU Cores: {}\n", core_count));
report.push_str(&format!("CPU Threads: {}\n", thread_count));

// Less ideal: Raw command output only
report.push_str(&String::from_utf8_lossy(&output.stdout));
```

### 7. Documentation

When adding a new OS or significant changes:

1. Update `CLAUDE.md` with new OS-specific commands
2. Update `README.md` if new dependencies are required
3. Add examples to this `SKILLS.md` document
4. Update GitHub Actions workflow if new test environments are needed

## Troubleshooting

### Issue: Command Not Found

**Symptom**: `Failed to execute <command>`

**Solutions**:
1. Check if command is installed on target OS
2. Verify command is in system PATH
3. Use absolute path: `/usr/bin/command` instead of `command`
4. Add availability check before executing

### Issue: Permission Denied

**Symptom**: Commands fail with permission errors

**Solutions**:
1. Add `sudo` prefix for privileged commands
2. Use `-n` flag for non-interactive sudo
3. Provide fallback message for users without sudo access
4. Document privilege requirements in section header

### Issue: Compilation Errors

**Symptom**: `cfg!(target_os = "xxx")` not recognized

**Solutions**:
1. Check Rust documentation for valid `target_os` values
2. Use `cargo build --target <triple>` for cross-compilation
3. Test on actual target platform if cross-compilation fails

### Issue: Incomplete Output

**Symptom**: Report missing expected information

**Solutions**:
1. Check command exit status: `output.status.success()`
2. Check stderr: `String::from_utf8_lossy(&output.stderr)`
3. Add debug output during development
4. Test commands manually on target OS first

### Issue: Report Too Large

**Symptom**: `invenio.md` is excessively large

**Solutions**:
1. Limit verbose command output: use `head -n 100` equivalent
2. Filter unnecessary information
3. Use collapsed sections for verbose output
4. Consider summarizing data instead of raw output

### Issue: HTML Not Rendering

**Symptom**: `<details>` tags show as text in viewer

**Solutions**:
1. Ensure markdown viewer supports HTML (GitHub, GitLab, VS Code do)
2. Check for proper tag closure: every `<details>` needs `</details>`
3. Verify syntax: `<details open>` vs `<details>` (no `=`)
4. Test with: `grep -c '<details' file.md` and `grep -c '</details>' file.md`

## Example: Adding ChromeOS Support

Here's a complete example of adding ChromeOS support:

### 1. Add OS Detection

```rust
fn get_os_type() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "linux") {
        // Check if it's ChromeOS
        if std::path::Path::new("/etc/lsb-release").exists() {
            if let Ok(contents) = std::fs::read_to_string("/etc/lsb-release") {
                if contents.contains("CHROMEOS") {
                    return "chromeos".to_string();
                }
            }
        }
        "linux".to_string()
    }
    // ... rest of OS detection
}
```

### 2. Create Discovery Function

```rust
fn discover_chromeos() -> String {
    let mut report = String::new();

    report.push_str("<details open>\n<summary>\n\n### Hardware Overview\n\n</summary>\n\n");
    let output = Command::new("cat")
        .arg("/etc/lsb-release")
        .output()
        .expect("Failed to read lsb-release");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    report.push_str("\n<details open>\n<summary>\n\n### CPU Information\n\n</summary>\n\n");
    let output = Command::new("cat")
        .arg("/proc/cpuinfo")
        .output()
        .expect("Failed to read cpuinfo");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Add more sections...

    report
}
```

### 3. Add Dispatch

```rust
match os_type.as_str() {
    "chromeos" => {
        report.push_str("# ChromeOS Systems\n");
        report.push_str(&discover_chromeos());
    }
    // ... other OS cases
}
```

### 4. Test

```bash
cargo build
cargo run
cat invenio.md
```

---

## Additional Resources

- [Rust std::process::Command documentation](https://doc.rust-lang.org/std/process/struct.Command.html)
- [Rust cfg! macro documentation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [Layer Migration Platform documentation](https://example.com) (if available)
- [Invenio GitHub Issues](https://github.com/your-org/invenio/issues)

## Contributing

When contributing OS integrations:

1. Create a feature branch: `git checkout -b feature/add-<os-name>-support`
2. Implement the integration following this guide
3. Test thoroughly on target OS
4. Update documentation (CLAUDE.md, README.md, this file)
5. Submit pull request with:
   - Description of OS support added
   - Sample output from `invenio.md`
   - Test results on target platform
   - Any new dependencies or requirements

---

**Version**: 1.0
**Last Updated**: 2026-01-15
**Maintainer**: Layer Migration Platform Team
