// Debug derive allows printing of Token values with {:?}
// Clone derive allows cloning of Password values
#[derive(Debug, Clone)]
pub struct Password<'a>
{
    password:&'a str,
    size:usize,
}

// construct on Password
impl<'a> Password<'a>
{
    // new constructor
    pub fn new(password:&'a str) -> Self
    {
        // return new Password instance
        Self
        {
            // clone password string
            password: Self::clean(&password),
            // store password size
            size: password.len(),
        }
    }

    // clean
    fn clean(password:&str) -> &str
    {
        // return trimmed password
        password.trim()
    }

    // size getter
    pub fn size(&self) -> usize
    {
        // return password size
        self.size
    }

    // password getter
    pub fn password(&self) -> &str
    {
        // return password reference
        self.password
    }
}
