# http-status-text-mapper-tools

Maps integer HTTP status codes to their IANA reason phrases and back.

```rust
use http_status_text_mapper_tools::{code_to_phrase, phrase_to_code, try_code_to_phrase};

assert_eq!(code_to_phrase(200), Some("OK"));
assert_eq!(code_to_phrase(299), None);

assert_eq!(phrase_to_code("Not Found"), Some(404));
assert_eq!(phrase_to_code("not found"), None);

assert_eq!(try_code_to_phrase(404), Ok("Not Found"));
assert_eq!(try_code_to_phrase(299), Err(http_status_text_mapper_tools::UnknownStatusCode(299)));
```

The crate exports four items:

- `code_to_phrase(code: u16) -> Option<&'static str>`
- `phrase_to_code(phrase: &str) -> Option<u16>`
- `try_code_to_phrase(code: u16) -> Result<&'static str, UnknownStatusCode>`
- `entries() -> &'static [StatusEntry]`, where `StatusEntry` has `code: u16` and `phrase: &'static str`

## Why this exists

HTTP reason phrases are not part of the wire format in HTTP/2 and HTTP/3, and even in
HTTP/1.1 they are optional. But logs, dashboards, and test assertions still want to turn
`404` into `Not Found` and back. The IANA registry is the source of truth, and it is small
enough to embed as a static table.

The trade-off is exact, case-sensitive phrase matching. A caller who writes `"ok"` gets
`None`, not `200`. Accepting case-insensitive input would make the reverse mapping
ambiguous the moment two registered phrases differ only in case (they do not today, but
the registry is append-only and could). Exact matching also means the crate never has to
decide whether to normalize whitespace, which would otherwise hide typos in log pipelines.

## The awkward edge

Codes that are not in the registry return `None` from `code_to_phrase` and
`phrase_to_code`. That includes well-formed but unassigned codes such as `299`, and it
includes the entire 1xx range except the handful of registered entries. If you need an
error instead of an `Option`, use `try_code_to_phrase`, which returns `UnknownStatusCode`.
