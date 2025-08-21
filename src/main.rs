//! # alhamd
//!
//! `alhamd` is short form for 'Alhamdulillaah' (meaning, Praise be to God).
//! This crate provides translation of 'Alhamdulillaah (الحمد لله)' in different languages.
//!
//!
//! Run with
//!
//! ```not_rust
//! cargo run french
//! ```
//! OR
//!
//! ```not_rust
//! cargo run fr
//! ```

use std::env;

fn main() {
    println!("Bismillaah (In the name of God)! Alhamdulillaah (Praise be to God)!");
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let lang = &args[1];
        match lang.as_str().to_lowercase().as_str() {
            "ar" | "arabic" => println!("Alhamdulillaah in '{}' language: الحمد لله", lang),
            "ta" | "tamil" => println!("Alhamdulillaah in '{}' language: எல்லாப் புகழும் இறைவனுக்கே", lang),
            "en" | "english" => println!("Alhamdulillaah in '{}' language: Praise be to God", lang),
            "fr" | "french" => println!("Alhamdulillaah in '{}' language: Dieu soit loué", lang),
            _ => println!("Oh, oh! No translation found for the language: {}\n We'll add in future, as soon as possible.", lang),
        }
    } else {
        println!(
            "\nYou must provide language code like 'en' or 'English' or 'fr' or 'French', etc."
        );
        println!("Try running like: ");
        println!("$ cargo alhamd fr");
        println!("OR");
        println!("$ cargo alhamd french");
    }
}
