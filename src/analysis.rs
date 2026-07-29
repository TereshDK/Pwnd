use crate::password::Password;

// store analysis results of a password
pub struct Info
{
    // contains uppercase letters
    pub uppercase: bool,
    // contains lowercase letters
    pub lowercase: bool,
    // contains digits
    pub digits: bool,
    // contains special characters
    pub special_chars: bool,
}

// construct on Analysis
impl Info
{
    // new constructor
    pub fn new(password:&Password) -> Self
    {
        // return new Info instance
        Self
        {
            // contains bool if contains uppercase letters
            uppercase: Self::contains_uppercase(password),
            // contains bool if contains lowercase letters
            lowercase: Self::contains_lowercase(password),
            // contains bool if contains digits
            digits: Self::contains_digits(password),
            // contains bool if contains special characters
            special_chars: Self::contains_special_chars(password),
        }
    }

    // check if password contains uppercase letters
    fn contains_uppercase(password:&Password) -> bool
    {
        // iterate over password characters
        for ch in password.password().char_indices()
        {
            // check if character is uppercase
            if ch.1.is_uppercase()
            {
                // return true if character is uppercase
                return true;
            }
        }
        // return false if no uppercase letters found
        false
    }

    // check if password contains lowercase letters
    fn contains_lowercase(password:&Password) -> bool
    {
        // iterate over password characters
        for ch in password.password().char_indices()
        {
            // check if character is lowercase
            if ch.1.is_lowercase()
            {
                // return true if character is lowercase
                return true;
            }
        }
        // return false if no lowercase letters found
        false
    }

    // check if password contains digits
    fn contains_digits(password:&Password) -> bool
    {
        // iterate over password characters
        for ch in password.password().char_indices()
        {
            // check if character is a digit
            if ch.1.is_digit(10)
            {
                // return true if character is a digit
                return true;
            }
        }
        // return false if no digits found
        false
    }

    // check if password contains special characters
    fn contains_special_chars(password:&Password) -> bool
    {
        // iterate over password characters
        for ch in password.password().char_indices()
        {
            // check if character is a special character
            if ch.1.is_ascii_punctuation()
            {
                // return true if character is a special character
                return true;
            }
        }
        // return false if no special characters found
        false
    }
}
