mod new_pass;
mod greetings;

use std::io;
use new_pass::new_password;
use greetings::greeting;

fn main() {
    greeting();
    // TODO: Implement autenticating system with database
    loop { 
        println!("Press:\n1 - Generate new password.\n2 - See saved passwords\n3 - Finish Program");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let response: u8 = input.trim().parse().expect("Please type 1, 2 or 3");
        // Generating new password
        if response == 1 {
            loop {
                let _password = new_password(); // Pass generated
                println!("1 - Generate new password\n2 - Save password\n3 - Finish program");
                let mut response = String::new();
                io::stdin()
                    .read_line(&mut response)
                    .expect("Failed to read line");
                let response: u8 = response.trim().parse().expect("Please type a number");
                match response {
                    1 => continue,
                    2 => panic!("Sorry, not yet implemented"),// TODO: Redirecting to saving pass on database
                    3 => std::process::exit(0),
                    _ => println!("\n{} is not valid, please type one of the options.\nGenerating new password.", response),
                }
            }
        }
        else if response == 2 {
            panic!("Not yet implemented");// TODO: Redirect to interaction with database
        }
        else if response == 3 {
            std::process::exit(0);
        }
        else {
            println!("Please type one of the options available");
        }
    }
}

