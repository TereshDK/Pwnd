// import specific traits / functions from io
use std::io::{Write, stdin, stdout};
// import colored crate for colored output
use colored::Colorize;

// import password
mod password;
use crate::password::Password;
// import analysis
mod analysis;
use crate::analysis::Info;
// import score
mod score;

fn main()
{
    // prompt user to enter password
    print!("{} ", "Enter password: ".bright_blue());
    // flush stdout to ensure prompt is displayed before input
    stdout().flush().expect("[ERROR] failed to flush stdout");
    // call read_line to get user input
    let input:String = read_line();
    let password:Password = Password::new(input.as_ref());
    // print password
    print!("{} {} {}\n",
        "You entered:".bright_green(),
        password.password(),
        format!("{} {}", "of length".bright_green(), password.size()));
    println!();

    let info:Info = Info::new(&password);

    println!("{}", "Info:".bright_blue());
    println!("  {} {}", "- uppercase:".bright_green(), info.uppercase);
    println!("  {} {}", "- lowercase:".bright_green(), info.lowercase);
    println!("  {} {}", "- digits:".bright_green(), info.digits);
    println!("  {} {}", "- special chars:".bright_green(), info.special_chars);
}

// reads a line from stdin and returns it as a String
fn read_line() -> String
{
    // read line from stdin
    let mut line:String = String::new();
    stdin().read_line(&mut line).expect("[ERROR] failed to read line");
    // return line
    line
}
