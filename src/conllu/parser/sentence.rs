use crate::conllu::treebank::Upos;
use crate::conllu::treebank::Word;
use anyhow::{anyhow, Result};

/// Parses a word line, extracting all fields, validating them, and converting them to their
/// correct form.
// TODO: Add context including line number for debugging
pub fn parse_word(input: &str) -> Result<Word> {
    let split: Vec<&str> = input.split("\t").collect();

    // read all fields
    let id = &split[0];
    let form = &split[1];
    let lemma = &split[2];
    let upos = &split[3];
    let xpos = &split[4];
    let feats = &split[5];
    let head = &split[6];
    let deprel = &split[7];
    let deps = &split[8];
    let misc = &split[9];

    // validate fields
    if !is_valid_id(id) {
        return Err(anyhow!("invalid word ID"));
    }

    let upos = parse_upos(upos)?;

    todo!()
}

/// Checks if a string is a valid word ID, as defined by CoNNL-U.
pub fn is_valid_id(input: &str) -> bool {
    // range, for multiword tokens
    if input.contains("-") {
        let mut split = input.split("-");

        let lower = split.next().unwrap();
        let upper = split.next().unwrap();

        return lower.parse::<u32>().is_ok() && upper.parse::<u32>().is_ok();
    } else {
        return input.parse::<u32>().is_ok();
    }

    // TODO: Support decimal numbers (they're used for empty nodes. (must be between 0 and 1))
}

/// Parses a UPOS field value into its corresponding [Upos] variant.
pub fn parse_upos(input: &str) -> Result<Upos> {
    return match input {
        "ADJ" => Ok(Upos::Adjective),
        "ADP" => Ok(Upos::Adposition),
        "ADV" => Ok(Upos::Adverb),
        "AUX" => Ok(Upos::Auxiliary),
        "CCONJ" => Ok(Upos::CoordinatingConjunction),
        "DET" => Ok(Upos::Determiner),
        "INTJ" => Ok(Upos::Interjection),
        "NOUN" => Ok(Upos::Noun),
        "NUM" => Ok(Upos::Numeral),
        "PART" => Ok(Upos::Particle),
        "PRON" => Ok(Upos::Pronoun),
        "PROPN" => Ok(Upos::ProperNoun),
        "PUNCT" => Ok(Upos::Punctuation),
        "SCONJ" => Ok(Upos::SubordinatingConjunction),
        "SYM" => Ok(Upos::Symbol),
        "VERB" => Ok(Upos::Verb),
        "X" => Ok(Upos::Other),

        _ => Err(anyhow!("invalid UPOS {}", input)),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_id() {
        assert!(is_valid_id("3"));
        assert!(is_valid_id("34"));
        assert!(is_valid_id("34-23"));
        assert!(is_valid_id("4-3"));

        assert!(!is_valid_id("4294967296"));
        assert!(!is_valid_id("4294967296"));
        assert!(!is_valid_id("4294967296-4294967296"));
        assert!(!is_valid_id("4294967295-4294967296"));
        assert!(!is_valid_id("4294967296-4294967295"));

        assert!(!is_valid_id("aaa"));
        assert!(!is_valid_id("-34"));
        assert!(!is_valid_id("34-"));
    }

    #[test]
    fn test_upos() {
        assert_eq!(parse_upos("CCONJ").unwrap(), Upos::CoordinatingConjunction);
        assert!(parse_upos("cconj").is_err());
    }
}
