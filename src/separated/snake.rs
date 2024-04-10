use crate::shared::TokenizableString;
use super::SeparatedString;

const SNAKE_SEPARATOR : &str = "_";

pub struct SnakeString {
    _impl:SeparatedString,
}

impl SnakeString {
    pub fn new() -> Self {
        return SnakeString{_impl: SeparatedString{
            separator: SNAKE_SEPARATOR.to_string(),
            tokens: vec![]
        }};
    }
}

impl TokenizableString for SnakeString {
    fn load_from_string(&mut self, input: &String) {
        self._impl.load_from_string(input);
    }

    fn load_tokens(&mut self, tkns: &[String]) {
        self._impl.load_tokens(tkns);
    }

    fn as_tokens(&self) -> &[String] {
       return self._impl.as_tokens();
    }

    fn to_string(&self) -> String {
        return self._impl.to_string();
    }
    
    fn is_codec(&self,word:&str) -> bool {
        return self._impl.is_codec(word);
    }

    fn codec_name(&self) -> String {
        return "snake".to_string();
    }
}

