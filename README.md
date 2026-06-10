# Unit Converter

A simple Rust console application for converting units of measurement.

## Features

The application asks for:

- the value to convert;
- the unit to convert from;
- the unit to convert to.

After that, it prints the converted result or shows a message if the conversion is not supported.

## Supported Conversions

- `meters` -> `feet`
- `feet` -> `meters`
- `kilograms` -> `pounds`
- `pounds` -> `kilograms`
- `centimeters` -> `inches`
- `inches` -> `centimeters`
- `meters` -> `inches`
- `inches` -> `meters`
- `kilograms` -> `grams`
- `grams` -> `kilograms`
- `meters` -> `centimeters`
- `centimeters` -> `meters`

## How to Run

Make sure Rust is installed.

Run this command in the project folder:

```bash
cargo run
```

## Example

```text
unit converter
Enter the value to convert:
10
Enter the unit to convert from:
meters
Enter the unit to convert to:
feet
Result: 32.8084
```

## Project Structure

```text
unit_converter/
|-- Cargo.toml
|-- Cargo.lock
`-- src/
    `-- main.rs
```

## Note

Unit names must be entered in English, exactly as shown in the supported conversions list.
