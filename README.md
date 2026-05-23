<div align="center">

# 🇮🇹 pizza-analysis-italian

**Italian text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--italian-blue)](https://github.com/pizza-rs/analysis-italian)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Italian language analysis with elision handling, light stemming, and stop words.
Correctly processes Italian contracted articles (l', dell', un', etc.).

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `italian_elision` | Strip Italian elided articles (l', dell', all', etc.) |
| TokenFilter | `italian_light_stem` | Italian light stemmer |
| TokenFilter | `italian_stop` | Italian stop words (279 entries) |
| Analyzer | `italian` | Full pipeline: lowercase → elision → light_stem → stop |

### Elision

Italian contracts prepositions and articles before vowels:
- `l'uomo` → `uomo`
- `dell'arte` → `arte`
- `un'altra` → `altra`

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_italian::register_all(&mut factory);

let analyzer = factory.get_analyzer("italian").unwrap();
// "l'università" → ["universit"]
```

## Installation

```toml
[dependencies]
pizza-analysis-italian = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["italian"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
