#[derive(Clone)]
pub struct FigmaNodeId(String);

impl FigmaNodeId {
    pub fn parse(s: String) -> Result<Self, String> {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err("Figma node id cannot be empty!".to_string());
        }

        let parts: Vec<&str> = trimmed.split(':').collect();

        if parts.len() != 2
            || parts[0].is_empty()
            || parts[1].is_empty()
            || !parts[0].chars().all(|c| c.is_ascii_digit())
            || !parts[1].chars().all(|c| c.is_ascii_digit())
        {
            return Err(format!(
                "'{}' is not a valid Figma node id (expected format 'number:number')",
                trimmed
            ));
        }

        Ok(Self(trimmed.to_string()))
    }
}

impl AsRef<str> for FigmaNodeId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::FigmaNodeId;

    #[test]
    fn node_id_is_empty() {
        assert!(FigmaNodeId::parse("".to_string()).is_err());
    }

    #[test]
    fn node_id_wrong_format() {
        assert!(FigmaNodeId::parse("abc".to_string()).is_err());
        assert!(FigmaNodeId::parse("123".to_string()).is_err());
        assert!(FigmaNodeId::parse("123:456:789".to_string()).is_err());
    }

    #[test]
    fn node_id_is_valid() {
        let id = FigmaNodeId::parse("123:456".to_string()).unwrap();
        assert_eq!(id.as_ref(), "123:456");
    }
}