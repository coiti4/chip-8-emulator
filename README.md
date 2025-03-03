# CHIP-8 Emulator

CHIP-8 Emulator is a Rust-based emulator for the CHIP-8 architecture. It provides a complete implementation of the CHIP-8 instruction set and is designed to run classic CHIP-8 programs and games. The emulator uses SDL2 for rendering and input handling, offering a simple and efficient way to experience retro computing.

![CHIP-8 Emulator Demo](assets/demo.gif)

## Features

   - Full emulation of the CHIP-8 instruction set.
   - Support for classic CHIP-8 ROMs.
   - Keyboard input mapping for CHIP-8 keypad.
   - Pause functionality with a visual "PAUSE" indicator.
   - Easy-to-use file dialog for loading ROMs.
   - Change the current exectuted ROMs using the file dialog.
   - Audio support.

## Project Structure

The repository is organized into the following directories and files to keep the project modular and maintainable:

| Path         | Description                        |
|--------------|------------------------------------|
| `core/`      | Contains the core logic of the CHIP-8 emulator, including the CPU, memory, and instruction set implementation. This module is independent of the UI. |
| `gui/`       | Handles the graphical user interface using SDL2. This includes rendering the CHIP-8 screen, handling user input, and displaying messages like "PAUSE". |
| `roms/`      | Stores CHIP-8 ROM files that can be loaded into the emulator. Includes example ROMs for testing (e.g., games like INVADERS). |

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Controls](#controls)
- [Contributing](#contributing)
- [Acknowledgments](#acknowledgments)
- [License](#license)

## Installation

To build and run the CHIP-8 Emulator, you need to have Rust and Cargo installed on your system. Additionally, you'll need the SDL2 library and its development headers.

### Prerequisites

 - **Rust** (latest stable version recommended)
 - **Cargo**
 - **SDL2** (with development libraries)

On Ubuntu, you can install the required dependencies with:

```bash
sudo apt update
sudo apt install libsdl2-dev libsdl2-ttf-dev
```

### Building the Project

1. Clone the repository:

```bash
git clone https://github.com/coiti4/chip-8-emulator.git
cd chip-8-emulator
```
2. Build the project using Cargo:

```bash
cargo build --release
```

This will create an optimized binary in the target/release/ directory.

## Usage

To run the emulator, use the following command:
```bash
cargo run --release
```

When the emulator starts, it will prompt you to select a CHIP-8 ROM file using a file dialog. You can also provide the path to the ROM as a command-line argument:

```bash
cargo run --release -- /path/to/rom.ch8
```

## Controls

- **CHIP-8 Keypad Mapping:**

The emulator maps the CHIP-8 keypad to the following keyboard keys:

| CHIP-8 Key | Keyboard Key |
|------------|--------------|
| 1          | 1            |
| 2          | 2            |
| 3          | 3            |
| C          | 4            |
| 4          | Q            |
| 5          | W            |
| 6          | E            |
| D          | R            |
| 7          | A            |
| 8          | S            |
| 9          | D            |
| E          | F            |
| A          | Z            |
| 0          | X            |
| B          | C            |
| F          | V            |

- **Pause:** Press P or space to pause or resume the emulator. When paused, the screen will display "PAUSE" and ignore keypad inputs.

- **Load New ROM:** Press Enter to open a file dialog and load a new ROM.

- **Exit:** Press Escape or close the window to exit the emulator.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## Acknowledgments

This project was inspired by the excellent work of [aquova](https://github.com/aquova) in their [chip8-book](https://github.com/aquova/chip8-book) repository. Their guide on building a CHIP-8 emulator provided valuable insights and motivation for this implementation.

The provided Chip-8 games are supplied from [Zophar's Domain](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html). Original author unknown.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
