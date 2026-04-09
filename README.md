# PCILeechGui

A lightweight graphical user interface (GUI) wrapper for [PCILeech](https://github.com/ufrisk/pcileech), built with Rust and the [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) (egui) library.

## Features

- **Command Execution**: Run PCILeech commands through a simple interface.
- **Quick Commands**: Pre-defined buttons for common tasks:
  - `Probe Device`: Quickly test connection to the hardware.
  - `Inject "5x shift = CMD"`: Patch target system for CMD access via Sticky Keys.
- **Other Utilities**:
  - `Display`: View memory contents at specific ranges.
  - `KMD Load`: Load Kernel Mode Driver.
  - `Dump Memory`: Save memory dump to a file.
  - `Help`: Access PCILeech help documentation.
- **Real-time Terminal**: Integrated output window that captures and scrolls with the process output.
- **Background Execution**: Commands run in a separate thread, keeping the UI responsive.
- **Windows Integration**: Launch the app without a console window.

## Prerequisites

- **PCILeech**: The `PCILeechGui` app should be placed in the same directory as the `pcileech` executable.
- **Hardware**: Compatible DMA hardware (e.g., Screamer PCIe) if using physical device functions.

## Installation / Building

### Download Executables
Latest executables for **Windows**, **Linux**, and **macOS** are available in the [GitHub Releases](https://github.com/yourusername/PCILeechGui/releases).

### Build from Source
To build the project from source, you need to have [Rust](https://www.rust-lang.org/) installed.

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/PCILeechGui.git
   cd PCILeechGui
   ```

2. Build the application:
   ```bash
   cargo build --release
   ```

3. The executable will be located in `target/release/`.

## Usage

1. Launch `PCILeechGui`.
2. Enter custom arguments in the **Arguments** field or click one of the **Quick Commands**.
3. Click **Run command** to execute.
4. View the results in the **Output** terminal below.
5. Use **Clear Output** to reset the terminal view.

## Development

The project is structured into several modules:
- `main.rs`: Entry point and application initialization.
- `app.rs`: UI layout and event handling logic.
- `pcileech.rs`: Backend process spawning and output management.


## License

This project is licensed under the terms specified in the `Cargo.toml` file (Authors: Naimad).
