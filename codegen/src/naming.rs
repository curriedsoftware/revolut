use heck::{ToSnakeCase, ToUpperCamelCase};

/// Convert a schema name to a Rust type name (PascalCase).
pub fn to_type_name(name: &str) -> String {
    // Already PascalCase from OpenAPI, but normalize
    let result = name.to_upper_camel_case();
    if result.is_empty() {
        name.to_string()
    } else {
        result
    }
}

/// Keywords that cannot use the `r#` raw identifier syntax.
/// These must be suffixed with `_` instead.
const NON_RAW_KEYWORDS: &[&str] = &["self", "super", "crate", "Self"];

/// Convert a property name to a Rust field name (snake_case).
pub fn to_field_name(name: &str) -> String {
    let snake = name.to_snake_case();
    // Handle Rust keywords
    match snake.as_str() {
        // `self`, `super`, `crate` cannot be raw identifiers — suffix with `_`
        kw if NON_RAW_KEYWORDS.contains(&kw) => format!("{snake}_"),
        "type" | "ref" | "move" | "fn" | "let" | "mut" | "pub" | "mod" | "use"
        | "match" | "if" | "else" | "for" | "while" | "loop" | "break" | "continue"
        | "return" | "struct" | "enum" | "trait" | "impl" | "where" | "async" | "await"
        | "virtual" | "static" | "const" | "extern" | "in" | "as"
        | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv"
        | "try" | "typeof" | "unsafe" | "unsized" | "yield" => format!("r#{snake}"),
        _ => snake,
    }
}

/// Detect the serde rename_all convention from enum variant values.
pub fn detect_rename_all(variants: &[String]) -> Option<String> {
    if variants.is_empty() {
        return None;
    }

    // Check if all variants match a specific pattern
    let all_snake = variants.iter().all(|v| v == &v.to_snake_case());
    let all_screaming = variants
        .iter()
        .all(|v| v.chars().all(|c| c.is_uppercase() || c == '_'));
    let all_pascal = variants
        .iter()
        .all(|v| v == &v.to_upper_camel_case() && v.chars().next().is_some_and(|c| c.is_uppercase()));
    let all_camel = variants.iter().all(|v| {
        v.chars().next().is_some_and(|c| c.is_lowercase())
            && !v.contains('_')
            && v.len() > 1
            && v.chars().any(|c| c.is_uppercase())
    });
    let all_kebab = variants
        .iter()
        .all(|v| v == &v.to_lowercase().replace('_', "-") && v.contains('-'));

    if all_screaming {
        Some("SCREAMING_SNAKE_CASE".to_string())
    } else if all_snake && variants.iter().any(|v| v.contains('_')) {
        Some("snake_case".to_string())
    } else if all_pascal {
        Some("PascalCase".to_string())
    } else if all_camel {
        Some("camelCase".to_string())
    } else if all_kebab {
        Some("kebab-case".to_string())
    } else if all_snake {
        // all lowercase single words match both snake_case and PascalCase lower
        // default to snake_case
        Some("snake_case".to_string())
    } else {
        None
    }
}

/// Rust keywords that cannot be used as enum variant names even in PascalCase.
const KEYWORD_VARIANTS: &[&str] = &["Self", "Type", "Ref", "Fn", "Pub", "Mod", "Use"];

/// Map leading digit words to their English equivalents for valid Rust identifiers.
/// Inserts an underscore separator so `to_upper_camel_case` treats it as a word boundary.
fn digit_prefix_to_word(s: &str) -> String {
    // Find the leading digit(s)
    let digit_end = s
        .char_indices()
        .find(|(_, c)| !c.is_ascii_digit())
        .map(|(i, _)| i)
        .unwrap_or(s.len());
    if digit_end == 0 {
        return s.to_string();
    }
    let digits = &s[..digit_end];
    let rest = &s[digit_end..];
    let word = match digits {
        "0" => "zero",
        "1" => "one",
        "2" => "two",
        "3" => "three",
        "4" => "four",
        "5" => "five",
        "6" => "six",
        "7" => "seven",
        "8" => "eight",
        "9" => "nine",
        "10" => "ten",
        other => return format!("_{other}{rest}"),
    };
    // Underscore-separate so heck sees a word boundary
    let rest = rest.trim_start_matches('_');
    format!("{word}_{rest}")
}

/// Convert a snake_case or other format variant to PascalCase for Rust enum variant.
pub fn to_variant_name(value: &str) -> String {
    let cleaned = value.replace('-', "_");

    // Handle leading digits BEFORE camel-case conversion so word boundaries are preserved
    let cleaned = if cleaned.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        digit_prefix_to_word(&cleaned)
    } else {
        cleaned
    };

    let result = cleaned.to_upper_camel_case();
    let result = if result.is_empty() {
        value.to_string()
    } else {
        let mut chars = result.chars();
        match chars.next() {
            Some(c) => {
                let upper: String = c.to_uppercase().collect();
                format!("{upper}{}", chars.as_str())
            }
            None => result,
        }
    };

    // Handle Rust keyword conflicts for enum variants
    if KEYWORD_VARIANTS.contains(&result.as_str()) {
        format!("{result}_")
    } else {
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_rename_all() {
        assert_eq!(
            detect_rename_all(&["active".into(), "inactive".into()]),
            Some("snake_case".into())
        );
        assert_eq!(
            detect_rename_all(&["CREATED".into(), "PENDING".into(), "COMPLETED".into()]),
            Some("SCREAMING_SNAKE_CASE".into())
        );
        assert_eq!(
            detect_rename_all(&[
                "TransactionCreated".into(),
                "TransactionStateChanged".into()
            ]),
            Some("PascalCase".into())
        );
    }

    #[test]
    fn test_to_field_name() {
        assert_eq!(to_field_name("type"), "r#type");
        assert_eq!(to_field_name("virtual"), "r#virtual");
        assert_eq!(to_field_name("account_id"), "account_id");
        // `self` cannot be a raw identifier
        assert_eq!(to_field_name("self"), "self_");
        assert_eq!(to_field_name("Self"), "self_");
        assert_eq!(to_field_name("super"), "super_");
        assert_eq!(to_field_name("crate"), "crate_");
    }

    #[test]
    fn test_to_variant_name_digit_prefix() {
        assert_eq!(to_variant_name("3ds_challenge_abandoned"), "ThreeDsChallengeAbandoned");
        assert_eq!(to_variant_name("0"), "Zero");
        assert_eq!(to_variant_name("1st"), "OneSt");
    }
}
