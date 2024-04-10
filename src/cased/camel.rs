use regex::Regex;

use crate::shared;

pub struct CamelString {
    tokens: Vec<String>,
}

impl CamelString {
    pub fn new() -> Self {
        CamelString { tokens: vec![] }
    }
}

impl shared::TokenizableString for CamelString {
    fn load_from_string(&mut self, input: &String) {
        let mut start = 0;
        let mut idx = 0;
        for (_, c) in input.char_indices() {
            if c.is_ascii_uppercase() {
                let subs = &input[start..idx];
                if subs.len() > 0 {
                    self.tokens.push(subs.to_string().to_lowercase());
                }
                start = idx;
            }
            idx += 1;
        }
        // we need to push the rest of
        // string
        self.tokens.push(input[start..].to_string().to_lowercase());
    }

    fn as_tokens(&self) -> &[String] {
        return self.tokens.as_slice();
    }

    fn load_tokens(&mut self, tkns: &[String]) {
        self.tokens = tkns.to_vec();
    }

    fn to_string(&self) -> String {
        let mut out = "".to_string();
        for (idx, tkn) in self.tokens.iter().enumerate() {
            let start_ltr = if idx == 0 {
                tkn[..1].to_ascii_lowercase()
            } else {
                tkn[..1].to_ascii_uppercase()
            };
            out = format!("{}{}{}", out, start_ltr, &tkn[1..])
        }
        return out;
    }
    
    fn is_codec(&self,word:&str) -> bool {
       let r = Regex::new("([A-Z][a-z0-9]+)+").unwrap();
       return r.is_match(word);
    }

    fn codec_name(&self) -> String {
        return "camel".to_string();
    }
}
