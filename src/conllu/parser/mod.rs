use self::line::parse_line;

use super::treebank::{Sentence, Treebank};

pub mod line;
pub mod sentence;

pub fn parse(lines: Vec<String>) {
    // parsing state
    let mut treebank = Treebank::default();
    let mut sentence = Sentence::default();

    // parse lines one by one
    for line in lines {
        let line = parse_line(line);

        if line.is_none() {
            continue;
        }

        match line.unwrap() {
            line::Line::Comment(_) => {}
            line::Line::Boundary => {
                // a boundary line marks the end of a sentence so we finish the sentence, put it
                // in the treebank, and start a new one.

                treebank.sentences.push(sentence);

                sentence = Sentence::default();
            }
            line::Line::Word(line) => {
                let word = sentence::parse_word(&line).unwrap();

                sentence.words.push(word);
            }
        }
    }
}
