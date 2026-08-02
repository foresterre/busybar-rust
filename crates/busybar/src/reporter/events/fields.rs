use std::fmt;

pub type Field = (String, String);

pub fn field(key: &str, value: impl fmt::Display) -> Field {
    (key.to_owned(), value.to_string())
}

pub fn prefixed(prefix: &str, fields: Vec<Field>) -> Vec<Field> {
    fields
        .into_iter()
        .map(|(key, value)| (format!("{prefix} {key}"), value))
        .collect()
}

pub fn write_fields(f: &mut fmt::Formatter<'_>, fields: &[Field]) -> fmt::Result {
    for (index, (key, value)) in fields.iter().enumerate() {
        if index > 0 {
            f.write_str("\n")?;
        }

        write!(f, "{key}: {value}")?;
    }

    Ok(())
}
