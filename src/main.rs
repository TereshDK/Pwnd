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
use crate::score::Score;
// import report
mod report;

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
    // store password info
    let info:Info = Info::new(&password);
    // store password score info
    let score:Score = Score::new(&password, &info);
    // print score report
    report::pretty_score_report(&score);
    // print new line
    println!();
    // print info report
    report::pretty_info_report(&score, &info);
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
