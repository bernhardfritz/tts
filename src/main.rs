use std::{fs::File, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
struct Args {
    text: String,
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let params = &[
        ("client", "tw-ob"),
        ("ie", "UTF-8"),
        ("q", &args.text),
        ("tl", "en"),
    ];
    let qs = serde_urlencoded::to_string(params).unwrap();
    let mut res =
        reqwest::blocking::get(format!("https://translate.google.com/translate_tts?{}", qs))
            .unwrap();
    let mut file = File::create(args.path).unwrap();
    res.copy_to(&mut file).unwrap();
}
