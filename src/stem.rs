//! Italian light stemmer.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Italian light stemmer — removes common Italian suffixes.
#[derive(Clone, Debug, Default)]
pub struct ItalianLightStemFilter;

impl ItalianLightStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for ItalianLightStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 5 {
            return (false, None);
        }
        let stemmed = stem_italian_light(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_italian_light(word: &str) -> String {
    let mut result = String::from(word);

    if result.ends_with("chi") || result.ends_with("ghi") {
        result.truncate(result.len() - 1);
        result.push('o');
        return result;
    }
    if result.ends_with("ione") || result.ends_with("ioni") {
        result.truncate(result.len() - 4);
        return result;
    }
    if result.ends_with("mente") {
        result.truncate(result.len() - 5);
        return result;
    }
    if result.ends_with('i')
        || result.ends_with('e')
        || result.ends_with('a')
        || result.ends_with('o')
    {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural() {
        let f = ItalianLightStemFilter::new();
        let mut token = Token::new("ragazzi", 0, 7, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "ragazz");
    }

    #[test]
    fn test_mente() {
        let f = ItalianLightStemFilter::new();
        let mut token = Token::new("velocemente", 0, 11, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "veloce");
    }
}
