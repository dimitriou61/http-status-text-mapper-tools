//! Maps HTTP status codes to their standard reason phrases and back per the IANA registry.
//!
//! The registry is treated as a set of exact pairs. Lookups are case-sensitive for
//! phrases because the registry itself is case-sensitive: "OK" is the registered
//! phrase, not "ok". This is a deliberate choice; accepting case-insensitive input
//! would make the reverse mapping ambiguous when a caller passes a phrase that is
//! not in the registry but differs only in case.

use std::fmt;

/// A single entry from the IANA HTTP status code registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusEntry {
    /// The integer status code.
    pub code: u16,
    /// The registered reason phrase.
    pub phrase: &'static str,
}

/// The IANA registry as of the most recent update included in this crate.
///
/// The list is intentionally a `&'static [StatusEntry]` rather than a map so that
/// `code_to_phrase` and `phrase_to_code` can both be simple linear scans. The
/// registry is small enough that a linear scan is faster than hashing and avoids
/// any allocation or lazy initialization.
const REGISTRY: &[StatusEntry] = &[
    StatusEntry { code: 100, phrase: "Continue" },
    StatusEntry { code: 101, phrase: "Switching Protocols" },
    StatusEntry { code: 102, phrase: "Processing" },
    StatusEntry { code: 103, phrase: "Early Hints" },
    StatusEntry { code: 200, phrase: "OK" },
    StatusEntry { code: 201, phrase: "Created" },
    StatusEntry { code: 202, phrase: "Accepted" },
    StatusEntry { code: 203, phrase: "Non-Authoritative Information" },
    StatusEntry { code: 204, phrase: "No Content" },
    StatusEntry { code: 205, phrase: "Reset Content" },
    StatusEntry { code: 206, phrase: "Partial Content" },
    StatusEntry { code: 207, phrase: "Multi-Status" },
    StatusEntry { code: 208, phrase: "Already Reported" },
    StatusEntry { code: 226, phrase: "IM Used" },
    StatusEntry { code: 300, phrase: "Multiple Choices" },
    StatusEntry { code: 301, phrase: "Moved Permanently" },
    StatusEntry { code: 302, phrase: "Found" },
    StatusEntry { code: 303, phrase: "See Other" },
    StatusEntry { code: 304, phrase: "Not Modified" },
    StatusEntry { code: 305, phrase: "Use Proxy" },
    StatusEntry { code: 307, phrase: "Temporary Redirect" },
    StatusEntry { code: 308, phrase: "Permanent Redirect" },
    StatusEntry { code: 400, phrase: "Bad Request" },
    StatusEntry { code: 401, phrase: "Unauthorized" },
    StatusEntry { code: 402, phrase: "Payment Required" },
    StatusEntry { code: 403, phrase: "Forbidden" },
    StatusEntry { code: 404, phrase: "Not Found" },
    StatusEntry { code: 405, phrase: "Method Not Allowed" },
    StatusEntry { code: 406, phrase: "Not Acceptable" },
    StatusEntry { code: 407, phrase: "Proxy Authentication Required" },
    StatusEntry { code: 408, phrase: "Request Timeout" },
    StatusEntry { code: 409, phrase: "Conflict" },
    StatusEntry { code: 410, phrase: "Gone" },
    StatusEntry { code: 411, phrase: "Length Required" },
    StatusEntry { code: 412, phrase: "Precondition Failed" },
    StatusEntry { code: 413, phrase: "Payload Too Large" },
    StatusEntry { code: 414, phrase: "URI Too Long" },
    StatusEntry { code: 415, phrase: "Unsupported Media Type" },
    StatusEntry { code: 416, phrase: "Range Not Satisfiable" },
    StatusEntry { code: 417, phrase: "Expectation Failed" },
    StatusEntry { code: 418, phrase: "I'm a Teapot" },
    StatusEntry { code: 421, phrase: "Misdirected Request" },
    StatusEntry { code: 422, phrase: "Unprocessable Content" },
    StatusEntry { code: 423, phrase: "Locked" },
    StatusEntry { code: 424, phrase: "Failed Dependency" },
    StatusEntry { code: 425, phrase: "Too Early" },
    StatusEntry { code: 426, phrase: "Upgrade Required" },
    StatusEntry { code: 428, phrase: "Precondition Required" },
    StatusEntry { code: 429, phrase: "Too Many Requests" },
    StatusEntry { code: 431, phrase: "Request Header Fields Too Large" },
    StatusEntry { code: 451, phrase: "Unavailable For Legal Reasons" },
    StatusEntry { code: 500, phrase: "Internal Server Error" },
    StatusEntry { code: 501, phrase: "Not Implemented" },
    StatusEntry { code: 502, phrase: "Bad Gateway" },
    StatusEntry { code: 503, phrase: "Service Unavailable" },
    StatusEntry { code: 504, phrase: "Gateway Timeout" },
    StatusEntry { code: 505, phrase: "HTTP Version Not Supported" },
    StatusEntry { code: 506, phrase: "Variant Also Negotiates" },
    StatusEntry { code: 507, phrase: "Insufficient Storage" },
    StatusEntry { code: 508, phrase: "Loop Detected" },
    StatusEntry { code: 510, phrase: "Not Extended" },
    StatusEntry { code: 511, phrase: "Network Authentication Required" },
];

