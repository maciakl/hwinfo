#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use colored::Colorize;

fn main() {
    
    let os = std::env::consts::OS;

    if os != "windows" {
        eprintln!("sorry, this program currently only works on windows");
        std::process::exit(1);
    }

    print_info();
}


#[derive(Deserialize, Debug)]
struct Win32_OperatingSystem {
    Caption: String,
    BuildNumber: String,
    RegisteredUser: String,
    SystemDrive: String,
}

#[derive(Deserialize, Debug)]
struct Win32_BIOS {
    SerialNumber: String,
    Version: String,
    SMBIOSBIOSVersion: String,
}

#[derive(Deserialize, Debug)]
struct Win32_ComputerSystem {
    Name: String,
    UserName: String,
    Domain: String,
    Manufacturer: String,
    Model: String,
    TotalPhysicalMemory: usize,
}

#[derive(Deserialize, Debug)]
struct Win32_DiskDrive {
    Model: String,
    Size: usize,
}

#[derive(Deserialize, Debug)]
struct Win32_PhysicalMemoryArray {
    MemoryDevices: usize,
}

#[derive(Deserialize, Debug)]
struct Win32_VideoController {
    Name: String,
}

#[derive(Deserialize, Debug)]
struct Win32_PhysicalMemory {
    Capacity: usize,
    DeviceLocator: String,
}

