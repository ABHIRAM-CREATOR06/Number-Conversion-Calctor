# Multi-Dimensional Calculator

A **Rust**-based utility library for high-precision conversions and calculations across multiple domains, including arithmetic, geometry, physics, and more. Built with the [`rust_decimal`](https://crates.io/crates/rust_decimal) crate for accurate decimal arithmetic.

---

## Features

### Number Conversion
- **Decimal Conversion**: Convert numbers between bases (up to base 32) with high precision.
- **Language & Script Detection**: Identify the language and script of any input text.

### Geometry
- **Area Calculator**: Compute areas for:
  - Square
  - Rectangle
  - Triangle
  - Circle
  - Regular polygon (n sides)
- **Unit Conversion**: Convert results into various units.

### Physics
- **Physics Calculator**: Calculate and convert:
  - Mass, Force, Energy, Velocity, Pressure, Acceleration, Volume
- **Unit Conversion**: Support for multiple units.

### Temperature
- **Temperature Converter**: Convert between:
  - Celsius, Fahrenheit, Kelvin, Rankine
- **Precision**: Up to 4 decimal places.

### Length
- **Length Converter**: Convert between:
  - Meter, Centimeter, Inch, Foot, Yard, Mile

### Time
- **Time Operations**:
  - Unit conversion
  - Time difference calculation
  - Efficiency calculation

### Miscellaneous
- **Angle Operations**
- **Data Transfer Units**
- **Fuel Economy**: Calculate efficiency, carbon footprint, annual fuel cost, and monthly savings.
- **Network**: Speed prediction and statistics.
- **System Reliability**: Data reliability and predictability metrics.

---

## Project Structure

```text
src/
├── main.rs              # Main entry point
├── arithmetic.rs         # Arithmetic operations
├── area.rs               # Area calculations
├── data.rs               # Data transfer speed operations
├── physics.rs            # Physics operations
├── physicc.rs            # Base conversion (submodule)
├── lenght.rs             # Length operations
├── tempe.rs              # Temperature operations
├── time.rs               # Time operations
target/                   # Debugging files
Cargo.toml
Cargo.lock

```
---

## Dependencies

This project relies on the following Rust crates:
   Crate                  | Version   |
 |------------------------|-----------|
 | rust_decimal           | 1.36      |
 | rust_decimal_macros    | 1.36      |
 | whatlang               | 0.16.4    |

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rust_decimal = "1.36"
rust_decimal_macros = "1.36"
whatlang = "0.16.4"
```

## Installation

- Clone the Repo
```bash
git clone [https://github.com/ABHIRAM-CREATOR06/multi-dimensional-calculator.git](https://github.com/ABHIRAM-CREATOR06/multi-dimensional-calculator.git)

cd multi-dimensional-calculator

```

 - Build and run

``` cargo
cargo build
cargo run
```
---

## Contributing

Contributions make the open-source community an amazing place to learn and inspire.

Please adhere to the Code of Conduct.

Feel free to fork the project and submit a Pull Request.

Show your support by giving the project a ⭐ and following the developer!

---

## ✍️ Author
Abhiram GitHub: @ABHIRAM-CREATOR06

Development Status: ✅ Completed (Sept 21, 2024)

License: MIT
