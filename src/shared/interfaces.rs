pub trait TokenizableString {
    fn load_from_string(&mut self, input: &String);
    fn load_tokens(&mut self, tkns: &[String]);
    fn as_tokens(&self) -> &[String];
    fn to_string(&self) -> String;
    fn is_codec(&self,word:&str) -> bool;
    fn codec_name(&self) -> String;
}
