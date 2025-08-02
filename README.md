# Tawa Hotplate Firmware

Tawa is an embedded firmware project for a soldering hotplate, designed to provide precise and customizable temperature control for electronics assembly and reflow soldering. The firmware is written in Rust and supports user-defined temperature curves for advanced soldering profiles.

## Features

- **Custom Temperature Curves:**
  - Supports user-defined time-temperature profiles (polylines) for flexible reflow and preheat processes.
  - Compile-time and runtime validation of curves for safety and reliability.
- **STM32 Microcontroller Support:**
  - Optimized for STM32 MCUs, with plans for additional hardware support.
- **Safe Rust Codebase:**
  - Leverages Rust's safety and reliability for embedded systems.
- **Extensible Design:**
  - Modular architecture allows for easy addition of new interpolation strategies and hardware targets.

## Planned/Future Features

- **Raspberry Pi Pico and ESP32 Support:**
  - Planned support for Raspberry Pi Pico (RP2040) and ESP32 microcontrollers.
- **Web/Serial UI:**
  - User interface for uploading and managing custom temperature curves.
- **Advanced Curve Types:**
  - Support for more complex curve types (e.g., Bezier, step, etc.)
- **Data Logging:**
  - Real-time temperature logging and export.

## Project Structure

- `core/` — Core logic for temperature curve management, validation, and interpolation.
- `src/time_temperature_curve/` — Polyline and curve implementations, error handling, and interfaces.
- `tests/` — Unit and compile-fail tests for curve validation and safety.

## Getting Started

1. **Clone the repository:**
   ```sh
   git clone <repo-url>
   cd tawa-rs
   ```
2. **Build the core library:**
   ```sh
   cargo build -p core
   ```
3. **Run tests:**
   ```sh
   cargo test -p core
   ```

## Contributing

Contributions are welcome! Please open issues or pull requests for new features, hardware support, or bug fixes.

## License

This project is licensed under the MIT License.
