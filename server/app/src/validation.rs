//! Laravel validation: the 422 body, message wording and the rule engine.
//!
//! Slice 3.1 added the core used by login (`required_string`). Slice 3.3
//! adds [`validate`], a port of Laravel's `Validator` for the rules the
//! chapter endpoints use. The behaviour it copies is pinned by
//! `server/contract/tests/validation.rs` and `chapters_write.rs`:
//!
//! - **What is validated.** [`validation_view`] is the input as Laravel's
//!   validator sees it after the global `TrimStrings` and
//!   `ConvertEmptyStringsToNull` middleware, with one approved difference:
//!   blank strings (only characters in [`php::is_blank_char`]) become
//!   `null`, but other strings are **not trimmed** (KNOWN_ISSUES K2, K18).
//!   Handlers validate the view and store the raw input.
//! - **Error order.** Rules without `*` come first, in declaration order;
//!   then each wildcard rule's expansions, rule by rule
//!   (`ValidationRuleParser::explodeRules`). Within one wildcard rule the
//!   order is Laravel's `ValidationData::initializeAndGatherData`, copied
//!   literally in [`expand`]: for a pattern ending in `*`, scalar, null and
//!   empty-array children come before non-empty arrays and objects.
//! - **Skipping.** A rule other than `required`/`present` is skipped when
//!   the value is a PHP-blank string, when the key is missing, when it is
//!   `null` and the attribute is `nullable`, or when the attribute has
//!   `sometimes` and the key is missing. A failed `required`/`present`
//!   stops the attribute. `unique` is skipped once the attribute has an
//!   error.
//! - **Display names.** Attributes from a wildcard keep their key as is
//!   (`items.0.type`); others go through `Str::snake` (`titleEm` → `title
//!   em`).
//!
//! To add a rule: add a [`Rule`] variant, its check in `passes`, its message
//! in `message`, and say whether it is implicit in `Rule::is_implicit`.

use crate::error::AppError;
use crate::php;
use serde_json::{Map, Value, json};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

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

    /// Whether `field` has an error (`MessageBag::has`).
    pub fn has(&self, field: &str) -> bool {
        self.list.iter().any(|(f, _)| f == field)
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

/// Laravel's display name for a declared attribute:
/// `str_replace('_', ' ', Str::snake($attribute))`. `titleEm` → `title em`;
/// dotted paths stay as they are.
pub fn display_name(attribute: &str) -> String {
    php::snake_words(attribute)
}

/// `required`: missing, `null`, a string that PHP's `trim` empties, or an
/// empty array/object fails. (Login's `password` is exempt from
/// `TrimStrings`, so only PHP's own `trim` characters count here; chapter
/// input is nulled through [`validation_view`] first.)
pub fn is_missing(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => true,
        Some(Value::String(s)) => php::trim(s).is_empty(),
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

/// The input Laravel's validator sees: every blank string (see
/// [`php::is_blank`]) becomes `null`, at any depth; nothing is trimmed.
pub fn validation_view(input: &Map<String, Value>) -> Map<String, Value> {
    fn view(v: &Value) -> Value {
        match v {
            Value::String(s) if php::is_blank(s) => Value::Null,
            Value::Array(items) => Value::Array(items.iter().map(view).collect()),
            Value::Object(map) => {
                Value::Object(map.iter().map(|(k, v)| (k.clone(), view(v))).collect())
            }
            other => other.clone(),
        }
    }
    input.iter().map(|(k, v)| (k.clone(), view(v))).collect()
}

/// `regex:` patterns the app uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    /// `/^[a-z0-9-]+$/`, but anchored at the very end: PCRE's `$` also
    /// accepts a trailing "\n", which Laravel's `TrimStrings` had already
    /// removed (K18).
    Slug,
}

impl Pattern {
    fn is_match(self, s: &str) -> bool {
        match self {
            Pattern::Slug => {
                !s.is_empty()
                    && s.bytes()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
            }
        }
    }
}

/// One Laravel rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    Required,
    Present,
    Sometimes,
    Nullable,
    String,
    Integer,
    Array,
    Min(i64),
    Max(i64),
    Regex(Pattern),
    In(&'static [&'static str]),
    /// `unique`: the caller looks the value up first (it needs the
    /// database) and passes the answer in.
    Unique {
        taken: bool,
    },
}

