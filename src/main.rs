use std::io;

fn convert_units(value: f64, from_unit: &str, to_unit: &str) -> Option<f64> {
    let conversion_factor = match (from_unit, to_unit) {
        ("meters", "feet") => 3.28084,
        ("feet", "meters") => 0.3048,
        ("kilograms", "pounds") => 2.20462,
        ("pounds", "kilograms") => 0.453592,
        ("centimeters", "inches") => 0.393701,
        ("inches", "centimeters") => 2.54,
        ("meters", "inches") => 39.3701,
        ("inches", "meters") => 0.0254,
        ("kilograms", "grams") => 1000.0,
        ("grams", "kilograms") => 0.001,
        ("meters", "centimeters") => 100.0,
        ("centimeters", "meters") => 0.01,
        _ => return None,
    };
    Some(value * conversion_factor)
}

fn main() {
    println!("unit converter");
    println!("Enter the value to convert:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let value: f64 = input.trim().parse().expect("Failed to parse input");
    println!("Enter the unit to convert from:");
    let mut from_unit = String::new();
    io::stdin().read_line(&mut from_unit).expect("Failed to read line");
    println!("Enter the unit to convert to:");
    let mut to_unit = String::new();
    io::stdin().read_line(&mut to_unit).expect("Failed to read line");
    if let Some(result) = convert_units(value, from_unit.trim().to_lowercase().as_str(),to_unit.trim().to_lowercase().as_str()) {
        println!("Result: {}", result);
    } else {
        println!("Conversion not supported.");
    }
}
