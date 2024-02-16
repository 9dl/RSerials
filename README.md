# RSerials

RSerials is a Rust program that retrieves various system information on a Windows platform using the Windows Management Instrumentation Command-line (WMIC) utility. It fetches details such as Windows version, serial numbers, BIOS information, CPU details, memory information, disk drives, baseboard details, peripherals like keyboard and mouse, monitor, graphics card, and MAC addresses of network adapters. It provides a comprehensive snapshot of the hardware configuration of the system.

## Usage
Prebuild: https://github.com/9dl/RSerials/releases
1. Make sure you have Rust installed on your system.
2. Clone this repository.
3. Navigate to the cloned directory.
4. Compile the program using `cargo build --release`.
5. Run the executable generated in the `target/release` directory.

## Features

- **Windows Type/Version**: Fetches information about the Windows operating system, including its caption and version.
- **Windows Serial number**: Retrieves the serial number of the Windows operating system.
- **BIOS**: Fetches details about the BIOS, including the serial number.
- **smBIOS**: Retrieves the Universally Unique Identifier (UUID) of the computer system.
- **CPU**: Fetches details about the CPU, including its name.
- **Processor ID**: Retrieves the processor ID of the CPU.
- **Memory**: Fetches details about memory chips, including their names and serial numbers.
- **Disk drives**: Retrieves information about disk drives, including their models and serial numbers.
- **Baseboard**: Fetches details about the baseboard, including its serial number and product.
- **Product**: Retrieves information about the baseboard product.
- **Keyboard**: Fetches details about the keyboard, including its description and device ID.
- **Mouse**: Retrieves details about the mouse, including its description and PNPDeviceID.
- **Monitor**: Fetches information about the monitor, including its description and PNPDeviceID.
- **Graphics Card**: Retrieves details about the graphics card, including its description and PNPDeviceID.
- **MacAddress**: Fetches MAC addresses of network adapters that are connected and have a PCI connection, along with their names.
- **Continuously Refreshes**: The program operates in a loop, allowing users to press any key to refresh and check serials again.


## Disclaimer

This tool is provided as-is, without any warranty. Use it responsibly and at your own risk.

