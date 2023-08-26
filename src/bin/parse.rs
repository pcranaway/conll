use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use conll::conllu;

fn main() {
    let file_path = env::args().nth(1).expect("expected file path");

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let treebank = conllu::parser::parse(lines).unwrap();

    dbg!(treebank);
}
