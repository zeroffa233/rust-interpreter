pub mod interpreter;
use interpreter::Interpreter;
use std::io::Write;

fn main() {
    loop {
        let mut interpreter = Interpreter::new();
        let mut input = String::new();
        print!("calc> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let result = interpreter.expr(input.trim().to_string());
        println!("{}", result);
    }
}
