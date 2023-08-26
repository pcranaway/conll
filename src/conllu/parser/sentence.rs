use crate::conllu::treebank::{Upos, Word};
use anyhow::{anyhow, Result};
use regex::Regex;

/// Parses a word line, extracting all fields, validating them, and converting them to their
/// correct form.
// TODO: Add context including line number for debugging
pub fn parse_word(regex: &Regex, input: &str) -> Result<Word> {
    let mut split = input.split("\t");

    // read all fields

    let id = split.next().unwrap();
    let form = split.next().unwrap();
    let lemma = split.next().unwrap();
    let upos = split.next().unwrap();
    let xpos = split.next().unwrap();
    let feats = split.next().unwrap();
    let head = split.next().unwrap();
    let deprel = split.next().unwrap();
    let deps = split.next().unwrap();
    let misc = split.next().unwrap();

    // validate id
    if !is_valid_id(&regex, id) {
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
        if is_valid_id(&regex, head) {
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
pub fn is_valid_id(regex: &Regex, input: &str) -> bool {
    regex.is_match(input)
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

///// /// Parses a feats field value into a Vector<(&str, &str)>
///// pub fn parse_feats(input: &str) -> Result<Vector<&str, &str>> {
/////
///// }

#[cfg(test)]
mod tests {
    use crate::conllu::parser::make_id_regex;

    use super::*;

    #[test]
    fn test_valid_id() {
        let regex = make_id_regex();

        assert!(is_valid_id(&regex, "3"));
        assert!(is_valid_id(&regex, "34"));
        assert!(is_valid_id(&regex, "34-23"));
        assert!(is_valid_id(&regex, "4-3"));

        assert!(!is_valid_id(&regex, "aaa"));
        assert!(!is_valid_id(&regex, "-34"));
        assert!(!is_valid_id(&regex, "34-"));
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
