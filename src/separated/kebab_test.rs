#[cfg(test)]
mod tests {
    use super::super::KebabString;
    use crate::shared::TokenizableString;
    
    #[test]
    fn should_detect_kebab() {
        let instance = KebabString::new();
        let is_codec = instance.is_codec("is-kebab");
        assert!(is_codec);
    }
}
