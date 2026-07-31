// import colored crate for colored output
use colored::{ColoredString, Colorize};

// use Score, Level
use crate::score::Score;
// use Info
use crate::analysis::Info;

#[allow(dead_code)]
// print score in formatted output
pub fn score_report(score:&Score)
{
    // print score header
    println!("{}", "Score:".bright_blue());
    // print length score
    println!("  {} {}", "- length:".bright_green(), score.get_length_score());
    // print uppercase score
    println!("  {} {}", "- uppercase:".bright_green(), score.get_uppercase_score());
    // print lowercase score
    println!("  {} {}", "- lowercase:".bright_green(), score.get_lowercase_score());
    // print digits score
    println!("  {} {}", "- digits:".bright_green(), score.get_digits_score());
    // print special chars score
    println!("  {} {}", "- special chars:".bright_green(), score.get_special_chars_score());
}

// print score in pretty formatted output
pub fn pretty_score_report(score:&Score)
{
    // print score header
    println!("{}", "Score:".bright_blue());
    // print length score
    pretty_score("length", score.get_length_score());
    // print lowercase score
    pretty_score("lowercase", score.get_lowercase_score());
    // print uppercase score
    pretty_score("uppercase", score.get_uppercase_score());
    // print digits score
    pretty_score("digits", score.get_digits_score());
    // print special chars score
    pretty_score("special chars", score.get_special_chars_score());
}

// pretty print score
fn pretty_score(label:&str, score:u8)
{
    // match label to print appropriately
    match label.to_lowercase().as_str()
    {
        // print length score
        "length" => println!("{} ............ {}{}", label.bright_magenta(), "+".bright_red(), format!("{}", score).bright_green()),
        // print lowercase score
        "lowercase" => println!("{} ......... {}{}", label.bright_magenta(), "+".bright_red(), format!("{}", score).bright_green()),
        // print uppercase score
        "uppercase" => println!("{} ......... {}{}", label.bright_magenta(), "+".bright_red(), format!("{}", score).bright_green()),
        // print digits score
        "digits" => println!("{} ............ {}{}", label.bright_magenta(), "+".bright_red(), format!("{}", score).bright_green()),
        // print special chars score
        "special chars" => println!("{} ..... {}{}", label.bright_magenta(), "+".bright_red(), format!("{}", score).bright_green()),
        // if none, do nothing
        _ => {}
    }
}

// print info in pretty formatted output
pub fn pretty_info_report(score:&Score, info:&Info)
{
    // print password analysis header
    println!("{}", "Password Analysis:".bright_blue());
    // print uppercase score
    pretty_info("Contains uppercase", info.has_uppercase());
    // print lowercase score
    pretty_info("Contains lowercase", info.has_lowercase());
    // print digits score
    pretty_info("Contains digits", info.has_digits());
    // print special chars score
    pretty_info("Contains special characters\n", info.has_special_chars());

    // print total password score
    println!("{} {}/100\n", "Score:".bright_green(), format!("{}", score.get_total_score()).red());

    // print strength header
    println!("{}", "Strength:".bright_blue());
    // print strength
    println!("{}", format!("{:?}", score.get_strength_level()).red());
}

// pretty print info
fn pretty_info(description:&str, has:bool)
{
    // checkmark in bright green
    let checkmark:ColoredString = "✓".bright_green();
    // crossmark in bright red
    let crossmark:ColoredString = "✗".bright_red();
    // if has that required character
    if has
    {
        // print checkmark and description
        println!("{} {}", checkmark, description.purple());
    }
    // if does not have that required character
    else
    {
        // print crossmark and description
        println!("{} {}", crossmark, description.purple());
    }
}
