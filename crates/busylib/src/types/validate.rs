//! Validation rules shared by the value types

pub fn name(value: &str) -> bool {
    !value.is_empty() && value.chars().all(is_name_char)
}

pub fn asset_path(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|character| is_name_char(character) || character == '/')
}

pub fn stock_path(value: &str) -> bool {
    match value.strip_prefix("shared/") {
        Some(rest) => {
            !rest.is_empty()
                && rest.chars().all(|character| {
                    character.is_ascii_lowercase()
                        || character.is_ascii_digit()
                        || matches!(character, '_' | '.')
                })
        }
        None => false,
    }
}

pub fn storage_path(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("/ext") else {
        return false;
    };

    if rest.is_empty() {
        return true;
    }

    if !rest.starts_with('/') {
        return false;
    }

    rest.split('/')
        .skip(1)
        .all(|segment| segment.chars().all(is_name_char))
}

pub fn path_prefix(value: &str) -> bool {
    !value.is_empty()
        && value
            .split('/')
            .all(|segment| !segment.is_empty() && segment.chars().all(is_name_char))
}

pub fn device_name(value: &str) -> bool {
    let length = value.chars().count();
    (1..=20).contains(&length)
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(
                    character,
                    ' ' | '!'
                        | '('
                        | ')'
                        | '_'
                        | '='
                        | '+'
                        | ';'
                        | ':'
                        | ','
                        | '.'
                        | '?'
                        | '\''
                        | '|'
                        | '@'
                        | '#'
                        | '$'
                        | '%'
                        | '^'
                        | '&'
                        | '*'
                        | '['
                        | ']'
                        | '{'
                        | '}'
                        | '/'
                        | '\\'
                        | '"'
                        | '<'
                        | '>'
                        | '-'
                )
        })
}

pub fn printable_ascii(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
}

pub fn log_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

pub fn timezone_name(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    first.is_ascii_alphabetic()
        && value.chars().count() <= 51
        && characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, ' ' | '_' | '+' | '-')
        })
}

pub fn time_of_day(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 5 || bytes[2] != b':' {
        return false;
    }
    matches!(
        (two_digits(&bytes[..2]), two_digits(&bytes[3..])),
        (Some(hours), Some(minutes)) if hours <= 23 && minutes <= 59
    )
}

pub fn access_key(value: &str) -> bool {
    (4..=10).contains(&value.len()) && value.bytes().all(|byte| byte.is_ascii_digit())
}

pub fn timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();

    if bytes.len() < 20 || &bytes[..2] != b"20" {
        return false;
    }

    if two_digits(&bytes[2..4]).is_none() {
        return false;
    }

    if bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
    {
        return false;
    }

    let (Some(month), Some(day), Some(hour), Some(minute), Some(second)) = (
        two_digits(&bytes[5..7]),
        two_digits(&bytes[8..10]),
        two_digits(&bytes[11..13]),
        two_digits(&bytes[14..16]),
        two_digits(&bytes[17..19]),
    ) else {
        return false;
    };

    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return false;
    }

    offset(&bytes[19..])
}

fn offset(bytes: &[u8]) -> bool {
    if bytes == b"Z" {
        return true;
    }

    if !matches!(bytes.first(), Some(b'+' | b'-')) || bytes.len() < 3 {
        return false;
    }

    match two_digits(&bytes[1..3]) {
        Some(hours) if hours <= 23 => {}
        _ => return false,
    }

    match bytes.len() {
        3 => true,
        5 => two_digits(&bytes[3..5]).is_some_and(|minutes| minutes <= 59),
        6 => bytes[3] == b':' && two_digits(&bytes[4..6]).is_some_and(|minutes| minutes <= 59),
        _ => false,
    }
}

fn two_digits(bytes: &[u8]) -> Option<u8> {
    match bytes {
        [tens, ones] if tens.is_ascii_digit() && ones.is_ascii_digit() => {
            Some((tens - b'0') * 10 + (ones - b'0'))
        }
        _ => None,
    }
}

fn is_name_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
}
