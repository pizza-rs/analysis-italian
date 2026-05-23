//! Italian stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Italian stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "a",
    "abbia",
    "abbiamo",
    "abbiano",
    "abbiate",
    "ad",
    "agl",
    "agli",
    "ai",
    "al",
    "all",
    "alla",
    "alle",
    "allo",
    "anche",
    "avemmo",
    "avendo",
    "avesse",
    "avessero",
    "avessi",
    "avessimo",
    "aveste",
    "avesti",
    "avete",
    "aveva",
    "avevamo",
    "avevano",
    "avevate",
    "avevi",
    "avevo",
    "avrai",
    "avranno",
    "avrebbe",
    "avrebbero",
    "avrei",
    "avremmo",
    "avremo",
    "avreste",
    "avresti",
    "avrete",
    "avrà",
    "avrò",
    "avuta",
    "avute",
    "avuti",
    "avuto",
    "c",
    "che",
    "chi",
    "ci",
    "coi",
    "col",
    "come",
    "con",
    "contro",
    "cui",
    "da",
    "dagl",
    "dagli",
    "dai",
    "dal",
    "dall",
    "dalla",
    "dalle",
    "dallo",
    "degl",
    "degli",
    "dei",
    "del",
    "dell",
    "della",
    "delle",
    "dello",
    "di",
    "dov",
    "dove",
    "e",
    "ebbe",
    "ebbero",
    "ebbi",
    "ed",
    "era",
    "erano",
    "eravamo",
    "eravate",
    "eri",
    "ero",
    "essendo",
    "faccia",
    "facciamo",
    "facciano",
    "facciate",
    "faccio",
    "facemmo",
    "facendo",
    "facesse",
    "facessero",
    "facessi",
    "facessimo",
    "faceste",
    "facesti",
    "faceva",
    "facevamo",
    "facevano",
    "facevate",
    "facevi",
    "facevo",
    "fai",
    "fanno",
    "farai",
    "faranno",
    "farebbe",
    "farebbero",
    "farei",
    "faremmo",
    "faremo",
    "fareste",
    "faresti",
    "farete",
    "farà",
    "farò",
    "fece",
    "fecero",
    "feci",
    "fosse",
    "fossero",
    "fossi",
    "fossimo",
    "foste",
    "fosti",
    "fu",
    "fui",
    "fummo",
    "furono",
    "gli",
    "ha",
    "hai",
    "hanno",
    "ho",
    "i",
    "il",
    "in",
    "io",
    "l",
    "la",
    "le",
    "lei",
    "li",
    "lo",
    "loro",
    "lui",
    "ma",
    "mi",
    "mia",
    "mie",
    "miei",
    "mio",
    "ne",
    "negl",
    "negli",
    "nei",
    "nel",
    "nell",
    "nella",
    "nelle",
    "nello",
    "noi",
    "non",
    "nostra",
    "nostre",
    "nostri",
    "nostro",
    "o",
    "per",
    "perché",
    "più",
    "quale",
    "quanta",
    "quante",
    "quanti",
    "quanto",
    "quella",
    "quelle",
    "quelli",
    "quello",
    "questa",
    "queste",
    "questi",
    "questo",
    "sarai",
    "saranno",
    "sarebbe",
    "sarebbero",
    "sarei",
    "saremmo",
    "saremo",
    "sareste",
    "saresti",
    "sarete",
    "sarà",
    "sarò",
    "se",
    "sei",
    "si",
    "sia",
    "siamo",
    "siano",
    "siate",
    "siete",
    "sono",
    "sta",
    "stai",
    "stando",
    "stanno",
    "starai",
    "staranno",
    "starebbe",
    "starebbero",
    "starei",
    "staremmo",
    "staremo",
    "stareste",
    "staresti",
    "starete",
    "starà",
    "starò",
    "stava",
    "stavamo",
    "stavano",
    "stavate",
    "stavi",
    "stavo",
    "stemmo",
    "stesse",
    "stessero",
    "stessi",
    "stessimo",
    "steste",
    "stesti",
    "stette",
    "stettero",
    "stetti",
    "stia",
    "stiamo",
    "stiano",
    "stiate",
    "sto",
    "su",
    "sua",
    "sue",
    "sugl",
    "sugli",
    "sui",
    "sul",
    "sull",
    "sulla",
    "sulle",
    "sullo",
    "suo",
    "suoi",
    "ti",
    "tra",
    "tu",
    "tua",
    "tue",
    "tuo",
    "tuoi",
    "tutti",
    "tutto",
    "un",
    "una",
    "uno",
    "vi",
    "voi",
    "vostra",
    "vostre",
    "vostri",
    "vostro",
    "è",
    ];
    words.iter().copied().collect()
});

/// Removes Italian stop words from the token stream.
#[derive(Clone, Debug)]
pub struct ItalianStopFilter {
    stop_words: HashSet<String>,
}

impl Default for ItalianStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl ItalianStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for ItalianStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 279);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = ItalianStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = ItalianStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = ItalianStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
