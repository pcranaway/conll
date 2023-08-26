/// The whole point of this module is to test and detect the different three kinds of lines
/// specified by CoNLL-U.

#[derive(Debug, PartialEq)]
pub enum Line {
    Comment(String),
    Boundary,
    Word(String),
}

/// Detects what kind of line a given line is ([Line])
pub fn parse_line(input: String) -> Option<Line> {
    if is_comment_line(&input) {
        // TODO: get rid of `#` and actually parse the contents of the comment (don't confuse
        // them with regular programming comments -- they apparently have actual meaning.)
        return Some(Line::Comment(input));
    }

    if is_boundary_line(&input) {
        return Some(Line::Boundary);
    }

    if is_word_line(&input) {
        return Some(Line::Word(input));
    }

    return None;
}

/// Checks if given input is a comment line, starting with a `#`
pub fn is_comment_line(input: &str) -> bool {
    return input.starts_with("#");
}

/// Checks if given input is a boundary line, meaning that it separates sentences from
/// eachother. Those lines are empty.
pub fn is_boundary_line(input: &str) -> bool {
    return input.trim().is_empty();
}

/// Checks if given input is a word line. This is a little hard to check, so we just check if it
/// contains at least one tab character.
pub fn is_word_line(input: &str) -> bool {
    return input.contains("\t");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_WORD_LINE: &str ="2	Mijn	mijn	PRON	VNW|bez|det|stan|vol|1|ev|prenom|zonder|agr	Person=1|Poss=Yes|PronType=Prs	3	nmod:poss	3:nmod:poss	_";

    #[test]
    fn test_comment() {
        assert!(is_comment_line("#a"));
        assert!(is_comment_line("# aaaaaaaaaaaaaaaaaaaa"));

        assert!(!is_comment_line("a"));
    }

    #[test]
    fn test_boundary() {
        assert!(is_boundary_line(""));
        assert!(is_boundary_line(
            "
            "
        ));
    }

    #[test]
    fn test_word_line() {
        assert!(is_word_line(TEST_WORD_LINE));

        assert!(!is_word_line("# a"));
        assert!(!is_word_line("a"));
    }

    #[test]
    fn parse_line_optional() {
        assert!(parse_line("# a".to_string()).is_some());
        assert!(parse_line("".to_string()).is_some());
        assert!(parse_line(TEST_WORD_LINE.to_string()).is_some());

        assert!(parse_line("a".to_string()).is_none());
    }

    #[test]
    fn detect_comment_line() {
        // TODO: fix this test as well

        assert_eq!(
            parse_line("# a".to_string()).unwrap(),
            Line::Comment("# a".to_string())
        );
    }

    #[test]
    fn detect_boundary_line() {
        assert_eq!(parse_line("".to_string()).unwrap(), Line::Boundary);
        assert_eq!(
            parse_line(
                "

                                  "
                .to_string()
            )
            .unwrap(),
            Line::Boundary
        );
    }

    #[test]
    fn detect_word_line() {
        assert_eq!(
            parse_line(TEST_WORD_LINE.to_string()).unwrap(),
            Line::Word(TEST_WORD_LINE.to_string())
        );
    }
}