impl Rule {
    /// Laravel's `$implicitRules` among ours: they run on missing and blank
    /// values, and a failure stops the attribute.
    fn is_implicit(self) -> bool {
        matches!(self, Rule::Required | Rule::Present)
    }
}

/// A validation-view value in PHP's array model: arrays and objects are
/// both ordered string-keyed maps (lists keyed "0".."n-1").
#[derive(Debug, Clone)]
enum P {
    Scalar(Value),
    Arr(Vec<(String, P)>),
}

impl P {
    fn from_value(v: &Value) -> P {
        match v {
            Value::Array(items) => P::Arr(
                items
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (i.to_string(), P::from_value(v)))
                    .collect(),
            ),
            Value::Object(map) => P::Arr(
                map.iter()
                    .map(|(k, v)| (k.clone(), P::from_value(v)))
                    .collect(),
            ),
            other => P::Scalar(other.clone()),
        }
    }

    fn child(&self, key: &str) -> Option<&P> {
        match self {
            P::Arr(ch) => ch.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            P::Scalar(_) => None,
        }
    }

    fn is_null(&self) -> bool {
        matches!(self, P::Scalar(Value::Null))
    }

    /// The scalar as JSON; `None` for arrays.
    fn scalar(&self) -> Option<&Value> {
        match self {
            P::Scalar(v) => Some(v),
            P::Arr(_) => None,
        }
    }
}

/// `Arr::get($data, $path)` (`None` = missing).
fn get_path<'a>(root: &'a P, path: &[String]) -> Option<&'a P> {
    path.iter().try_fold(root, |cur, seg| cur.child(seg))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Seg {
    Star,
    Key(String),
}

fn parse_pattern(pattern: &str) -> Vec<Seg> {
    pattern
        .split('.')
        .map(|s| {
            if s == "*" {
                Seg::Star
            } else {
                Seg::Key(s.to_string())
            }
        })
        .collect()
}

/// `data_set($target, $segments, null, overwrite: true)`.
fn data_set(target: &mut P, segs: &[Seg]) {
    let Some((first, rest)) = segs.split_first() else {
        return;
    };
    if !matches!(target, P::Arr(_)) {
        *target = P::Arr(Vec::new());
    }
    let P::Arr(children) = target else {
        return;
    };
    match first {
        Seg::Star => {
            for (_, child) in children.iter_mut() {
                if rest.is_empty() {
                    *child = P::Scalar(Value::Null);
                } else {
                    data_set(child, rest);
                }
            }
        }
        Seg::Key(k) => {
            let idx = match children.iter().position(|(ck, _)| ck == k) {
                Some(i) => i,
                None => {
                    children.push((k.clone(), P::Arr(Vec::new())));
                    children.len() - 1
                }
            };
            if rest.is_empty() {
                children[idx].1 = P::Scalar(Value::Null);
            } else {
                data_set(&mut children[idx].1, rest);
            }
        }
    }
}

/// `Arr::dot`: the paths of the leaves (scalars, null, empty arrays), in
/// order.
fn dot(p: &P, prefix: &mut Vec<String>, out: &mut Vec<Vec<String>>) {
    if let P::Arr(children) = p {
        for (k, v) in children {
            prefix.push(k.clone());
            match v {
                P::Arr(inner) if !inner.is_empty() => dot(v, prefix, out),
                _ => out.push(prefix.clone()),
            }
            prefix.pop();
        }
    }
}

/// The regex `^pattern` (no end anchor, `*` = `[^.]*`) on a dotted key:
/// the matched prefix. A literal last segment may match the start of a
/// longer key segment, as the regex does.
fn prefix_match(segs: &[Seg], key: &[String]) -> Option<Vec<String>> {
    if key.len() < segs.len() {
        return None;
    }
    let last = segs.len() - 1;
    segs.iter()
        .zip(key)
        .enumerate()
        .map(|(i, (seg, k))| match seg {
            Seg::Star => Some(k.clone()),
            Seg::Key(lit) if i == last && k.starts_with(lit.as_str()) => Some(lit.clone()),
            Seg::Key(lit) if k == lit => Some(lit.clone()),
            Seg::Key(_) => None,
        })
        .collect()
}

