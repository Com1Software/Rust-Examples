#[derive(Debug)]
enum Command {
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Div(i32, i32),
}

fn execute(cmd: Command) -> Option<i32> {
    match cmd {
        Command::Add(a, b) => Some(a + b),
        Command::Sub(a, b) => Some(a - b),
        Command::Mul(a, b) => Some(a * b),
        Command::Div(a, b) => {
            if b == 0 {
                None
            } else {
                Some(a / b)
            }
        }
    }
}

fn main() {
    let commands = vec![
        Command::Add(10, 5),
        Command::Sub(20, 3),
        Command::Mul(7, 6),
        Command::Div(10, 0),
    ];

    for cmd in commands {
        println!("Executing: {:?}", cmd);

        match execute(cmd) {
            Some(result) => println!("Result: {}", result),
            None => println!("Error: invalid operation"),
        }

        println!();
    }
}