#[derive(Deserialize, Debug)]
struct Win32_Processor {
    Name: String,
    NumberOfCores: usize,
    MaxClockSpeed: usize,

}#[derive(Deserialize, Debug)]
struct Win32_NetworkAdapterConfiguration {
    Description: String,
    IPEnabled: bool,
    DHCPEnabled: bool,
    DefaultIPGateway: Vec<String>,
    DNSServerSearchOrder: Vec<String>,
    MACAddress: Option<String>,
    IPAddress: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Win32_LogicalDisk {
    DeviceID: String,
    VolumeName: Option<String>,
    ProviderName: Option<String>,
    Size: Option<usize>,
    FreeSpace: Option<usize>,
}

fn print_info() {

    println!();
    println!("{}", "---hwinfo (c) luke maciak-------------------------------------------------------".on_red().italic());
    println!("{}", "   see github.com/maciakl/hwinfo".italic());
    println!();

    let com = COMLibrary::new().unwrap();
    let wmi = WMIConnection::new(com.into()).unwrap();

    let os : Vec<Win32_OperatingSystem>;
    let bios : Vec<Win32_BIOS>;
    let computer : Vec<Win32_ComputerSystem>;
    let disk : Vec<Win32_DiskDrive>;
    let cpu : Vec<Win32_Processor>;
    let memory : Vec<Win32_PhysicalMemoryArray>;

    let dimm : Vec<Win32_PhysicalMemory>;
    let nic : Vec<Win32_NetworkAdapterConfiguration>;
    let vol : Vec<Win32_LogicalDisk>;
    let video : Vec<Win32_VideoController>;
    

    os = wmi.query().unwrap();
    bios = wmi.query().unwrap();
    computer = wmi.query().unwrap();
    disk = wmi.query().unwrap();
    cpu = wmi.query().unwrap();
    memory = wmi.query().unwrap();
    dimm = wmi.query().unwrap();
    nic = wmi.query().unwrap();
    vol = wmi.query().unwrap();
    video = wmi.query().unwrap();
    
    let unknown = String::new(); // use for unknown values



    println!();
    println!("{}", "---Device Information-----------------------------------------------------------".red());
    println!();
    println!("{}:\t\t{}", "Manufacturer".bold().blue(), computer[0].Manufacturer);
    println!("{}:\t\t{}", "Model Name".bold().blue(), computer[0].Model);
    println!("{}:\t\t{}", "Service Tag".bold().blue(), bios[0].SerialNumber);
    println!("{}:\t\t{}", "BIOS Version".bold().blue(), bios[0].Version);
    println!("{}:\t\t{}", "SMBIOS Version".bold().blue(), bios[0].SMBIOSBIOSVersion);
    println!();
    
    println!("{}", "---Processor and Graphics-------------------------------------------------------".red());
    println!();
    for p in &cpu {
        let speed = to_GHz(p.MaxClockSpeed);
        let cores = p.NumberOfCores.to_string();
        println!("{}:\t\t{}", "CPU Name".bold().cyan(), cpu[0].Name);
        println!("{}:\t\t{:.2}GHz", "CPU Speed".bold().cyan(), speed);
        println!("{}:\t{}", "Number of Cores".bold().cyan(), cores);
        println!("{}:\t\t{}", "Video Card".bold().cyan(), video[0].Name);
        println!();
    }

    println!("{}", "---Memory-----------------------------------------------------------------------".red());
    println!();

    let mem = to_GB(computer[0].TotalPhysicalMemory).to_string();

    println!("{}:\t\t{}GB", "Total Memory".bold().yellow(), mem.yellow());
    println!("{}:\t\t{}", "Memory Slots".bold().yellow(), memory[0].MemoryDevices);

    println!();
    let mut capacity:String;
    for m in &dimm {
        capacity = to_GB(m.Capacity).to_string();
        println!("   {}:\t\t{}", "DIMM #".bold().yellow(), m.DeviceLocator);
        println!("   {}:\t\t{}GB", "Capacity".bold().yellow(), capacity.yellow());
        println!();
    }

    println!("{}", "---Operating System-------------------------------------------------------------".red());
    println!();

    let build:&str = &os[0].BuildNumber;
    let version = buid_to_version(build);

    println!("{}:\t{}", "Operating System".bold().purple(), os[0].Caption.yellow());
    println!("{}:\t\t{}", "Build Number".bold().purple(), build);
    println!("{}:\t\t{}", "Version Number".bold().purple(), version);
    println!("{}:\t\t{}", "System Drive".bold().purple(), os[0].SystemDrive);
    println!();

    println!("{}", "---User Information-------------------------------------------------------------".red());
    println!();
    println!("{}:\t\t{}", "Computer Name".bold().red(), computer[0].Name);
    println!("{}:\t\t{}", "Domain Name".bold().red(), computer[0].Domain);
    println!("{}:\t\t{}", "Current User".bold().red(), computer[0].UserName);
    println!("{}:\t{}", "Registered User".bold().red(), os[0].RegisteredUser);
    println!();


    println!("{}", "---Physical Drives--------------------------------------------------------------".red());
    println!();

    let mut size:String;

    for d in &disk {
        size = to_GB(d.Size).to_string();

        println!("{}:\t\t{}", "Disk Model".bold().green(), d.Model);
        println!("{}:\t\t{}GB", "Disk Size".bold().green(), size.yellow());
        println!();
    }

    println!("{}", "---Logical Volumes--------------------------------------------------------------".red());
    println!();

    for v in &vol {
        size = match v.Size { 
            Some(s) => to_GB(s).to_string(),
            _ => "?".to_string()
        };

        let free = match v.FreeSpace {
            Some(f) => to_GB(f).to_string(),
            _ => "?".to_string()
        };

        let vname = v.VolumeName.as_ref().unwrap_or(&unknown);
        let pname = v.ProviderName.as_ref().unwrap_or(&unknown);

        println!("{}:\t\t{}", "Drive Letter".bold().green(), v.DeviceID);
        println!("{}:\t\t{}", "Volume Name".bold().green(), vname);
        println!("{}:\t\t{}", "Remote Name".bold().green(), pname);
        println!("{}:\t\t{}GB", "Volume Size".bold().green(), size.yellow());
        println!("{}:\t\t{}GB", "Free Space".bold().green(), free.yellow());
        println!();
    }

    println!("{}", "---Network Adapters-------------------------------------------------------------".red());
    println!();
    for n in &nic {
        if n.IPEnabled {

            println!("{}:\t\t{}", "NIC Name".bold().bright_blue(), n.Description);
            println!("{}:\t\t{}", "MAC Address".bold().bright_blue(), n.MACAddress.as_ref().unwrap());
            println!("{}:\t\t{}", "DHCP Enabled".bold().bright_blue(), n.DHCPEnabled);
            println!("{}:\t\t{}", "IP Address".bold().bright_blue(), n.IPAddress[0].yellow());
            println!("{}:\t{}", "Default Gateway".bold().bright_blue(), n.DefaultIPGateway[0]);
            print!("{}:\t\t", "DNS Servers".bold().bright_blue());
            for dn in &n.DNSServerSearchOrder {
                print!("{}   ", dn);
            }
            println!();
        }
    }

    println!();
    println!("{}", "--------------------------------------------------------------------------------".red());

}

// Convert bytes to GB returning a rounded unsigned integer
fn to_GB(memory: usize) -> usize {
    let tmp:f32 = memory as f32 / (1024.0 * 1024.0 * 1024.0);
    tmp.ceil() as usize
}

// Convert Hz to GHz returning a rounded float
fn to_GHz(speed:usize) -> f32 {
    let tmp:f32 = speed as f32 / 100.0;
    tmp.round() / 10.0
}

// Translate the windows build number to the more common version number
fn buid_to_version(build:&str) -> String {

    let version = match build {
        "18363" => "1909",
        "19041" => "2004",
        "19042" => "20H2",
        "19043" => "21H1",
        "19044" => "21H2",
        "19045" => "22H2",
        _ => "Unknown"
    };

    version.to_string()
}
