mod propositional_logic;

use std::io;
use propositional_logic::Argument;

fn main() {
    let mut line = String::new();
    let mut argument = String::new();

    loop {

        line.clear();

        println!(" > \r");
        io::stdin().read_line(&mut line);

        if line == "STOP\n" {
            break;
        }

        argument += line.as_str();
        argument += "\n";
        
    }

    let argument = Argument::parse_argument(argument.as_str());
    println!("\n\n-----------------------------------\n{}", argument);

    argument.check_argument();
}
