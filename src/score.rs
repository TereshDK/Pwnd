// use Info
use crate::analysis::Info;
// use Password
use crate::password::Password;

// Debug derive allows printing of Token values with {:?}
#[derive(Debug)]
// password strength level placement
pub enum Level
{
    // weak password
    Weak = 24,
    // fair password
    Fair = 49,
    // strong password
    Strong = 74,
    // very strong password
    VeryStrong = 100,
}

// password strength score
pub struct Score
{
    // password length score
    length:u8,
    // password uppercase score
    uppercase:u8,
    // password lowercase score
    lowercase:u8,
    // password digits score
    digits:u8,
    // password special chars score
    special_chars:u8,
}

// construct on Score
impl Score
{
    // new construct
    pub fn new(password:&Password, info:&Info) -> Self
    {
        // return new Score instance
        Self
        {
            // password length score
            length: Self::score_length(password),
            // password uppercase score
            uppercase: Self::score_uppercase(info),
            // password lowercase score
            lowercase: Self::score_lowercase(info),
            // password digits score
            digits: Self::score_digits(info),
            // password special chars score
            special_chars: Self::score_special_chars(info),
        }
    }

    // calculate score for password length
    fn score_length(password:&Password) -> u8
    {
        // match password size and assign score
        match password.size()
        {
            // between 0-7 is 0 points
            0..=7 => 0,
            // between 8-11 is 10 points
            8..=11 => 10,
            // between 12-15 is 20 points
            12..=15 => 20,
            // between 16-19 is 30 points
            16..=19 => 30,
            // between 20+ is 40 points
            _ => 40,
        }
    }

    // calculate score for password's uppercase letters
    fn score_uppercase(info:&Info) -> u8
    {
        // contains at least one uppercase character
        if info.has_uppercase()
        {
            // return 15 points
            return 15;
        }
        // no uppercase character
        // return 0 points
        0
    }

    // calculate score for password's lowercase letters
    fn score_lowercase(info:&Info) -> u8
    {
        // contains at least one lowercase character
        if info.has_lowercase()
        {
            // return 15 points
            return 15;
        }
        // no lowercase character
        // return 0 points
        0
    }

    // calculate score for password's digits
    fn score_digits(info:&Info) -> u8
    {
        // contains at least one digit
        if info.has_digits()
        {
            // return 15 points
            return 15;
        }
        // no digit
        // return 0 points
        0
    }

    // calculate score for password's special characters
    fn score_special_chars(info:&Info) -> u8
    {
        // contains at least one special character
        if info.has_special_chars()
        {
            // return 15 points
            return 15;
        }
        // no special character
        // return 0 points
        0
    }

    // get length score
    pub fn get_length_score(&self) -> u8
    {
        // return length
        self.length
    }

    // get uppercase score
    pub fn get_uppercase_score(&self) -> u8
    {
        // return uppercase
        self.uppercase
    }

    // get lowercase score
    pub fn get_lowercase_score(&self) -> u8
    {
        // return lowercase
        self.lowercase
    }

    // get digits score
    pub fn get_digits_score(&self) -> u8
    {
        // return digits
        self.digits
    }

    // get special chars score
    pub fn get_special_chars_score(&self) -> u8
    {
        // return special chars
        self.special_chars
    }

    // get total score
    pub fn get_total_score(&self) -> u8
    {
        self.length + self.uppercase +
        self.lowercase + self.digits +
        self.special_chars
    }

    // get password strength level
    pub fn get_strength_level(&self) -> Level
    {
        match self.get_total_score()
        {
            0..=24 => Level::Weak,
            25..=49 => Level::Fair,
            50..=74 => Level::Strong,
            75..=100 => Level::VeryStrong,
            _ => Level::VeryStrong,
        }
    }
}
