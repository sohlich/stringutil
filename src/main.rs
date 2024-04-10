mod cased;
mod separated;
mod shared;

use std::{
    io::{self, IsTerminal, Read, Write},
    process::exit,
};

use cased::{CamelString, PascalString};
use separated::{KebabString, SnakeString};
use shared::TokenizableString;

fn main() {
    let codecs: Vec<Box<dyn TokenizableString>> = vec![
        Box::new(CamelString::new()),
        Box::new(PascalString::new()),
        Box::new(KebabString::new()),
        Box::new(SnakeString::new()),
    ];

    let mut stdargs = std::env::args();

    let mut word = String::new();
    if !io::stdin().is_terminal() {
        if let Err(nok) = io::stdin().read_to_string(&mut word) {
            println!("{}", nok);
            exit(2);
        }
    } else {
        word = std::env::args().nth(2).expect("no word provided")
    }

    let encoder_code = stdargs.nth(1).expect("no encoder provided");

    word = word.replace("\n", "");

    let detected_reader = detect_decoder(codecs, &word);
    let mut reader: Box<dyn TokenizableString>;
    match detected_reader {
        Ok(r) => {
            reader = r;
        }
        Err(msg) => {
            println!("{}", msg);
            exit(2);
        }
    }

    let detected_writer = get_codec(encoder_code.as_str());

    let mut writer: Box<dyn TokenizableString>;
    match detected_writer {
        Ok(w) => {
            writer = w;
        }
        Err(_) => {
            println!("encoder not found for {}", encoder_code);
            exit(2);
        }
    }

    reader.load_from_string(&word);
    writer.load_tokens(reader.as_tokens());
    let w_res = io::stdout().write_all(writer.to_string().as_bytes());
    if let Err(e) = w_res {
        println!("{}", e);
    }
}

fn detect_decoder(
    codecs: Vec<Box<dyn TokenizableString>>,
    word: &str,
) -> Result<Box<dyn TokenizableString>, String> {
    for codec in codecs {
        if codec.is_codec(word) {
            return Ok(codec);
        }
    }
    return Err("no codec detected".to_string());
}

fn get_codec(reader_code: &str) -> Result<Box<dyn TokenizableString>, String> {
    let out: Box<dyn TokenizableString>;
    match reader_code.to_lowercase().as_str() {
        "p" | "pascal" => {
            out = Box::new(PascalString::new());
        }
        "c" | "camel" => {
            out = Box::new(CamelString::new());
        }
        "s" | "snake" => {
            out = Box::new(SnakeString::new());
        }
        "k" | "kebab" => {
            out = Box::new(KebabString::new());
        }
        _ => {
            return Err(format!("{} not defined", reader_code));
        }
    }

    return Ok(out);
}
