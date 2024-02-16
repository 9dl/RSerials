use std::process::Command;
use colored::*;

fn main() {
    loop {
        println!("{}", "Windows Type/Version".bright_yellow());
        execute_command("wmic", &["os", "get", "caption,version"]);

        println!("{}", "Windows Serial number".bright_yellow());
        execute_command("wmic", &["os", "get", "serialnumber"]);

        println!("{}", "Bios".bright_yellow());
        execute_command("wmic", &["bios", "get", "serialnumber"]);

        println!("{}", "smBIos".bright_yellow());
        execute_command("wmic", &["csproduct", "get", "uuid"]);

        println!("{}", "CPU".bright_yellow());
        execute_command("wmic", &["cpu", "get", "name"]);

        println!("{}", "Processor ID".bright_yellow());
        execute_command("wmic", &["cpu", "get", "processorid"]);

        println!("{}", "Memory".bright_yellow());
        execute_command("wmic", &["memorychip", "get", "name,serialnumber"]);

        println!("{}", "Disk drives".bright_yellow());
        execute_command("wmic", &["diskdrive", "get", "model,serialnumber"]);

        println!("{}", "Baseboard".bright_yellow());
        execute_command("wmic", &["baseboard", "get", "serialnumber"]);

        println!("{}", "Product".bright_yellow());
        execute_command("wmic", &["baseboard", "get", "product"]);

        println!("{}", "Keyboard".bright_yellow());
        execute_command("wmic", &["path", "Win32_Keyboard", "get", "Description,DeviceID"]);

        println!("{}", "Mouse".bright_yellow());
        execute_command("wmic", &["path", "Win32_PointingDevice", "get", "Description,PNPDeviceID"]);

        println!("{}", "Monitor".bright_yellow());
        execute_command("wmic", &["path", "Win32_DesktopMonitor", "get", "Description,PNPDeviceID"]);

        println!("{}", "Graphics Card".bright_yellow());
        execute_command("wmic", &["path", "Win32_VideoController", "get", "Description,PNPDeviceID"]);

        println!("{}", "MacAddress".bright_yellow());
        execute_command(
            "wmic",
            &["path", "Win32_NetworkAdapter", "where", "PNPDeviceID like '%%PCI%%' AND NetConnectionStatus=2 AND AdapterTypeID='0'", "get", "MacAddress,Name"]
        );

        println!("{}", "Press any key to check Serial's again.".bright_yellow());
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}

fn execute_command(command: &str, args: &[&str]) {
    if let Ok(output) = Command::new(command)
        .args(args)
        .output() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            let lines: Vec<&str> = stdout.lines().skip(1).collect();
            for line in lines {
                println!("{}", line.trim());
            }
        }
    }
}
