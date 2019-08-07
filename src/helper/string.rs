
/// Unquotes `s`.
pub fn unquote(s: &str) -> String {
    if s.chars().count() < 2 {
        return String::from(s);
    }

    let quote = s.chars().next().unwrap();

    if quote != '"' && quote != '\'' && quote != '`' {
        return String::from(s);
    }

    if s.chars().last().unwrap() != quote {
        return String::from(s);
    }

    String::from(&s[1..s.len() - 1])
}