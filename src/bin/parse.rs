use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use argh::FromArgs;
use conll::conllu;

#[derive(FromArgs)]
/// Parses a Treebank from a CoNLL-U file.
struct Args {
    /// the path of the .conllu file
    #[argh(positional)]
    file_path: String,

    /// whether it should be silent (not print out the parsed Treebank)
    #[argh(switch, short = 's')]
    silent: bool,
}

fn main() {
    let args: Args = argh::from_env();

    let file = File::open(args.file_path).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let treebank = conllu::parser::parse(lines).unwrap();

    if !args.silent {
        dbg!(treebank);
    }
}
