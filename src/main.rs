#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use colored::Colorize;

fn main() {

    get_info();
}


#[derive(Deserialize, Debug)]
struct Win32_OperatingSystem {
    Caption: String,
    BuildNumber: String,
}

#[derive(Deserialize, Debug)]
struct Win32_BIOS {
    SerialNumber: String,
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
struct Win32_Processor {
    Name: String,
    NumberOfCores: usize,
    MaxClockSpeed: usize,
}

fn get_info() {

    let com = COMLibrary::new().unwrap();
    let wmi = WMIConnection::new(com.into()).unwrap();

    let os : Vec<Win32_OperatingSystem>;
    let bios : Vec<Win32_BIOS>;
    let computer : Vec<Win32_ComputerSystem>;
    let disk : Vec<Win32_DiskDrive>;
    let cpu : Vec<Win32_Processor>;
    let memory : Vec<Win32_PhysicalMemoryArray>;

    os = wmi.query().unwrap();
    bios = wmi.query().unwrap();
    computer = wmi.query().unwrap();
    disk = wmi.query().unwrap();
    cpu = wmi.query().unwrap();
    memory = wmi.query().unwrap();


    let mem = to_GB(computer[0].TotalPhysicalMemory).to_string();
    let build:&str = &os[0].BuildNumber;
    let version = buid_to_version(build);
    let speed = to_GHz(cpu[0].MaxClockSpeed);
    let cores = cpu[0].NumberOfCores.to_string();

    println!();
    println!("{}:\t\t{}", "Manufacturer".bold().blue(), computer[0].Manufacturer);
    println!("{}:\t\t{}", "Model Name".bold().blue(), computer[0].Model);
    println!("{}:\t\t{}", "Service Tag".bold().blue(), bios[0].SerialNumber);
    
    println!();
    println!("{}:\t\t{}", "CPU Name".bold().cyan(), cpu[0].Name);
    println!("{}:\t\t{:.2}GHz", "CPU Speed".bold().cyan(), speed);
    println!("{}:\t{}", "Number of Cores".bold().cyan(), cores);

    println!();
    println!("{}:\t\t{}GB", "Total Memory".bold().yellow(), mem.yellow());
    println!("{}:\t\t{}", "Memory Slots".bold().yellow(), memory[0].MemoryDevices);

    println!();
    println!("{}:\t{}", "Operating System".bold().purple(), os[0].Caption.yellow());
    println!("{}:\t\t{}", "Build Number".bold().purple(), build);
    println!("{}:\t\t{}", "Version Number".bold().purple(), version);

    println!();
    println!("{}:\t\t{}", "Computer Name".bold().red(), computer[0].Name);
    println!("{}:\t\t{}", "Domain Name".bold().red(), computer[0].Domain);
    println!("{}:\t\t{}", "User Name".bold().red(), computer[0].UserName);

    println!();

    let mut size:String;
    for d in &disk {
        size = to_GB(d.Size).to_string();
        println!("{}:\t\t{}", "Disk Model".bold().green(), d.Model);
        println!("{}:\t\t{}GB", "Disk Size".bold().green(), size.yellow());
        println!();
    }

}


fn to_GB(memory: usize) -> usize {
    let tmp:f32 = memory as f32 / (1024.0 * 1024.0 * 1024.0);
    tmp.ceil() as usize
}

fn to_GHz(speed:usize) -> f32 {
    let tmp:f32 = speed as f32 / 100.0;
    tmp.round() / 10.0
}

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
