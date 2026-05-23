//! Italian elision removal.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashSet;
use pizza_engine::analysis::{Token, TokenFilter};

const ITALIAN_ARTICLES: &[&str] = &[
    "c", "l", "all", "dall", "dell", "nell", "sull", "coll", "pell",
    "gl", "agl", "dagl", "degl", "negl", "sugl", "un", "m", "t", "s", "v", "d",
];

/// Removes Italian article elisions (l', dell', etc.) from tokens.
#[derive(Clone, Debug)]
pub struct ItalianElisionFilter {
    articles: HashSet<String>,
}

impl ItalianElisionFilter {
    pub fn new() -> Self {
        Self {
            articles: ITALIAN_ARTICLES.iter().map(|a| a.to_string()).collect(),
        }
    }
}

impl Default for ItalianElisionFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenFilter for ItalianElisionFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if let Some(apos_pos) = term.find(|c| c == '\'' || c == '\u{2019}') {
            let prefix = &term[..apos_pos];
            if self.articles.contains(&prefix.to_lowercase()) {
                let remainder = &term[apos_pos + 1..];
                if !remainder.is_empty() {
                    token.term = Cow::Owned(remainder.to_string());
                    token.start_offset += (apos_pos as u32) + 1;
                }
            }
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_italian_elision() {
        let f = ItalianElisionFilter::new();
        let mut token = Token::new("l'uomo", 0, 6, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "uomo");
    }

    #[test]
    fn test_dell() {
        let f = ItalianElisionFilter::new();
        let mut token = Token::new("dell'arte", 0, 9, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "arte");
    }
}
