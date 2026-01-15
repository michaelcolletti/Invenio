use std::fs::File; 
use std::io::Write;
use std::process::Command;

// Removed duplicate main function


fn discover_on_premises() -> String {
    // Placeholder for on-premises discovery logic
    let mut report = String::new();

    // Discover Linux systems
    report.push_str("## Linux Systems\n");
    report.push_str(&discover_linux());

    // Discover Windows systems
    report.push_str("## Windows Systems\n");
    report.push_str(&discover_windows());

    // Discover Unix systems
    report.push_str("## Unix Systems\n");
    report.push_str(&discover_unix());
    // Discover AIX systems
    report.push_str("## AIX Systems\n");
    report.push_str(&discover_aix());

    report
}

fn discover_linux() -> String {
    let mut report = String::new();

    // Discover CPU information
    report.push_str("### CPU Information\n");
    let output = Command::new("lscpu")
        .output()
        .expect("Failed to execute lscpu command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover memory information
    report.push_str("### Memory Information\n");
    let output = Command::new("free")
        .arg("-h")
        .output()
        .expect("Failed to execute free command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover storage information
    report.push_str("### Storage Information\n");
    let output = Command::new("lsblk")
        .output()
        .expect("Failed to execute lsblk command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover filesystem information
    report.push_str("### Filesystem Information\n");
    let output = Command::new("df")
        .arg("-h")
        .output()
        .expect("Failed to execute df command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover network interfaces
    report.push_str("### Network Interfaces\n");
    let output = Command::new("ip")
        .arg("addr")
        .output()
        .expect("Failed to execute ip addr command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover OS information
    report.push_str("### OS Information\n");
    let output = Command::new("cat")
        .arg("/etc/os-release")
        .output()
        .expect("Failed to execute cat command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    report
}

fn discover_windows() -> String {
    let mut report = String::new();

    // Discover CPU information
    report.push_str("### CPU Information\n");
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-WmiObject Win32_Processor | Select-Object -Property Name, NumberOfCores, NumberOfLogicalProcessors")
        .output()
        .expect("Failed to execute PowerShell command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover memory information
    report.push_str("### Memory Information\n");
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-WmiObject Win32_PhysicalMemory | Select-Object -Property Capacity")
        .output()
        .expect("Failed to execute PowerShell command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover storage information
    report.push_str("### Storage Information\n");
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-WmiObject Win32_DiskDrive | Select-Object -Property Model, Size")
        .output()
        .expect("Failed to execute PowerShell command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover filesystem information
    report.push_str("### Filesystem Information\n");
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-WmiObject Win32_LogicalDisk | Select-Object -Property DeviceID, FileSystem, FreeSpace, Size")
        .output()
        .expect("Failed to execute PowerShell command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover network interfaces
    report.push_str("### Network Interfaces\n");
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-NetAdapter | Select-Object -Property Name, InterfaceDescription, MacAddress, Status")
        .output()
        .expect("Failed to execute PowerShell command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover OS information
    report.push_str("### OS Information\n");
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-ComputerInfo | Select-Object -Property WindowsVersion, WindowsBuildLabEx, OsArchitecture")
        .output()
        .expect("Failed to execute PowerShell command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    report
}

fn discover_unix() -> String {
    let mut report = String::new();

    // Discover CPU information
    report.push_str("### CPU Information\n");
    let output = Command::new("lscpu")
        .output()
        .expect("Failed to execute lscpu command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover memory information
    report.push_str("### Memory Information\n");
    let output = Command::new("free")
        .arg("-h")
        .output()
        .expect("Failed to execute free command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover storage information
    report.push_str("### Storage Information\n");
    let output = Command::new("lsblk")
        .output()
        .expect("Failed to execute lsblk command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover filesystem information
    report.push_str("### Filesystem Information\n");
    let output = Command::new("df")
        .arg("-h")
        .output()
        .expect("Failed to execute df command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover network interfaces
    report.push_str("### Network Interfaces\n");
    let output = Command::new("ip")
        .arg("addr")
        .output()
        .expect("Failed to execute ip addr command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover OS information
    report.push_str("### OS Information\n");
    let output = Command::new("uname")
        .arg("-a")
        .output()
        .expect("Failed to execute uname command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    report
}

fn discover_aix() -> String {
    let mut report = String::new();

    // Discover CPU information
    report.push_str("### CPU Information\n");
    let output = Command::new("lsattr")
        .arg("-El")
        .arg("proc0")
        .output()
        .expect("Failed to execute lsattr command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover memory information
    report.push_str("### Memory Information\n");
    let output = Command::new("lsattr")
        .arg("-El")
        .arg("sys0")
        .output()
        .expect("Failed to execute lsattr command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover storage information
    report.push_str("### Storage Information\n");
    let output = Command::new("lsvg")
        .output()
        .expect("Failed to execute lsvg command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover filesystem information
    report.push_str("### Filesystem Information\n");
    let output = Command::new("df")
        .arg("-k")
        .output()
        .expect("Failed to execute df command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover network interfaces
    report.push_str("### Network Interfaces\n");
    let output = Command::new("netstat")
        .arg("-in")
        .output()
        .expect("Failed to execute netstat command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Discover OS information
    report.push_str("### OS Information\n");
    let output = Command::new("oslevel")
        .output()
        .expect("Failed to execute oslevel command");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

// Discover logical volumes
        report.push_str(&String::from_utf8_lossy(&output.stdout));
let output = Command::new("lsvg")
    .arg("-o")
    .output()
    .expect("Failed to execute lsvg command for logical volumes");
report.push_str(&String::from_utf8_lossy(&output.stdout));

// Discover volume groups
report.push_str("### Volume Groups\n");
let output = Command::new("lsvg")
    .output()
    .expect("Failed to execute lsvg command for volume groups");
report.push_str(&String::from_utf8_lossy(&output.stdout));

report
}

fn discover_macos() -> String {
    let mut report = String::new();

    // Hardware Overview
    report.push_str("<details open>\n<summary>\n\n### Hardware Overview\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPHardwareDataType")
        .output()
        .expect("Failed to execute system_profiler for hardware");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // CPU Information (detailed)
    report.push_str("\n<details open>\n<summary>\n\n### Detailed CPU Information\n\n</summary>\n\n");
    let cpu_info = vec![
        ("Brand", "machdep.cpu.brand_string"),
        ("Core Count", "machdep.cpu.core_count"),
        ("Thread Count", "machdep.cpu.thread_count"),
        ("L1 Cache Size", "hw.l1icachesize"),
        ("L2 Cache Size", "hw.l2cachesize"),
        ("L3 Cache Size", "hw.l3cachesize"),
        ("CPU Frequency", "hw.cpufrequency"),
        ("Bus Frequency", "hw.busfrequency"),
        ("CPU Type", "hw.cputype"),
        ("CPU Subtype", "hw.cpusubtype"),
    ];

    for (label, key) in cpu_info {
        let output = Command::new("sysctl")
            .arg("-n")
            .arg(key)
            .output();
        if let Ok(output) = output {
            if output.status.success() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
                report.push_str(&format!("{}: {}\n", label, value));
            }
        }
    }
    report.push_str("\n</details>\n");

    // Memory Information (detailed)
    report.push_str("\n<details open>\n<summary>\n\n### Memory Information\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPMemoryDataType")
        .output()
        .expect("Failed to execute system_profiler for memory");
    report.push_str(&String::from_utf8_lossy(&output.stdout));

    // Additional memory details from sysctl
    let mem_info = vec![
        ("Physical Memory", "hw.memsize"),
        ("Page Size", "hw.pagesize"),
    ];

    for (label, key) in mem_info {
        let output = Command::new("sysctl")
            .arg("-n")
            .arg(key)
            .output();
        if let Ok(output) = output {
            if output.status.success() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
                report.push_str(&format!("{}: {}\n", label, value));
            }
        }
    }
    report.push_str("\n</details>\n");

    // Storage Information (detailed)
    report.push_str("\n<details open>\n<summary>\n\n### Storage Devices\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPStorageDataType")
        .output()
        .expect("Failed to execute system_profiler for storage");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // NVMe Information
    report.push_str("\n<details open>\n<summary>\n\n### NVMe Devices\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPNVMeDataType")
        .output()
        .expect("Failed to execute system_profiler for NVMe");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Disk utility information
    report.push_str("\n<details open>\n<summary>\n\n### Disk Layout\n\n</summary>\n\n");
    let output = Command::new("diskutil")
        .arg("list")
        .output()
        .expect("Failed to execute diskutil list");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // SATA Information
    report.push_str("\n<details open>\n<summary>\n\n### SATA Devices\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPSerialATADataType")
        .output()
        .expect("Failed to execute system_profiler for SATA");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Network Information (comprehensive)
    report.push_str("\n<details>\n<summary>\n\n### Network Overview\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPNetworkDataType")
        .output()
        .expect("Failed to execute system_profiler for network");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Ethernet Details
    report.push_str("\n<details>\n<summary>\n\n### Ethernet Information\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPEthernetDataType")
        .output()
        .expect("Failed to execute system_profiler for ethernet");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // WiFi/AirPort Details
    report.push_str("\n<details>\n<summary>\n\n### WiFi (AirPort) Information\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPAirPortDataType")
        .output()
        .expect("Failed to execute system_profiler for AirPort");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Bluetooth Information
    report.push_str("\n<details>\n<summary>\n\n### Bluetooth Information\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPBluetoothDataType")
        .output()
        .expect("Failed to execute system_profiler for Bluetooth");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Network interfaces (ifconfig)
    report.push_str("\n<details>\n<summary>\n\n### Network Interfaces (ifconfig)\n\n</summary>\n\n");
    let output = Command::new("ifconfig")
        .output()
        .expect("Failed to execute ifconfig");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Network routes
    report.push_str("\n<details>\n<summary>\n\n### Network Routing Table\n\n</summary>\n\n");
    let output = Command::new("netstat")
        .arg("-nr")
        .output()
        .expect("Failed to execute netstat");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Network protocols statistics
    report.push_str("\n<details>\n<summary>\n\n### Network Protocol Statistics\n\n</summary>\n\n");
    let output = Command::new("netstat")
        .arg("-s")
        .output()
        .expect("Failed to execute netstat -s");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Display/Graphics Information
    report.push_str("\n<details>\n<summary>\n\n### Display and Graphics Information\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
        .expect("Failed to execute system_profiler for displays");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Power and Battery Information
    report.push_str("\n<details>\n<summary>\n\n### Power and Battery Information\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPPowerDataType")
        .output()
        .expect("Failed to execute system_profiler for power");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Power management settings
    report.push_str("\n<details>\n<summary>\n\n### Power Management Settings\n\n</summary>\n\n");
    let output = Command::new("pmset")
        .arg("-g")
        .output()
        .expect("Failed to execute pmset");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Battery health and cycles (if applicable)
    report.push_str("\n<details>\n<summary>\n\n### Battery Status\n\n</summary>\n\n");
    let output = Command::new("pmset")
        .arg("-g")
        .arg("batt")
        .output()
        .expect("Failed to execute pmset -g batt");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Thermal information (via powermetrics - requires root, so may fail)
    report.push_str("\n<details>\n<summary>\n\n### Thermal Information (requires root privileges)\n\n</summary>\n\n");
    let output = Command::new("sudo")
        .arg("-n")
        .arg("powermetrics")
        .arg("--samplers")
        .arg("thermal")
        .arg("-n")
        .arg("1")
        .arg("-i")
        .arg("1000")
        .output();
    if let Ok(output) = output {
        if output.status.success() {
            report.push_str(&String::from_utf8_lossy(&output.stdout));
        } else {
            report.push_str("Thermal information unavailable (requires root privileges)\n");
        }
    } else {
        report.push_str("Thermal information unavailable (requires root privileges)\n");
    }
    report.push_str("\n</details>\n");

    // USB Devices
    report.push_str("\n<details>\n<summary>\n\n### USB Devices\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPUSBDataType")
        .output()
        .expect("Failed to execute system_profiler for USB");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Thunderbolt/USB-C Information
    report.push_str("\n<details>\n<summary>\n\n### Thunderbolt Information\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPThunderboltDataType")
        .output()
        .expect("Failed to execute system_profiler for Thunderbolt");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // PCI Devices
    report.push_str("\n<details>\n<summary>\n\n### PCI Devices\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPPCIDataType")
        .output()
        .expect("Failed to execute system_profiler for PCI");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Audio Information
    report.push_str("\n<details>\n<summary>\n\n### Audio Devices\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPAudioDataType")
        .output()
        .expect("Failed to execute system_profiler for audio");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Camera Information
    report.push_str("\n<details>\n<summary>\n\n### Camera Information\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPCameraDataType")
        .output()
        .expect("Failed to execute system_profiler for camera");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // OS Information
    report.push_str("\n<details>\n<summary>\n\n### Operating System Information\n\n</summary>\n\n");
    let output = Command::new("system_profiler")
        .arg("SPSoftwareDataType")
        .output()
        .expect("Failed to execute system_profiler for software");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // Kernel and system information
    report.push_str("\n<details>\n<summary>\n\n### Kernel Information\n\n</summary>\n\n");
    let output = Command::new("uname")
        .arg("-a")
        .output()
        .expect("Failed to execute uname");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // System uptime
    report.push_str("\n<details>\n<summary>\n\n### System Uptime\n\n</summary>\n\n");
    let output = Command::new("uptime")
        .output()
        .expect("Failed to execute uptime");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    // I/O Registry hardware tree (abbreviated for key components)
    report.push_str("\n<details>\n<summary>\n\n### I/O Registry - Platform Expert\n\n</summary>\n\n");
    let output = Command::new("ioreg")
        .arg("-l")
        .arg("-p")
        .arg("IODeviceTree")
        .arg("-n")
        .arg("efi")
        .output()
        .expect("Failed to execute ioreg");
    report.push_str(&String::from_utf8_lossy(&output.stdout));
    report.push_str("\n</details>\n");

    report
}

fn check_command_availability(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
fn discover_aws() -> String {
    if !check_command_availability("aws") {
        return "AWS CLI is not available.\n".to_string();
    }
    let output = Command::new("aws")
        .arg("ec2")
        .arg("describe-instances")
        .output()
        .expect("Failed to execute AWS CLI command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn discover_gcp() -> String {
    if !check_command_availability("gcloud") {
        return "GCP CLI is not available.\n".to_string();
    }
    let output = Command::new("gcloud")
        .arg("compute")
        .arg("instances")
        .arg("list")
        .output()
        .expect("Failed to execute GCP CLI command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn discover_oci() -> String {
    if !check_command_availability("oci") {
        return "OCI CLI is not available.\n".to_string();
    }
    let output = Command::new("oci")
        .arg("compute")
        .arg("instance")
        .arg("list")
        .output()
        .expect("Failed to execute OCI CLI command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_os_type() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "linux") {
        "linux".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else if cfg!(target_os = "freebsd") || cfg!(target_os = "openbsd") || cfg!(target_os = "netbsd") || cfg!(target_os = "solaris") {
        "unix".to_string()
    } else if cfg!(target_os = "unix") {
        "unix".to_string()
    } else if cfg!(target_os = "aix") {
        "aix".to_string()
    } else {
        "unknown".to_string()
    }
}

fn main() {
    let os_type = get_os_type();
    let mut report = String::new();

    match os_type.as_str() {
        "windows" => {
            report.push_str("# Windows Systems\n");
            report.push_str(&discover_windows());
        }
        "linux" => {
            report.push_str("# Linux Systems\n");
            report.push_str(&discover_linux());
        }
        "macos" => {
            report.push_str("# macOS Systems\n");
            report.push_str(&discover_macos());
        }
        "unix" => {
            report.push_str("# Unix Systems\n");
            report.push_str(&discover_unix());
        }
        "aix" => {
            report.push_str("# AIX Systems\n");
            report.push_str(&discover_aix());
        }
        _ => {
            report.push_str("Unsupported OS\n");
        }
    }

    // Write report to invenio.md
    let mut file = File::create("invenio.md").expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");
}