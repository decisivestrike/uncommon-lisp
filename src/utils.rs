#[derive(Debug, PartialEq)]
pub enum ULispType {
    Number,
    String,
    Bool,
    Nil,

    List,
    Object,

    Identifier,
    Expression,
}

pub fn handle_escapes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some(c) => result.push(c),
                None => break,
            }
        } else {
            result.push(c);
        }
    }

    result
}
