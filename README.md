# 💻hwinfo

A simple windows-only command line tool that displays useful system information.

Example:

<img width="631" alt="hwinfon" src="https://github.com/user-attachments/assets/b9fc25c5-944a-4744-ba6d-493077f491d5" />

## Usage

Just run:

    hwinfo

There are no command line flags or arguments.

## Details

The information displayed includes:

- System Information:
  - Model Name
  - Service Tag or Serial Number
  - BIOS Version
  - SMBIOS Version
- CPU Information:
  - Name
  - Speed
  - Number of cores
- Video Card Name
- Total System Memory
- Number of memory slots
- List of installed memory modules with following info for each:
  - DIMM number
  - Capacity
- Operating System Information:
  - OS Name (eg. Microsoft Windows 10 Pro) 
  - Windows version
  - Windows build number
- Hostname
- Domain or workgroup name
- User information:
  - local username
  - registered email 
- List of physical storage drives along with:
  - Model Name
  - Capacity
- List of logical storage volumes along with:
  - Drive letter
  - Volume Name
  - Remote Name
  - Capacity
  - Free space
- List of active network adapters along with :
   - NIC Name
   - MAC Adresses (if available)
   - DHCP Status
   - IP Addresses
   - Default Gateway
   - DNS Servers

 The output is colorized on the terminal, but can be piped into a file as plain text.

 ## Installing

 ### Cargo

  Install via `cargo`:

     cargo install hw-info 

**⚠️Note:** this is a windows only tool.

### Scoop

 This tool is also distributed via `scoop` (see [scoop.sh](https://scoop.sh)).

 First, you need to add my bucket:

    scoop bucket add maciak https://github.com/maciakl/bucket

 Next simply run:
 
    scoop install maciak/hwinfo

**⚠️Note:** there is another `hwinfo` package in the `extras` bucket, so please make sure you always use `maciak/hwinfo` to avoid conflicts.
