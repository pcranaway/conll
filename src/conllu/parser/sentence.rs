use crate::conllu::treebank::Word;

/// Parses a word line, extracting all fields, validating them, and converting them to their
/// correct form.
pub fn parse_word(input: &str) -> Option<Word> {
    let split: Vec<&str> = input.split("\t").collect();

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
}
