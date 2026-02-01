#[derive(Debug, Clone)]
pub struct Diff(String);

impl Diff {
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn new_and_as_str_roundtrip(s in "(?s).{0,1000}") {
            let diff = Diff::new(&s);
            prop_assert_eq!(diff.as_str(), s);
        }
    }
}
