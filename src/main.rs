use structopt::StructOpt;
use std::io::{self, Read};
use syntect::parsing::SyntaxSet;
use syntect::html::{highlighted_html_for_string};
use syntect::highlighting::{ThemeSet};

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "l", long = "lang")]
    lang: String,

    #[structopt(short = "t", long = "theme", default_value = "base16-ocean.dark")]
    theme: String
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let args = Cli::from_args();
    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = ss.find_syntax_by_token(&args.lang).unwrap();
    let ts = ThemeSet::load_defaults();
    let output = highlighted_html_for_string(
        &buffer,
        &ss,
        &syntax,
        &ts.themes[&args.theme]
    );

    println!("{}", output);
}
