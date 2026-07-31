// use Duration
use std::time::Duration;
// use Digest, Sha1
use sha1::{Digest, Sha1};
// use reqwest Client
use reqwest::Client;
// use reqwest Error
use reqwest::Error;

// use Password
use crate::password::Password;

// private Hash struct
// with lifetime for string slices
struct Hash<'a>
{
    // first 5 characters of SHA-1 hash
    prefix:&'a str,
    // remaining 35 characters of SHA-1 hash
    suffix:&'a str,
}

#[allow(dead_code)]
// Pwned struct
pub struct Pwned
{
    // reusuable HTTP client instance
    client:Client,
    // base URL for k-anonymity endpoint
    base_url:String,
    // user-agent required by HIBP API
    user_agent:String,
    // timeout duration for outbound HTTP requests
    timeout:Duration,
}

// construct on Pwned
impl Pwned
{
    // new constructor
    pub fn new() -> Result<Self, Error>
    {
        // set base url to HIBP k-anonymity API URL
        let base_url:String = String::from("https://api.pwnedpasswords.com/range/");
        // application identifier for HIBP request header
        let user_agent:String = String::from("Password-Checker/0.1.0");
        // define timeout duration
        let timeout = Duration::from_secs(5);
        // build reqwest client
        // ? propagtes initilization errors to caller
        let client = Client::builder()
            .user_agent(&user_agent)
            .timeout(timeout)
            .build()?;
        // return initialized instance as a Result
        Ok(Self
        {
            client,
            base_url,
            user_agent,
            timeout,
        })
    }

    // hashes a password using SHA-1
    fn hash(password:&Password<'_>) -> String
    {
        // create new SHA-1 hasher
        let mut hasher = Sha1::new();
        // feed password bytes into hasher
        hasher.update(password.get_password().as_bytes());
        // return uppercase 20 byte hexadecimal hash
        format!("{:X}", hasher.finalize())
    }

    // split 40 character SHA-1 hex into prefix and suffix
    fn split_hash<'a>(hash:&'a str) -> Hash<'a>
    {
        // return a Hash instance
        Hash
        {
            // slices first 5 characters, 0..4
            prefix: &hash[..5],
            // slices remaining 35 characters, 5..40
            suffix: &hash[5..]
        }
    }

    // send HTTP GET request to HIBP
    // asynchronously fetch and return plain-text response body
    async fn request(&self, prefix:&str) -> Result<String, Error>
    {
        // construct full endpoint URL
        let url:String = format!("{}{}", self.base_url, prefix);
        // execute GET request asynchronously and await response headers
        let response = self.client.get(url).send().await?;
        // asynchronously decode and extract full response body as text
        let body:String = response.text().await?;
        // return body as a Result
        Ok(body)
    }

    // searches HIBP API response body for matching 35 character suffixes
    fn search_response(&self, body:&str, suffix:&str) -> u32
    {
        // iterate over each line, handling \n and \r\n with .lines
        for line in body.lines()
        {
            // split each line at colon into a tuple
            if let Some((line_suffix, count)) = line.split_once(":")
            {
                // check if returned suffix matches target password suffix
                if line_suffix == suffix
                {
                    // return count parsed as u32, otherwise default to 0 on failure
                    return count.parse::<u32>().unwrap_or(0);
                }
            }
        }
        // no matching suffix found
        return 0;
    }

    // to check if password has been compromised
    pub async fn check<'a>(&self, password:&Password<'_>) -> Result<u32, Error>
    {
        // compute SHA-1 hash of password
        let full_hash:String = Self::hash(password);
        // split hash into 5-char prefix and 35-char suffix
        let hash_parts:Hash = Self::split_hash(&full_hash);
        // fetch matching hash range using prefix
        let body:String = self.request(hash_parts.prefix).await?;
        // search suffix list for match
        let count:u32 = self.search_response(&body, hash_parts.suffix);
        // return total breach count as a Result
        Ok(count)
    }
}
