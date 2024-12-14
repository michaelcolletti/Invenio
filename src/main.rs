use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let mut report = String::new();

    // Discover AWS components
    report.push_str("# AWS Components\n");
    report.push_str(&discover_aws());

    // Discover GCP components
    report.push_str("# GCP Components\n");
    report.push_str(&discover_gcp());

    // Discover OCI components
    report.push_str("# OCI Components\n");
    report.push_str(&discover_oci());

    // Discover Azure components
    report.push_str("# Azure Components\n");
    report.push_str(&discover_azure());

    // Discover on-premises components
    report.push_str("# On-Premises Components\n");
    report.push_str(&discover_on_premises());

    // Write report to invenio.md
    let mut file = File::create("invenio.md").expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");
}

fn discover_aws() -> String {
    // Placeholder for AWS discovery logic
    let output = Command::new("aws")
        .arg("ec2")
        .arg("describe-instances")
        .output()
        .expect("Failed to execute AWS CLI command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn discover_gcp() -> String {
    // Placeholder for GCP discovery logic
    let output = Command::new("gcloud")
        .arg("compute")
        .arg("instances")
        .arg("list")
        .output()
        .expect("Failed to execute GCP CLI command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn discover_oci() -> String {
    // Placeholder for OCI discovery logic
    let output = Command::new("oci")
        .arg("compute")
        .arg("instance")
        .arg("list")
        .output()
        .expect("Failed to execute OCI CLI command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn discover_azure() -> String {
    // Placeholder for Azure discovery logic
    let output = Command::new("az")
        .arg("vm")
        .arg("list")
        .output()
        .expect("Failed to execute Azure CLI command");
    String::from_utf8_lossy(&output.stdout).to_string()
}

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

    report
}