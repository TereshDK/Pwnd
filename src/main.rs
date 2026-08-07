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
// import pwned
mod pwned;
use crate::pwned::Pwned;

#[tokio::main]
async fn main()
{
    // prompt user to enter password
    print!("{} ", "Enter password:".bright_blue());
    // flush stdout to ensure prompt is displayed before input
    stdout().flush().expect("[ERROR] failed to flush stdout");
    // call read_line to get user input
    let input:String = read_line();
    let password:Password = Password::new(input.as_ref());
    // print password
    print!("{} {} {}\n",
        "You entered:".bright_green(),
        password.get_password(),
        format!("{} {}", "of length".bright_green(), password.get_size()));
    println!();
    // store password info
    let info:Info = Info::new(&password);
    // store password score info
    let score:Score = Score::new(&password, &info);
    // print score report
    report::pretty_score_report(&score);
    println!();
    // print info report
    report::pretty_info_report(&score, &info);
    println!();

    // HIBP API
    print!("{} ", "Checking HaveIBeenPwned database".bright_yellow());
    stdout().flush().expect("[ERROR] failed to flush stdout");

    // initialize Pwned API client
    match Pwned::new()
    {
        // successfully initilizaed client
        Ok(pwned) =>
        {
            // asynchronously check if the password has been breached
            match pwned.check(&password).await
            {
                // status is ok
                Ok(0) =>
                {
                    // print that password was not found in data breaches
                    println!("\n{}", "✓ Password not found in known data breaches".bright_green());
                }
                // password breached
                Ok(count) =>
                {
                    // print password known to be breached with total breaches
                    println!("\n{}",
                        format!("[WARNING]: Password found in {} known data breaches!", count)
                            .bright_red()
                            .bold()
                    );
                }
                // request error
                Err(e) =>
                {
                    // print request failwith error
                    eprintln!("\n{}", format!("[ERROR] Network request failed: {}", e).bright_red());
                }
            }
        }
        // failure to initilize client
        Err(e) =>
        {
            // print error
            eprintln!("\n{}", format!("[ERROR] Failed to initialize Pwned client: {}", e).bright_red());
        }
    }
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
