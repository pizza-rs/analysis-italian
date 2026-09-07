//! Comprehensive tests for pizza-analysis-italian.

use pizza_analysis_italian::*;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// ItalianElisionFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn elision_construction() {
    let _f = ItalianElisionFilter::new();
}

#[test]
fn elision_removes_l_apostrophe() {
    let f = ItalianElisionFilter::new();
    // "l'uomo" → "uomo"
    let mut token = make_token("l'uomo");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "uomo");
}

#[test]
fn elision_removes_d_apostrophe() {
    let f = ItalianElisionFilter::new();
    // "d'Italia" → "Italia"
    let mut token = make_token("d'italia");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "italia");
}

#[test]
fn elision_removes_un_apostrophe() {
    let f = ItalianElisionFilter::new();
    // "un'amica" → "amica"
    let mut token = make_token("un'amica");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "amica");
}

#[test]
fn elision_removes_dell() {
    let f = ItalianElisionFilter::new();
    // "dell'anno" → "anno"
    let mut token = make_token("dell'anno");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "anno");
}

#[test]
fn elision_no_change_without_apostrophe() {
    let f = ItalianElisionFilter::new();
    let mut token = make_token("casa");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "casa");
}

#[test]
fn elision_empty_string() {
    let f = ItalianElisionFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// ItalianLightStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = ItalianLightStemFilter::new();
}

#[test]
fn stem_plural_i() {
    let f = ItalianLightStemFilter::new();
    // Lucene's Italian light stemmer leaves 5-char "gatti" unchanged
    // (itlight.txt: gatti→gatti); longer plurals do stem (gattoni→gatton).
    let mut token = make_token("gatti");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "gatti");
    let mut token = make_token("gattoni");
    f.filter(&mut token);
    assert_eq!(token.term.as_ref(), "gatton");
}

#[test]
fn stem_plural_e() {
    let f = ItalianLightStemFilter::new();
    // "case" (houses) → stem
    let mut token = make_token("case");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_feminine_a() {
    let f = ItalianLightStemFilter::new();
    // "bella" (beautiful, f.) → stem
    let mut token = make_token("bella");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_form() {
    let f = ItalianLightStemFilter::new();
    let mut token = make_token("mangiare");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = ItalianLightStemFilter::new();
    let mut token = make_token("il");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = ItalianLightStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_single_char() {
    let f = ItalianLightStemFilter::new();
    let mut token = make_token("a");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// ItalianStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = ItalianStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = ItalianStopFilter::new();
    let stop_words = [
        "il", "lo", "la", "le", "di", "a", "da", "in", "con", "e", "che", "non",
    ];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = ItalianStopFilter::new();
    let content_words = ["casa", "libro", "scuola", "gatto"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = ItalianStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("italian_elision").is_some());
    assert!(factory.get_token_filter("italian_light_stem").is_some());
    assert!(factory.get_token_filter("italian_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("italian").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("italian").unwrap();
    let mut input = String::from("Il gatto è sul tavolo");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("italian").unwrap();
    let mut input = String::from("il libro di casa");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"il"));
    assert!(!terms.contains(&"di"));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("italian").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_elision_handling() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("italian").unwrap();
    let mut input = String::from("l'uomo");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
