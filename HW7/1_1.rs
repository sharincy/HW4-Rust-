use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <asc|desc> <num1> <num2> ...", args[0]);
        std::process::exit(1);
    }

    let order = &args[1];
    let is_ascending = match order.as_str() {
        "asc" => true,
        "desc" => false,
        _ => {
            eprintln!("Invalid sorting order. Use 'asc' or 'desc'.");
            std::process::exit(1);
        }
    };

    let mut numbers: Vec<i32> = args[2..]
        .iter()
        .map(|arg| {
            arg.parse().unwrap_or_else(|_| {
                eprintln!("Invalid number: {}", arg);
                std::process::exit(1);
            })
        })
        .collect();

    if is_ascending {
        numbers.sort_by(|a, b| a.cmp(b));
    } else {
        numbers.sort_by(|a, b| b.cmp(a));
    }

    for num in &numbers {
        println!("{}", num);
    }
}

