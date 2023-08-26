use crate::conllu::treebank::{Upos, Word};
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

    // validate id
    if !is_valid_id(id) {
        return Err(anyhow!("invalid word ID {}", id));
    }

    // create word struct
    let mut word = Word {
        id: id.to_string(),

        ..Default::default()
    };

    // validate form
    if !is_unspecified(form) {
        word.form = Some(form.to_string());
    }

    // validate lemma
    if !is_unspecified(lemma) {
        word.lemma = Some(lemma.to_string());
    }

    // validate upos
    if !is_unspecified(upos) {
        word.upos = Some(parse_upos(upos)?);
    }

    // validate xpos
    if !is_unspecified(xpos) {
        word.xpos = Some(xpos.to_string());
    }

    // validate feats
    if !is_unspecified(feats) {
        // TODO: parse feats
        word.feats = Some(Vec::default());
    }

    // validate head
    if !is_unspecified(head) {
        if is_valid_id(head) {
            word.head = Some(head.to_string());
        } else {
            return Err(anyhow!("invalid head"));
        }
    }

    // validate deprel
    if !is_unspecified(deprel) {
        // TODO: parse deprel
        word.deprel = Some(deprel.to_string());
    }

    // validate deps
    if !is_unspecified(deps) {
        // TODO: parse deps
        word.deps = Some(deps.to_string());
    }

    // validate misc
    if !is_unspecified(misc) {
        word.misc = Some(misc.to_string());
    }

    Ok(word)
}

/// Checks if the given field value is unspecified (denoted by an underscore.)
pub fn is_unspecified(input: &str) -> bool {
    return input.trim() == "_";
}

/// Checks if a string is a valid word ID, as defined by CoNNL-U.
pub fn is_valid_id(input: &str) -> bool {
    if input.contains("-") {
        // range, for multiword tokens
        let mut split = input.split("-");

        let lower = split.next().unwrap();
        let upper = split.next().unwrap();

        return lower.parse::<u32>().is_ok() && upper.parse::<u32>().is_ok();
    } else if input.contains(".") {
        // used for empty nodes?
        let mut split = input.split(".");

        let lower = split.next().unwrap();
        let upper = split.next().unwrap();

        return lower.parse::<u32>().is_ok() && upper.parse::<u32>().is_ok();
    } else {
        // regular id
        return input.parse::<u32>().is_ok();
    }
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

    #[test]
    fn test_unspecified() {
        assert!(!is_unspecified("CCONJ"));
        assert!(!is_unspecified(""));
        assert!(!is_unspecified(" "));

        // not sure if we have to trim it
        assert!(is_unspecified("_ "));
        assert!(is_unspecified(" _ "));
        assert!(is_unspecified(" _"));

        assert!(is_unspecified("_"));
    }
}
