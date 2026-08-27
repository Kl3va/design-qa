
#[derive(Clone)]
pub struct TargetUrl(String);

impl TargetUrl {

 pub fn parse(s:String) -> Result<Self, String> {
  let trimmed = s.trim();

  if trimmed.is_empty() {
   return Err("target_url can't be empty".to_string());
  };

  let parsed = url::Url::parse(trimmed).map_err(|_| format!("{} is not a valid url", trimmed))?;

  if parsed.scheme() != "http" && parsed.scheme() != "https" {
    return Err(format!("'{}' must use http/https but got '{}'",trimmed, parsed.scheme()));
  };

  Ok(Self(trimmed.to_string()))
 }
}

impl AsRef<str> for TargetUrl {
 fn as_ref(&self) -> &str {
     &self.0
 }
}


#[cfg(test)]

mod test {
 use super::TargetUrl;

  #[test]
    fn url_is_empty() {
        assert!(TargetUrl::parse("".to_string()).is_err());
    }

    #[test]
    fn url_is_not_a_url() {
        assert!(TargetUrl::parse("not a url".to_string()).is_err());
    }

    #[test]
    fn url_wrong_scheme() {
        assert!(TargetUrl::parse("ftp://example.com".to_string()).is_err());
    }

    #[test]
    fn url_is_valid() {
        let url = TargetUrl::parse("https://example.com/page".to_string()).unwrap();
        assert_eq!(url.as_ref(), "https://example.com/page");
    }
}