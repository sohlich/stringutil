mod kebab;
mod snake;

mod kebab_test;

pub use kebab::KebabString;
use regex::Regex;
pub use snake::SnakeString;

use crate::shared::TokenizableString;

pub struct SeparatedString {
    separator: String,
    tokens: Vec<String>,
}

impl TokenizableString for SeparatedString {
    fn load_from_string(&mut self, input: &String) {
        self.tokens = input
            .split(self.separator.as_str())
            .map(|w| w.to_string())
            .collect();
    }

    fn load_tokens(&mut self, tkns: &[String]) {
        self.tokens = tkns.to_vec()
    }

    fn as_tokens(&self) -> &[String] {
        self.tokens.as_slice()
    }

    fn to_string(&self) -> String {
        self.tokens.join(self.separator.as_str())
    }

    fn is_codec(&self, word: &str) -> bool {
        let exp = format!("^[a-z0-9]+(?:{}[a-z0-9]+)*$", self.separator);
        let r = Regex::new(&exp).unwrap();
        return r.is_match(word);
    }

    fn codec_name(&self) -> String {
        return "separated".to_string();
    }
}
