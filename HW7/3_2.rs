use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        println!("Usage: cargo run -- <start> <end> <step> <label>");
        return;
    }

    let start: i32 = args[1].parse().expect("Invalid start value");
    let end: i32 = args[2].parse().expect("Invalid end value");
    let step: i32 = args[3].parse().expect("Invalid step size");
    let label: &str = &args[4];

    println!("<table>");
    println!("  <tr>");
    println!("    <th>{}</th>", label);
    println!("    <th>{}</th>", format!("{}^2", label));
    println!("    <th>{}</th>", format!("{}^3", label));
    println!("  </tr>");

    for x in (start..=end).step_by(step as usize) {
        let x_squared = x * x;
        let x_cubed = x * x * x;
        println!("  <tr>");
        println!("    <td>{}</td>", x);
        println!("    <td>{}</td>", x_squared);
        println!("    <td>{}</td>", x_cubed);
        println!("  </tr>");
    }

    println!("</table>");
}
