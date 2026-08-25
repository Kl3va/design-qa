#[derive(Debug)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn parse(s: String) -> Result<Self, String> {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err("Project name cannot be empty!".to_string());
        }

        if trimmed.chars().count() > 50 {
            return Err(format!(
                "Project {} exceeds the character length(50)",
                trimmed
            ));
        }

        Ok(Self(trimmed.to_string()))
    }
}

impl AsRef<str> for ProjectName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]

mod tests {
    use super::ProjectName;

    #[test]
    fn name_is_empty() {
        let name = ProjectName::parse("".to_string());

        assert!(name.is_err());
    }

    #[test]
    fn name_is_valid() {
        let name = ProjectName::parse("My project".to_string());
        assert!(name.is_ok());
    }

    #[test]
    fn name_greater_than_50_is_invalid() {
        let name = "a".repeat(51);
        let final_name = ProjectName::parse(name);

        assert!(final_name.is_err());
    }

    #[test]
    fn name_returned_is_equal() {
        let name = ProjectName::parse("Large project".to_string()).unwrap();

        assert_eq!(name.as_ref(), "Large project");
    }
}