/// Returns the registered reason phrase for `code`, or `None` if the code is not
/// in the registry.
///
/// # Examples
///
/// ```
/// use http_status_text_mapper_tools::code_to_phrase;
/// assert_eq!(code_to_phrase(200), Some("OK"));
/// assert_eq!(code_to_phrase(299), None);
/// ```
pub fn code_to_phrase(code: u16) -> Option<&'static str> {
    REGISTRY
        .iter()
        .find(|entry| entry.code == code)
        .map(|entry| entry.phrase)
}

/// Returns the registered status code for `phrase`, or `None` if the phrase is not
/// in the registry.
///
/// The comparison is exact and case-sensitive. A phrase that differs only in case
/// from a registered phrase is not a match; this keeps the reverse mapping
/// unambiguous and matches the registry's own spelling.
///
/// # Examples
///
/// ```
/// use http_status_text_mapper_tools::phrase_to_code;
/// assert_eq!(phrase_to_code("OK"), Some(200));
/// assert_eq!(phrase_to_code("ok"), None);
/// ```
pub fn phrase_to_code(phrase: &str) -> Option<u16> {
    REGISTRY
        .iter()
        .find(|entry| entry.phrase == phrase)
        .map(|entry| entry.code)
}

/// Returns the full registry as a slice of entries.
///
/// The slice is in registry order, which is sorted by code. Callers that need to
/// iterate over every entry can use this instead of probing codes one at a time.
pub fn entries() -> &'static [StatusEntry] {
    REGISTRY
}

/// Error returned when a status code is not present in the registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownStatusCode(pub u16);

impl fmt::Display for UnknownStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown HTTP status code: {}", self.0)
    }
}

impl std::error::Error for UnknownStatusCode {}

/// Like [`code_to_phrase`], but returns an error instead of `None` for unknown
/// codes.
///
/// This is useful in code paths where an unknown code is a bug rather than a
/// recoverable condition, so the caller can use `?` and get a descriptive error.
pub fn try_code_to_phrase(code: u16) -> Result<&'static str, UnknownStatusCode> {
    code_to_phrase(code).ok_or(UnknownStatusCode(code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_codes_map_to_phrases() {
        assert_eq!(code_to_phrase(100), Some("Continue"));
        assert_eq!(code_to_phrase(200), Some("OK"));
        assert_eq!(code_to_phrase(301), Some("Moved Permanently"));
        assert_eq!(code_to_phrase(404), Some("Not Found"));
        assert_eq!(code_to_phrase(418), Some("I'm a Teapot"));
        assert_eq!(code_to_phrase(451), Some("Unavailable For Legal Reasons"));
        assert_eq!(code_to_phrase(500), Some("Internal Server Error"));
        assert_eq!(code_to_phrase(511), Some("Network Authentication Required"));
    }

    #[test]
    fn unknown_codes_return_none() {
        assert_eq!(code_to_phrase(0), None);
        assert_eq!(code_to_phrase(99), None);
        assert_eq!(code_to_phrase(299), None);
        assert_eq!(code_to_phrase(600), None);
        assert_eq!(code_to_phrase(u16::MAX), None);
    }

    #[test]
    fn known_phrases_map_to_codes() {
        assert_eq!(phrase_to_code("Continue"), Some(100));
        assert_eq!(phrase_to_code("OK"), Some(200));
        assert_eq!(phrase_to_code("Not Found"), Some(404));
        assert_eq!(phrase_to_code("I'm a Teapot"), Some(418));
        assert_eq!(phrase_to_code("Network Authentication Required"), Some(511));
    }

    #[test]
    fn phrase_lookup_is_case_sensitive() {
        assert_eq!(phrase_to_code("ok"), None);
        assert_eq!(phrase_to_code("Ok"), None);
        assert_eq!(phrase_to_code("not found"), None);
        assert_eq!(phrase_to_code("NOT FOUND"), None);
    }

    #[test]
    fn unknown_phrases_return_none() {
        assert_eq!(phrase_to_code(""), None);
        assert_eq!(phrase_to_code("Not A Real Phrase"), None);
        assert_eq!(phrase_to_code("OK "), None);
        assert_eq!(phrase_to_code(" OK"), None);
    }

    #[test]
    fn registry_is_sorted_and_unique() {
        let all = entries();
        assert!(!all.is_empty());
        for window in all.windows(2) {
            assert!(window[0].code < window[1].code, "codes must be strictly ascending");
            assert_ne!(window[0].phrase, window[1].phrase, "phrases must be unique");
        }
    }

    #[test]
    fn entries_round_trip() {
        for entry in entries() {
            assert_eq!(code_to_phrase(entry.code), Some(entry.phrase));
            assert_eq!(phrase_to_code(entry.phrase), Some(entry.code));
        }
    }

    #[test]
    fn try_code_to_phrase_ok() {
        assert_eq!(try_code_to_phrase(200), Ok("OK"));
        assert_eq!(try_code_to_phrase(404), Ok("Not Found"));
    }

    #[test]
    fn try_code_to_phrase_err() {
        assert_eq!(try_code_to_phrase(299), Err(UnknownStatusCode(299)));
        assert_eq!(try_code_to_phrase(0), Err(UnknownStatusCode(0)));
    }

    #[test]
    fn unknown_status_code_display() {
        let err = UnknownStatusCode(999);
        assert_eq!(format!("{}", err), "unknown HTTP status code: 999");
    }

    #[test]
    fn status_entry_is_copy() {
        let entry = StatusEntry { code: 200, phrase: "OK" };
        let copy = entry;
        assert_eq!(entry, copy);
    }
}
