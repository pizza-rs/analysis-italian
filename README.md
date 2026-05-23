# pizza-analysis-italian

Italian language analysis with elision handling, light stemmer, and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `italian_elision` | Token Filter | Removes Italian elisions (l', dell', un', etc.) |
| `italian_stem` | Token Filter | Italian light stemmer — removes common suffixes |
| `italian_stop` | Token Filter | Italian stop words filter (279 words) |
| `italian` | Analyzer | Full pipeline: lowercase → elision → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "italian"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["italian_elision", "italian_stem", "italian_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
