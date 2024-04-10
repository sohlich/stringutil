use crate::shared::TokenizableString;

use super::camel;
use camel::CamelString;

pub struct PascalString {
    _impl: CamelString,
}

impl PascalString {
    pub fn new() -> Self{
        return PascalString{
            _impl: CamelString::new()
        }
    }
}

impl TokenizableString for PascalString {
    fn load_from_string(&mut self, input: &String) {
        self._impl.load_from_string(input);
    }

    fn load_tokens(&mut self, tkns: &[String]) {
       self._impl.load_tokens(tkns);
    }

    fn as_tokens(&self) -> &[String] {
      return  self._impl.as_tokens();
    }

    fn to_string(&self) -> String {
        let out = self._impl.to_string();
        let first = &out[0..1].to_uppercase();
        let rest = if out.len()  < 2 {""} else {&out[1..]};
        return format!("{}{}",first,rest);
    }
    
    fn is_codec(&self,word:&str) -> bool {
        return self._impl.is_codec(word);
    }
    
    fn codec_name(&self) -> String {
        return "pascal".to_string();
    }
}