fn full_match(segs: &[Seg], key: &[String]) -> bool {
    segs.len() == key.len()
        && segs.iter().zip(key).all(|(s, k)| match s {
            Seg::Star => true,
            Seg::Key(lit) => lit == k,
        })
}

/// The attributes a wildcard pattern expands to, in Laravel's order:
/// `ValidationRuleParser::explodeWildcardRules` over
/// `ValidationData::initializeAndGatherData`.
fn expand(segs: &[Seg], master: &P) -> Vec<Vec<String>> {
    // getLeadingExplicitAttributePath + extractDataFromPath
    let explicit: Vec<String> = segs
        .iter()
        .take_while(|s| **s != Seg::Star)
        .map(|s| match s {
            Seg::Key(k) => k.clone(),
            Seg::Star => unreachable!("take_while stops at Star"),
        })
        .collect();
    let mut data = if explicit.is_empty() {
        master.clone()
    } else {
        match get_path(master, &explicit) {
            Some(v) => explicit
                .iter()
                .rev()
                .fold(v.clone(), |inner, k| P::Arr(vec![(k.clone(), inner)])),
            None => P::Arr(Vec::new()),
        }
    };
    // initializeAttributeOnData: fill in the pattern's leaves with null.
    if segs.contains(&Seg::Star) && segs.last() != Some(&Seg::Star) {
        data_set(&mut data, segs);
    }
    let mut leaves = Vec::new();
    dot(&data, &mut Vec::new(), &mut leaves);
    // array_merge(Arr::dot(...), extractValuesForWildcards(...))
    let mut seen: HashSet<Vec<String>> = leaves.iter().cloned().collect();
    let mut keys = leaves.clone();
    for leaf in &leaves {
        if let Some(m) = prefix_match(segs, leaf)
            && seen.insert(m.clone())
        {
            keys.push(m);
        }
    }
    keys.retain(|k| full_match(segs, k));
    keys
}

struct Attr {
    path: Vec<String>,
    /// From a wildcard: the message shows the key as is.
    implicit: bool,
    rules: Vec<Rule>,
}

/// `ValidationRuleParser::explodeRules`: explicit attributes keep their
/// places, wildcard expansions follow, rule by rule.
fn explode(root: &P, rules: &[(&str, Vec<Rule>)]) -> Vec<Attr> {
    let mut out: Vec<Attr> = Vec::new();
    let mut index: HashMap<Vec<String>, usize> = HashMap::new();
    for (pattern, list) in rules.iter().filter(|(p, _)| !p.contains('*')) {
        let path: Vec<String> = pattern.split('.').map(str::to_string).collect();
        index.insert(path.clone(), out.len());
        out.push(Attr {
            path,
            implicit: false,
            rules: list.clone(),
        });
    }
    for (pattern, list) in rules.iter().filter(|(p, _)| p.contains('*')) {
        for path in expand(&parse_pattern(pattern), root) {
            if let Some(&i) = index.get(&path) {
                out[i].rules.extend(list);
            } else {
                index.insert(path.clone(), out.len());
                out.push(Attr {
                    path,
                    implicit: true,
                    rules: list.clone(),
                });
            }
        }
    }
    out
}

/// Laravel's `getSize`: a number for numeric values of attributes with a
/// numeric rule, else a count (array items or characters).
enum Size {
    Numeric(String),
    Count(usize),
}

fn size(value: Option<&P>, rules: &[Rule]) -> Size {
    let has_numeric = rules.contains(&Rule::Integer);
    match value {
        Some(P::Arr(ch)) => Size::Count(ch.len()),
        Some(P::Scalar(v)) if has_numeric && php::is_numeric(v) => {
            Size::Numeric(php::trim(&php::to_string(v).unwrap_or_default()).to_string())
        }
        Some(P::Scalar(v)) => Size::Count(php::to_string(v).unwrap_or_default().chars().count()),
        None => Size::Count(0),
    }
}

fn size_cmp(value: Option<&P>, rules: &[Rule], n: i64) -> Option<Ordering> {
    match size(value, rules) {
        Size::Numeric(s) => php::cmp_decimal(&s, n),
        Size::Count(c) => Some(i64::try_from(c).unwrap_or(i64::MAX).cmp(&n)),
    }
}

/// The `:attribute` text in messages.
fn shown_name(attr: &Attr, joined: &str) -> String {
    let plain = attr.path.iter().all(|s| !s.contains(['.', '*']));
    if attr.implicit && plain {
        joined.to_string()
    } else {
        display_name(joined)
    }
}

