use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Error;

// https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
pub fn get_token(token_with_bearer: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Bearer (?P<token>.*)$").unwrap();
    }
    let matches = RE
        .captures(token_with_bearer)
        .and_then(|cap| cap.name("token").map(|token| token.as_str()));
    matches
}

pub async fn fetch_jwks(url: &str) -> Result<String, Error> {
    let content = reqwest::get(url).await?.text().await?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_return_token() {
        let token = get_token("Bearer abcd1234");
        assert_eq!(token.unwrap(), "abcd1234");
    }
    #[test]
    fn test_should_return_none() {
        let token = get_token("NotBearer abcd1234");
        assert_eq!(token.is_none(), true);
    }
}
