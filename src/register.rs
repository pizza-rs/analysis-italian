//! Register Italian analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::LowercaseNormalizer;
use pizza_engine::analysis::Normalizer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;
use pizza_engine::analysis::Tokenizer;

use crate::ItalianElisionFilter;
use crate::ItalianLightStemFilter;
use crate::ItalianStopFilter;

/// Register Italian token filters and the `"italian"` analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("italian_elision", Box::new(ItalianElisionFilter::new()));
    factory.register_token_filter(
        "italian_light_stem",
        Box::new(ItalianLightStemFilter::new()),
    );
    factory.register_token_filter("italian_stop", Box::new(ItalianStopFilter::new()));

    let normalizers: Vec<Box<dyn Normalizer>> = vec![Box::new(LowercaseNormalizer::new())];
    let tokenizer: Box<dyn Tokenizer> = Box::new(StandardTokenizer::new());
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(ItalianElisionFilter::new()),
        Box::new(ItalianStopFilter::new()),
        Box::new(ItalianLightStemFilter::new()),
    ];
    factory.register_analyzer("italian", Analyzer::new(normalizers, tokenizer, filters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("italian_elision").is_some());
        assert!(factory.get_token_filter("italian_light_stem").is_some());
        assert!(factory.get_token_filter("italian_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("italian").is_some());
    }

    #[test]
    fn test_analyzer_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("italian").unwrap();
        let mut input = String::from("L'uomo non è nella casa");
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        assert!(!tokens.iter().any(|t| t.term == "non"));
        assert!(tokens.len() >= 2);
    }
}
