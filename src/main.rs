use std::io::{self, Write};
use oxidized_patterns::patterns::{
    adapter, abstract_factory, builder, command, fold,
    interpreter, newtype, observer, strategy, visitor,
};

fn main() {
    loop {
        println!("\n=== Oxidized Patterns Demo ===");
        println!("Choose a pattern to demo:");
        println!("  1) Builder");
        println!("  2) Fold");
        println!("  3) Command");
        println!("  4) Interpreter");
        println!("  5) Newtype");
        println!("  6) Strategy");
        println!("  7) Visitor");
        println!("  8) Observer");
        println!("  9) Abstract Factory");
        println!(" 10) Adapter");
        println!("  0) Exit");

        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }

        match input.trim() {
            "1" => builder::demo(),
            "2" => fold::demo(),
            "3" => command::demo(),
            "4" => interpreter::demo(),
            "5" => newtype::demo(),
            "6" => strategy::demo(),
            "7" => visitor::demo(),
            "8" => observer::demo(),
            "9" => abstract_factory::demo(),
            "10" => adapter::demo(),
            "0" => {
                println!("Exiting. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a number from 0 to 10."),
        }
    }
}