fn message(rule: Rule, name: &str, rules: &[Rule]) -> String {
    let kind = if rules.contains(&Rule::Integer) {
        "numeric"
    } else if rules.contains(&Rule::Array) {
        "array"
    } else {
        "string"
    };
    match rule {
        Rule::Required => format!("The {name} field is required."),
        Rule::Present => format!("The {name} field must be present."),
        Rule::String => format!("The {name} field must be a string."),
        Rule::Integer => format!("The {name} field must be an integer."),
        Rule::Array => format!("The {name} field must be an array."),
        Rule::Min(n) => match kind {
            "numeric" => format!("The {name} field must be at least {n}."),
            "array" => format!("The {name} field must have at least {n} items."),
            _ => format!("The {name} field must be at least {n} characters."),
        },
        Rule::Max(n) => match kind {
            "numeric" => format!("The {name} field must not be greater than {n}."),
            "array" => format!("The {name} field must not have more than {n} items."),
            _ => format!("The {name} field must not be greater than {n} characters."),
        },
        Rule::Regex(_) => format!("The {name} field format is invalid."),
        Rule::In(_) => format!("The selected {name} is invalid."),
        Rule::Unique { .. } => format!("The {name} has already been taken."),
        Rule::Sometimes | Rule::Nullable => String::new(),
    }
}

fn passes(rule: Rule, value: Option<&P>, exists: bool, rules: &[Rule]) -> bool {
    let scalar = value.and_then(P::scalar);
    match rule {
        Rule::Required => match value {
            None => false,
            Some(P::Arr(ch)) => !ch.is_empty(),
            Some(P::Scalar(v)) => !is_missing(Some(v)),
        },
        Rule::Present => exists,
        Rule::Sometimes | Rule::Nullable => true,
        Rule::String => matches!(scalar, Some(Value::String(_))),
        Rule::Integer => scalar.is_some_and(|v| php::filter_int(v).is_some()),
        Rule::Array => matches!(value, Some(P::Arr(_))),
        Rule::Min(n) => size_cmp(value, rules, n).is_some_and(Ordering::is_ge),
        Rule::Max(n) => size_cmp(value, rules, n).is_some_and(Ordering::is_le),
        Rule::Regex(p) => match scalar {
            Some(v @ (Value::String(_) | Value::Number(_))) => {
                php::to_string(v).is_some_and(|s| p.is_match(&s))
            }
            _ => false,
        },
        Rule::In(list) => scalar
            .and_then(php::to_string)
            .is_some_and(|s| list.contains(&s.as_str())),
        Rule::Unique { taken } => !taken,
    }
}

/// Laravel's `Validator::passes()` for `rules` (declaration order, `*`
/// wildcards allowed) over `view` (see [`validation_view`]).
pub fn validate(view: &Map<String, Value>, rules: &[(&str, Vec<Rule>)]) -> ValidationErrors {
    let root = P::from_value(&Value::Object(view.clone()));
    let mut errors = ValidationErrors::new();
    for attr in explode(&root, rules) {
        let joined = attr.path.join(".");
        let value = get_path(&root, &attr.path);
        let exists = value.is_some();
        let blank = matches!(value, Some(P::Scalar(Value::String(s))) if php::trim(s).is_empty());
        let nullable = attr.rules.contains(&Rule::Nullable);
        let sometimes = attr.rules.contains(&Rule::Sometimes);
        for &rule in &attr.rules {
            if matches!(rule, Rule::Sometimes | Rule::Nullable) {
                continue;
            }
            let implicit = rule.is_implicit();
            // isValidatable()
            let present_or_implicit = if blank { implicit } else { exists || implicit };
            let optional_ok = !sometimes || exists;
            let not_null = implicit || !nullable || !value.is_some_and(P::is_null);
            let unique_ok = !matches!(rule, Rule::Unique { .. }) || !errors.has(&joined);
            if !(present_or_implicit && optional_ok && not_null && unique_ok) {
                continue;
            }
            if !passes(rule, value, exists, &attr.rules) {
                errors.add(
                    &joined,
                    message(rule, &shown_name(&attr, &joined), &attr.rules),
                );
                if implicit {
                    break;
                }
            }
        }
    }
    errors
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
