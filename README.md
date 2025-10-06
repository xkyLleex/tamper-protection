# Tamper Protection

A Rust project focused on implementing tamper protection mechanisms. This project aims to detect and prevent unauthorized modifications to files on a folder.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Installation

Clone the repository:

```bash
git clone https://github.com/xkyLleex/tamper_protection.git
cd tamper_protection
```

Build the project:

```bash
cargo build --release
```

### Usage

1. After build it, get execute file from `target/release/tamper_protection.exe` or download it from [Release](https://github.com/xkyLleex/tamper-protection/releases)

2. running with terminal: `tamper_protection.exe <your_folder> <salt>`

3. if all done, you will get the hash code from program output.

## License

This project is licensed under the MIT License.