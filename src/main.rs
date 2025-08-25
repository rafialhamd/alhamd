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
//! cargo run fra
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
        let mut trans = "";

        match lang.as_str().to_lowercase().as_str() {
            "ar" | "ara" | "arabic" => trans = "الحمد لله",
            "sq" | "sqi" | "albanian" => trans = "Lavdërimi i qoftë Zotit",
            "en" | "eng" | "english" => trans = "Praise be to God",
            "fr" | "fra" | "french" => trans = "Dieu soit loué",
            "ga" | "gle" | "irish" => trans = "Moladh do Dhia",
            "it" | "ita" | "italian" => trans = "Sia lodato Dio",
            "ta" | "tam" | "tamil" => trans = "எல்லாப் புகழும் இறைவனுக்கே",
            _ => println!(
                ">>No translation found for the language: {lang}\n>>We'll add , as soon as possible."
            ),
        }
        if trans != "" {
            println!("Alhamdulillaah in '{}' language: {}", lang, trans);
        }
    } else {
        println!(
            "\nYou must provide language code like 'en' or 'eng' or 'English' or 'fr' or 'fra' or 'French', etc."
        );
        println!("Try running like: ");
        println!("$ cargo alhamd fr");
        println!("OR");
        println!("$ cargo alhamd fra");
        println!("OR");
        println!("$ cargo alhamd french");
    }
}
