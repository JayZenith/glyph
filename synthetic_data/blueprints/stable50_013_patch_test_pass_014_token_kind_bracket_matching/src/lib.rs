#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Number,
    Open(Bracket),
    Close(Bracket),
    Separator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bracket {
    Paren,
    Square,
    Curly,
}

pub fn closes(open: TokenKind, close: TokenKind) -> bool {
    match (open, close) {
        (TokenKind::Open(Bracket::Paren), TokenKind::Close(_)) => true,
        (TokenKind::Open(Bracket::Square), TokenKind::Close(Bracket::Square)) => true,
        (TokenKind::Open(Bracket::Curly), TokenKind::Close(Bracket::Curly)) => true,
        _ => false,
    }
}

pub fn classify_pair(open: TokenKind, close: TokenKind) -> &'static str {
    match (open, close) {
        (TokenKind::Open(_), TokenKind::Close(_)) if closes(open, close) => "matched",
        (TokenKind::Open(_), TokenKind::Close(_)) => "mismatched",
        _ => "not brackets",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_brackets_are_detected() {
        assert!(closes(TokenKind::Open(Bracket::Paren), TokenKind::Close(Bracket::Paren)));
        assert!(closes(TokenKind::Open(Bracket::Square), TokenKind::Close(Bracket::Square)));
        assert!(closes(TokenKind::Open(Bracket::Curly), TokenKind::Close(Bracket::Curly)));
    }

    #[test]
    fn mismatched_brackets_are_rejected() {
        assert!(!closes(TokenKind::Open(Bracket::Paren), TokenKind::Close(Bracket::Square)));
        assert!(!closes(TokenKind::Open(Bracket::Paren), TokenKind::Close(Bracket::Curly)));
        assert!(!closes(TokenKind::Open(Bracket::Square), TokenKind::Close(Bracket::Paren)));
    }

    #[test]
    fn non_bracket_pairs_are_not_matched() {
        assert!(!closes(TokenKind::Identifier, TokenKind::Close(Bracket::Paren)));
        assert!(!closes(TokenKind::Open(Bracket::Paren), TokenKind::Separator));
    }

    #[test]
    fn pair_classification_uses_match_result() {
        assert_eq!(classify_pair(TokenKind::Open(Bracket::Paren), TokenKind::Close(Bracket::Paren)), "matched");
        assert_eq!(classify_pair(TokenKind::Open(Bracket::Paren), TokenKind::Close(Bracket::Square)), "mismatched");
        assert_eq!(classify_pair(TokenKind::Number, TokenKind::Separator), "not brackets");
    }
}
