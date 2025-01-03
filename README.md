# hwinfo

A simple command line tool that displays useful system information.

Example:

<img width="460" alt="hwinfo" src="https://github.com/user-attachments/assets/e1fa04bb-a8f5-4d28-bb21-bc009606fe52" />

## Usage

Just run:

    hwinfo

There are no command line flags or arguments.

## Details

The information displayed includes:

- System Manufacturer
- Model Name
- Service Tag or Serial Number
- BIOS Version
- CPU Name, Speed and number of cores
- Total System Memory
- Number of memory slots
- List of installed memory modules, their DIMM number and capacity
- Windows version and build number
- Hostname
- Domain or workgroup name
- Current and registered usernames
- List of physical storage drives and their capacity
- List of logical storage volumes along with their drive letter, capacity and free space
- List of active network adapters along with the following information for each:
 - MAC Adresses
 - IP Addresses
 - Default Gateway
 - DHCP Enabled
 - DNS Servers

 The output is colorized on the terminal, but can be piped into a file as plain text.

 ## Installing

 This tool is distributed via `scoop` (see [scoop.sh](https://scoop.sh)).

 First, you need to add my bucket:

    scoop bucket add maciak https://github.com/maciakl/bucket

 Next simply run:
 
    scoop install maciak/hwinfo

If you don't want to use `scoop` you can simply download the executable from the release page and extract it somewhere in your path.
