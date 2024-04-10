use crate::shared::TokenizableString;
use super::SeparatedString;

const KEBAB_SEPARATOR : &str = "-";


pub struct KebabString {
    _impl:SeparatedString,
}


impl KebabString {
    pub fn new() -> Self {
        return KebabString{_impl: SeparatedString{
            separator: KEBAB_SEPARATOR.to_string(),
            tokens: vec![]
        }};
    }
}

impl TokenizableString for KebabString {
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
        return "kebab".to_string();
    }
}

