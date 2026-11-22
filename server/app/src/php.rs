//! PHP behaviours that Laravel's validator and Eloquent casts depend on.
//! Each function names the PHP or Laravel code it copies. The edge cases
//! were captured from the sandboxed Laravel app (PHP 8.5, Laravel 13) in
//! slice 3.3; contract tests: `server/contract/tests/validation.rs`.
//!
//! JSON numbers are read the way `json_decode` reads them: integers that
//! fit in `i64` are PHP ints, everything else (fractions, exponents,
//! integers above `PHP_INT_MAX`) is a PHP float.

use serde_json::{Number, Value};
use std::cmp::Ordering;

/// One character Laravel treats as blank: removed by `Str::trim` (the
/// `TrimStrings` middleware: PCRE `\s` in Unicode mode plus
/// `Str::INVISIBLE_CHARACTERS`) or by PHP `trim`. Found by running both on
/// every code point.
pub fn is_blank_char(c: char) -> bool {
    matches!(
        c,
        '\u{0}'
            | '\t'..='\r'
            | ' '
            | '\u{85}'
            | '\u{A0}'
            | '\u{AD}'
            | '\u{34F}'
            | '\u{61C}'
            | '\u{115F}'
            | '\u{1160}'
            | '\u{1680}'
            | '\u{17B4}'
            | '\u{17B5}'
            | '\u{180E}'
            | '\u{2000}'..='\u{200F}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{2060}'..='\u{2065}'
            | '\u{206A}'..='\u{206F}'
            | '\u{2800}'
            | '\u{3000}'
            | '\u{3164}'
            | '\u{FEFF}'
            | '\u{FFA0}'
            | '\u{1D159}'
            | '\u{1D173}'..='\u{1D17A}'
            | '\u{E0020}'
    )
}

/// A string Laravel's middleware turns into `null`: `Str::trim($s) === ''`
/// (then `ConvertEmptyStringsToNull`). The empty string is blank.
pub fn is_blank(s: &str) -> bool {
    s.chars().all(is_blank_char)
}

/// PHP `trim($s)` with the default characters `" \t\n\r\0\x0B"`.
pub fn trim(s: &str) -> &str {
    s.trim_matches(|c| matches!(c, ' ' | '\t' | '\n' | '\r' | '\0' | '\u{B}'))
}

/// A JSON number as PHP holds it after `json_decode`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Num {
    Int(i64),
    Float(f64),
}

pub fn num(n: &Number) -> Num {
    match n.as_i64() {
        Some(i) => Num::Int(i),
        None => Num::Float(n.as_f64().unwrap_or(f64::NAN)),
    }
}

