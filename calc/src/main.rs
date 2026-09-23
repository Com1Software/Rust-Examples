use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: calc <add|sub|mul|div> <num1> <num2>");
        std::process::exit(1);
    }

    let op = &args[1];
    let a: f64 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: '{}' is not a number", args[2]);
            std::process::exit(1);
        }
    };

    let b: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: '{}' is not a number", args[3]);
            std::process::exit(1);
        }
    };

    let result = match op.as_str() {
        "add" => a + b,
        "sub" => a - b,
        "mul" => a * b,
        "div" => {
            if b == 0.0 {
                eprintln!("Error: division by zero");
                std::process::exit(1);
            }
            a / b
        }
        _ => {
            eprintln!("Unknown operation '{}'", op);
            std::process::exit(1);
        }
    };

    println!("Result: {}", result);
}
