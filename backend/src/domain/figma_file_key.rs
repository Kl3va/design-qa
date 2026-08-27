#[derive(Clone)]

pub struct FigmaFileKey(String);

impl FigmaFileKey {

 pub fn parse(s:String) -> Result<Self, String> {
  let trimmed = s.trim();

  if trimmed.is_empty() {
   return Err("value cannot be empty".to_string());
  };

  if !trimmed.chars().all(|c| c.is_ascii_alphanumeric()) {
   return Err(format!("{} is not a valid figma key", trimmed));
  };

  if trimmed.chars().count() < 10 {
   return Err(format!("{} is too short to be a valid figma key", trimmed));
  };

  Ok(Self(trimmed.to_string()))
 }
}

impl AsRef<str> for FigmaFileKey {
 fn as_ref(&self) -> &str {
     &self.0
 }
}

#[cfg(test)]

mod test {
 use super::FigmaFileKey;

   #[test]
    fn key_is_empty() {
        assert!(FigmaFileKey::parse("".to_string()).is_err());
    }

    #[test]
    fn key_has_invalid_chars() {
        assert!(FigmaFileKey::parse("abc-123!@#".to_string()).is_err());
    }

    #[test]
    fn key_too_short() {
        assert!(FigmaFileKey::parse("abc123".to_string()).is_err());
    }

    #[test]
    fn key_is_valid() {
        let key = FigmaFileKey::parse("aBcD1234EfGh5678IjKl".to_string()).unwrap();
        assert_eq!(key.as_ref(), "aBcD1234EfGh5678IjKl");
    }
}