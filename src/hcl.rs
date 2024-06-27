use unicode_properties::{GeneralCategoryGroup, UnicodeGeneralCategory};

/// Escapes a string to be used as a quoted string in HCL.
pub fn escape_quoted_string(s: &str) -> String {
    // Reference implementation:
    // https://github.com/hashicorp/hcl/blob/360ae579460fab69e9939599af85c8f255a59007/hclwrite/generate.go#L350.
    let mut escaped = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '$' | '%' => {
                escaped.push(c);
                if let Some('{') = chars.peek() {
                    // Double up the template introducer symbol to escape it.
                    escaped.push(c);
                }
            }
            _ if is_go_printable(c) => escaped.push(c),
            _ if u32::from(c) < 65536 => {
                escaped.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => {
                escaped.push_str(&format!("\\U{:08x}", c as u32));
            }
        }
    }
    escaped
}

/// Reports whether the character is defined as printable by Go.
///
/// Such characters include letters, marks, numbers, punctuation, symbols, and
/// the ASCII space character, from categories L, M, N, P, S and the ASCII space
/// character.
fn is_go_printable(c: char) -> bool {
    if c == ' ' {
        return true;
    }
    let group = c.general_category_group();
    matches!(
        group,
        GeneralCategoryGroup::Letter
            | GeneralCategoryGroup::Mark
            | GeneralCategoryGroup::Number
            | GeneralCategoryGroup::Punctuation
            | GeneralCategoryGroup::Symbol
    )
}
