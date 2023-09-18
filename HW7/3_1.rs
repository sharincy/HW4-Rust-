use std::env;

fn fahrenheit_to_celsius(fahrenheit: i32) -> f64 {
    (5.0 / 9.0) * (fahrenheit as f64 - 32.0)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        println!("Usage: cargo run -- <start> <end> <step> <unit>");
        return;
    }

    let start: i32 = args[1].parse().expect("Invalid start temperature");
    let end: i32 = args[2].parse().expect("Invalid end temperature");
    let step: i32 = args[3].parse().expect("Invalid step size");
    let unit: &str = &args[4];

    println!("<table>");
    println!("  <tr>");
    println!("    <th>{} ({})</th>", unit, unit);
    println!("    <th>{} (Celsius)</th>", unit);
    println!("  </tr>");

    let mut fahrenheit = start;
    while fahrenheit <= end {
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("  <tr>");
        println!("    <td>{}</td>", fahrenheit);
        println!("    <td>{:.1}</td>", celsius);
        println!("  </tr>");
        fahrenheit += step;
    }

    println!("</table>");
}
