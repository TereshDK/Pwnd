use crate::analysis::Info;

// password strength level placement
pub enum Level
{
    Weak = 24,
    Fair = 49,
    Strong = 74,
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
    pub fn new(&self, info:&Info, length:u8) -> Self
    {
        // return new Score instance
        Self
        {
            // password length score
            length: Self::score_length(&self),
            // password uppercase score
            uppercase: Self::score_uppercase(&self),
            // password lowercase score
            lowercase: Self::score_lowercase(&self),
            // password digits score
            digits: Self::score_digits(&self),
            // password special chars score
            special_chars: Self::score_special_chars(&self),
        }
    }

    // calculate score for password length
    fn score_length(&self) -> u8
    {
        todo!()
    }

    // calculate score for password's uppercase letters
    fn score_uppercase(&self) -> u8
    {
        todo!()
    }

    // calculate score for password's lowercase letters
    fn score_lowercase(&self) -> u8
    {
        todo!()
    }

    // calculate score for password's digits
    fn score_digits(&self) -> u8
    {
        todo!()
    }

    // calculate score for password's special characters
    fn score_special_chars(&self) -> u8
    {
        todo!()
    }
}
