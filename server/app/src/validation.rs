//! Laravel validation errors: the 422 body and message wording.
//!
//! Slice 3.1 has the core used by login: the ordered error list, the
//! `required` and `string` checks, and attribute display names. Slice 3.3
//! adds the rule engine for chapter payloads. The behaviour to copy is
//! listed in `server/contract/tests/validation.rs`.

use crate::error::AppError;
use serde_json::{Map, Value, json};

/// Errors in insertion order (Laravel reports them in rule order).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ValidationErrors {
    list: Vec<(String, Vec<String>)>,
}

impl ValidationErrors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn single(field: &str, message: &str) -> Self {
        let mut e = Self::new();
        e.add(field, message);
        e
    }

    pub fn add(&mut self, field: &str, message: impl Into<String>) {
        let message = message.into();
        match self.list.iter_mut().find(|(f, _)| f == field) {
            Some((_, msgs)) => msgs.push(message),
            None => self.list.push((field.to_string(), vec![message])),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// `Ok(())` when empty, else the 422 error.
    pub fn into_result(self) -> Result<(), AppError> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(AppError::Validation(self))
        }
    }

    /// `{"message": first[ (and N more error|errors)], "errors": {...}}`.
    pub fn body(&self) -> Value {
        let total: usize = self.list.iter().map(|(_, m)| m.len()).sum();
        let first = self
            .list
            .first()
            .and_then(|(_, m)| m.first())
            .cloned()
            .unwrap_or_default();
        let message = match total.saturating_sub(1) {
            0 => first,
            1 => format!("{first} (and 1 more error)"),
            n => format!("{first} (and {n} more errors)"),
        };
        let errors: Map<String, Value> = self
            .list
            .iter()
            .map(|(f, m)| (f.clone(), json!(m)))
            .collect();
        json!({ "message": message, "errors": errors })
    }
}

/// Laravel's display name: `str_replace('_', ' ', Str::snake($attribute))`.
/// `titleEm` → `title em`; dotted paths stay as they are.
pub fn display_name(attribute: &str) -> String {
    let mut out = String::with_capacity(attribute.len() + 4);
    for (i, c) in attribute.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push(' ');
            }
            out.push(c.to_ascii_lowercase());
        } else if c == '_' {
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

/// `required`: missing, `null`, a string that trims to empty, or an empty
/// array/object fails.
pub fn is_missing(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => true,
        Some(Value::String(s)) => s.trim().is_empty(),
        Some(Value::Array(a)) => a.is_empty(),
        Some(Value::Object(o)) => o.is_empty(),
        Some(_) => false,
    }
}

pub fn msg_required(attr: &str) -> String {
    format!("The {} field is required.", display_name(attr))
}

pub fn msg_string(attr: &str) -> String {
    format!("The {} field must be a string.", display_name(attr))
}

/// `['required', 'string']` on one attribute: the first failing rule only
/// (a failed `required` stops the attribute, and `string` fails alone).
pub fn required_string<'a>(
    input: &'a Map<String, Value>,
    attr: &str,
    errors: &mut ValidationErrors,
) -> Option<&'a str> {
    let value = input.get(attr);
    if is_missing(value) {
        errors.add(attr, msg_required(attr));
        return None;
    }
    match value {
        Some(Value::String(s)) => Some(s.as_str()),
        _ => {
            errors.add(attr, msg_string(attr));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_names() {
        assert_eq!(display_name("titleEm"), "title em");
        assert_eq!(display_name("password"), "password");
        assert_eq!(display_name("lesson.formation.0"), "lesson.formation.0");
        assert_eq!(display_name("original_name"), "original name");
    }

    #[test]
    fn body_suffix_and_order() {
        let mut e = ValidationErrors::new();
        e.add("b", "B.");
        e.add("a", "A1.");
        e.add("a", "A2.");
        let body = e.body();
        assert_eq!(body["message"], "B. (and 2 more errors)");
        let keys: Vec<_> = body["errors"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        assert_eq!(keys, ["b", "a"]);
        assert_eq!(ValidationErrors::single("x", "X.").body()["message"], "X.");
    }

    #[test]
    fn required_semantics() {
        assert!(is_missing(None));
        assert!(is_missing(Some(&json!(null))));
        assert!(is_missing(Some(&json!("   "))));
        assert!(is_missing(Some(&json!([]))));
        assert!(is_missing(Some(&json!({}))));
        assert!(!is_missing(Some(&json!(0))));
        assert!(!is_missing(Some(&json!(false))));
        assert!(!is_missing(Some(&json!(" x "))));
    }

    #[test]
    fn required_string_messages() {
        let input = |v: Value| match json!({ "password": v }) {
            Value::Object(m) => m,
            _ => unreachable!(),
        };
        let mut e = ValidationErrors::new();
        assert_eq!(
            required_string(&input(json!(" pw ")), "password", &mut e),
            Some(" pw ")
        );
        assert!(e.is_empty());
        for (v, msg) in [
            (json!(null), "The password field is required."),
            (json!(""), "The password field is required."),
            (json!(["x"]), "The password field must be a string."),
            (json!(true), "The password field must be a string."),
            (json!(123), "The password field must be a string."),
        ] {
            let mut e = ValidationErrors::new();
            assert_eq!(required_string(&input(v), "password", &mut e), None);
            assert_eq!(e, ValidationErrors::single("password", msg));
        }
    }
}