/// `(string) $float` with `precision = 14` (`zend_gcvt(value, 14, '.',
/// 'E')`): 14 significant digits, trailing zeros dropped, exponent form
/// `1.0E+25` / `1.5E-5` when the decimal exponent is below -4 or above 14.
pub fn float_to_string(f: f64) -> String {
    if f.is_nan() {
        return "NAN".into();
    }
    if f.is_infinite() {
        return if f > 0.0 { "INF" } else { "-INF" }.into();
    }
    if f == 0.0 {
        return if f.is_sign_negative() { "-0" } else { "0" }.into();
    }
    let sci = format!("{:.13e}", f.abs());
    let (mantissa, exp) = sci.split_once('e').unwrap_or((&sci, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    let digits = digits.trim_end_matches('0');
    let digits = if digits.is_empty() { "0" } else { digits };
    // `decpt` is where the decimal point goes, counted from the first digit.
    let decpt = exp + 1;
    let mut out = String::new();
    if f < 0.0 {
        out.push('-');
    }
    if !(-3..=14).contains(&decpt) {
        let (first, rest) = digits.split_at(1);
        out.push_str(first);
        out.push('.');
        out.push_str(if rest.is_empty() { "0" } else { rest });
        out.push('E');
        let e = decpt - 1;
        out.push(if e < 0 { '-' } else { '+' });
        out.push_str(&e.abs().to_string());
    } else if decpt <= 0 {
        out.push_str("0.");
        out.push_str(&"0".repeat(usize::try_from(-decpt).unwrap_or(0)));
        out.push_str(digits);
    } else {
        let point = usize::try_from(decpt).unwrap_or(0);
        if digits.len() <= point {
            out.push_str(digits);
            out.push_str(&"0".repeat(point - digits.len()));
        } else {
            out.push_str(&digits[..point]);
            out.push('.');
            out.push_str(&digits[point..]);
        }
    }
    out
}

/// `(string) $value` for a scalar. `None` for arrays and objects.
pub fn to_string(v: &Value) -> Option<String> {
    match v {
        Value::Null => Some(String::new()),
        Value::Bool(b) => Some(if *b { "1" } else { "" }.into()),
        Value::Number(n) => Some(match num(n) {
            Num::Int(i) => i.to_string(),
            Num::Float(f) => float_to_string(f),
        }),
        Value::String(s) => Some(s.clone()),
        Value::Array(_) | Value::Object(_) => None,
    }
}

/// PHP 8 `is_numeric($string)`: optional whitespace (`" \t\n\r\v\f"`), an
/// optional sign, digits with an optional fraction (`1.`, `.5`), an
/// optional exponent, optional trailing whitespace.
pub fn is_numeric_str(s: &str) -> bool {
    let ws = |c: char| matches!(c, ' ' | '\t' | '\n' | '\r' | '\u{B}' | '\u{C}');
    let b = s.trim_matches(ws).as_bytes();
    let mut i = 0;
    if matches!(b.first(), Some(b'+' | b'-')) {
        i += 1;
    }
    let int_start = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    let mut digits = i - int_start;
    if i < b.len() && b[i] == b'.' {
        i += 1;
        let frac_start = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        digits += i - frac_start;
    }
    if digits == 0 {
        return false;
    }
    if i < b.len() && matches!(b[i], b'e' | b'E') {
        let mut j = i + 1;
        if matches!(b.get(j), Some(b'+' | b'-')) {
            j += 1;
        }
        let exp_start = j;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
        }
        if j == exp_start {
            return false;
        }
        i = j;
    }
    i == b.len()
}

/// `is_numeric($value)`.
pub fn is_numeric(v: &Value) -> bool {
    match v {
        Value::Number(_) => true,
        Value::String(s) => is_numeric_str(s),
        _ => false,
    }
}

/// `filter_var($s, FILTER_VALIDATE_INT)` on a string: trims `" \t\r\v\n"`
/// (not `\f`, not `\0`), then an optional sign and either `0` or a digit
/// string without a leading zero, within `i64`.
pub fn filter_int_str(s: &str) -> Option<i64> {
    let s = s.trim_matches(|c| matches!(c, ' ' | '\t' | '\r' | '\u{B}' | '\n'));
    let (neg, digits) = match s.as_bytes().first() {
        Some(b'-') => (true, &s[1..]),
        Some(b'+') => (false, &s[1..]),
        _ => (false, s),
    };
    if digits.is_empty() || !digits.bytes().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if digits.starts_with('0') && digits.len() > 1 {
        return None;
    }
    // Parse the magnitude as i128 so `-9223372036854775808` fits.
    let magnitude: i128 = digits.parse().ok()?;
    i64::try_from(if neg { -magnitude } else { magnitude }).ok()
}

/// `filter_var($value, FILTER_VALIDATE_INT)`: scalars go through their
/// PHP string form (`true` → `"1"`, `1.0` → `"1"`); arrays fail.
pub fn filter_int(v: &Value) -> Option<i64> {
    match v {
        Value::Number(n) => match num(n) {
            Num::Int(i) => Some(i),
            Num::Float(f) => filter_int_str(&float_to_string(f)),
        },
        Value::Array(_) | Value::Object(_) => None,
        other => filter_int_str(&to_string(other)?),
    }
}

/// `(int) $value` for a value that passed the `integer` rule: what
/// Eloquent's `integer` cast reads back. A float truncates toward zero
/// (`0.999999999999999` prints as `"1"` and passes, but reads as 0).
pub fn int_value(v: &Value) -> Option<i64> {
    let filtered = filter_int(v)?;
    match v {
        Value::Number(n) => match num(n) {
            Num::Int(i) => Some(i),
            // Passing floats print as integers, so |f| < 1e15.
            #[allow(clippy::cast_possible_truncation)]
            Num::Float(f) => Some(f.trunc() as i64),
        },
        _ => Some(filtered),
    }
}

/// `BigNumber::of($s)` (brick/math) compared with `n`, after Laravel's
/// exponent guard (`|exponent| > 1000` fails). `None` when brick would
/// throw, which makes Laravel's `min`/`max` rule fail.
pub fn cmp_decimal(s: &str, n: i64) -> Option<Ordering> {
    if s.contains(['e', 'E']) {
        let after = if s.contains('e') {
            s.split_once('e')
        } else {
            s.split_once('E')
        }
        .map_or("", |(_, a)| a);
        let scale = php_int_prefix(after);
        if !(-1000..=1000).contains(&scale) {
            return None;
        }
    }
    let b = s.as_bytes();
    let mut i = 0;
    let neg = match b.first() {
        Some(b'-') => {
            i += 1;
            true
        }
        Some(b'+') => {
            i += 1;
            false
        }
        _ => false,
    };
    let start = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    let integral = &s[start..i];
    if i < b.len() && b[i] == b'.' {
        i += 1;
    }
    let fstart = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    let fractional = &s[fstart..i];
    let mut exp: i64 = 0;
    if i < b.len() && matches!(b[i], b'e' | b'E') {
        i += 1;
        let estart = i;
        if matches!(b.get(i), Some(b'+' | b'-')) {
            i += 1;
        }
        let dstart = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == dstart {
            return None;
        }
        exp = s[estart..i].parse().ok()?;
    }
    // brick's `$` also matches before a final "\n".
    let rest = &s[i..];
    if !(rest.is_empty() || rest == "\n") || (integral.is_empty() && fractional.is_empty()) {
        return None;
    }

    // value = 0.DIGITS × 10^point
    let all = format!("{integral}{fractional}");
    let point = i64::try_from(integral.len()).ok()? + exp;
    let (int_digits, frac_nonzero) = if point <= 0 {
        (String::new(), all.bytes().any(|c| c != b'0'))
    } else {
        let p = usize::try_from(point).ok()?;
        if p >= all.len() {
            (format!("{all}{}", "0".repeat(p - all.len())), false)
        } else {
            (all[..p].to_string(), all[p..].bytes().any(|c| c != b'0'))
        }
    };
    let int_digits = int_digits.trim_start_matches('0');
    let is_zero = int_digits.is_empty() && !frac_nonzero;
    let n_abs = n.unsigned_abs().to_string();
    let abs_cmp = || {
        let n_digits = if n == 0 { "" } else { n_abs.as_str() };
        int_digits
            .len()
            .cmp(&n_digits.len())
            .then_with(|| int_digits.cmp(n_digits))
            .then(if frac_nonzero {
                Ordering::Greater
            } else {
                Ordering::Equal
            })
    };
    Some(match (is_zero || !neg, n >= 0) {
        _ if is_zero => 0.cmp(&n),
        (true, true) => abs_cmp(),
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (false, false) => abs_cmp().reverse(),
    })
}

/// `(int) $string`: the leading integer, 0 when there is none.
fn php_int_prefix(s: &str) -> i64 {
    let t = s.trim_start_matches([' ', '\t', '\n', '\r', '\u{B}', '\u{C}']);
    let (neg, rest) = match t.as_bytes().first() {
        Some(b'-') => (true, &t[1..]),
        Some(b'+') => (false, &t[1..]),
        _ => (false, t),
    };
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    let v: i64 = digits
        .parse()
        .unwrap_or(if digits.is_empty() { 0 } else { i64::MAX });
    if neg { -v } else { v }
}

/// Laravel's `Str::snake($s)` followed by `str_replace('_', ' ', …)`: the
/// display name of an attribute in validation messages (`titleEm` → `title
/// em`). Strings of only lowercase ASCII letters are kept as they are.
pub fn snake_words(s: &str) -> String {
    if !s.is_empty() && s.bytes().all(|c| c.is_ascii_lowercase()) {
        return s.to_string();
    }
    // ucwords: uppercase an ASCII letter at the start or after " \t\r\n\f\v".
    let mut ucwords = String::with_capacity(s.len());
    let mut at_word_start = true;
    for c in s.chars() {
        ucwords.push(if at_word_start {
            c.to_ascii_uppercase()
        } else {
            c
        });
        at_word_start = matches!(c, ' ' | '\t' | '\r' | '\n' | '\u{C}' | '\u{B}');
    }
    // preg_replace('/\s+/u', '') (PHP's /u has Unicode \s).
    let squeezed: Vec<char> = ucwords.chars().filter(|c| !c.is_whitespace()).collect();
    // preg_replace('/(.)(?=[A-Z])/u', '$1_'), then mb_strtolower.
    let mut out = String::with_capacity(squeezed.len() + 4);
    for (i, c) in squeezed.iter().enumerate() {
        out.push(*c);
        if squeezed.get(i + 1).is_some_and(char::is_ascii_uppercase) {
            out.push('_');
        }
    }
    out.to_lowercase().replace('_', " ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn floats_print_like_php() {
        for (f, s) in [
            (1.0, "1"),
            (1.5, "1.5"),
            (0.1, "0.1"),
            (-0.0, "-0"),
            (1e13, "10000000000000"),
            (1e14, "1.0E+14"),
            (123_456_789_012_345.0, "1.2345678901234E+14"),
            (1.5e-5, "1.5E-5"),
            (0.0001, "0.0001"),
            (0.999_999_999_999_999, "1"),
            (1e20, "1.0E+20"),
            (1e-7, "1.0E-7"),
            (-2.5, "-2.5"),
            (1.234_567_890_123_456_7e17, "1.2345678901235E+17"),
        ] {
            assert_eq!(float_to_string(f), s, "{f}");
        }
    }

    #[test]
    fn blank_and_trim() {
        assert!(is_blank(""));
        assert!(is_blank(" \t\n\u{A0}\u{AD}\u{200B}\u{FEFF}\u{3000}"));
        assert!(!is_blank(" x "));
        assert!(!is_blank("\u{2061}x"));
        assert_eq!(trim(" \t\0x\u{B}\n"), "x");
        assert_eq!(trim("\u{C}x\u{A0}"), "\u{C}x\u{A0}");
    }

    #[test]
    fn numeric_strings() {
        for s in [
            "1", " 1e3 ", "1.", ".5", "-0", "+5", "\u{C}1", "1\u{C}", "1E-2",
        ] {
            assert!(is_numeric_str(s), "{s:?}");
        }
        for s in ["", " ", ".", "1e", "e3", "0x1A", "1 2", "--1", "inf"] {
            assert!(!is_numeric_str(s), "{s:?}");
        }
    }

    #[test]
    fn filter_var_int() {
        let ok = |v: Value, n: i64| assert_eq!(filter_int(&v), Some(n), "{v}");
        ok(json!(3), 3);
        ok(json!("3"), 3);
        ok(json!(" 4 "), 4);
        ok(json!("\u{B}4\n"), 4);
        ok(json!("+5"), 5);
        ok(json!("-0"), 0);
        ok(json!(1.0), 1);
        ok(json!(true), 1);
        ok(json!(0.999_999_999_999_999), 1);
        ok(json!("-9223372036854775808"), i64::MIN);
        for v in [
            json!("03"),
            json!("3.0"),
            json!("1e3"),
            json!(3.5),
            json!("\u{C}4"),
            json!("\u{0}4"),
            json!("1."),
            json!(false),
            json!(null),
            json!([1]),
            json!("9223372036854775808"),
            json!(1e20),
            json!(9_223_372_036_854_775_808_u64),
        ] {
            assert_eq!(filter_int(&v), None, "{v}");
        }
        assert_eq!(int_value(&json!(0.999_999_999_999_999)), Some(0));
        assert_eq!(int_value(&json!(" 7 ")), Some(7));
    }

    #[test]
    fn decimal_comparison() {
        use Ordering::*;
        let c = |s: &str, n: i64| cmp_decimal(s, n);
        assert_eq!(c("1", 1), Some(Equal));
        assert_eq!(c("0", 1), Some(Less));
        assert_eq!(c("-0", 0), Some(Equal));
        assert_eq!(c("0.5", 1), Some(Less));
        assert_eq!(c("1.", 1), Some(Equal));
        assert_eq!(c(".5", 0), Some(Greater));
        assert_eq!(c("1e3", 1), Some(Greater));
        assert_eq!(c("1.0E-7", 1), Some(Less));
        assert_eq!(c("1.0E+20", 1), Some(Greater));
        assert_eq!(c("0.99999999999999999999", 1), Some(Less));
        assert_eq!(c("1.00000000000000000001", 1), Some(Greater));
        assert_eq!(c("-1", 1), Some(Less));
        assert_eq!(c("-3", -2), Some(Less));
        assert_eq!(c("120", 120), Some(Equal));
        assert_eq!(c("1e1001", 1), None);
        assert_eq!(c("\u{C}4", 1), None);
        assert_eq!(c("abc", 1), None);
        assert_eq!(c(".", 1), None);
    }

    #[test]
    fn display_names_copy_str_snake() {
        assert_eq!(snake_words("titleEm"), "title em");
        assert_eq!(snake_words("password"), "password");
        assert_eq!(snake_words("lesson.formation.0"), "lesson.formation.0");
        assert_eq!(snake_words("original_name"), "original name");
        assert_eq!(snake_words("items.a  b.Éx"), "items.a b.éx");
        assert_eq!(snake_words("items.aB.x_y"), "items.a b.x y");
        assert_eq!(snake_words("items.AB"), "items. a b");
    }
}
