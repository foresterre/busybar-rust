pub mod access_key;
pub mod app_name;
pub mod asset_name;
pub mod asset_path;
pub mod brightness;
pub mod color;
pub mod device_name;
pub mod element_id;
pub mod invalid_value;
pub mod log_name;
pub mod opacity;
pub mod priority;
pub mod stock_path;
pub mod storage_path;
pub mod text;
pub mod time_of_day;
pub mod timestamp;
pub mod timezone_name;
pub mod token;
pub mod try_into_value;
pub mod volume;

pub(crate) mod validate;

macro_rules! string_newtype {
    ($(#[$meta:meta])* $name:ident, $label:literal, $expectation:literal, $validate:path) => {
        $(#[$meta])*
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(
                value: impl Into<String>,
            ) -> Result<Self, $crate::types::invalid_value::InvalidValue> {
                let value = value.into();
                if $validate(&value) {
                    Ok(Self(value))
                } else {
                    Err($crate::types::invalid_value::InvalidValue::new(
                        $label,
                        value,
                        $expectation,
                    ))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, concat!(stringify!($name), "({:?})"), self.0)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::types::invalid_value::InvalidValue;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = $crate::types::invalid_value::InvalidValue;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl TryFrom<String> for $name {
            type Error = $crate::types::invalid_value::InvalidValue;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl $crate::types::try_into_value::TryIntoValue<$name> for &str {
            fn try_into_value(self) -> Result<$name, $crate::types::invalid_value::InvalidValue> {
                $name::new(self)
            }
        }

        impl $crate::types::try_into_value::TryIntoValue<$name> for String {
            fn try_into_value(self) -> Result<$name, $crate::types::invalid_value::InvalidValue> {
                $name::new(self)
            }
        }

        impl $crate::types::try_into_value::TryIntoValue<$name> for &String {
            fn try_into_value(self) -> Result<$name, $crate::types::invalid_value::InvalidValue> {
                $name::new(self.as_str())
            }
        }

        impl $crate::types::try_into_value::TryIntoValue<$name> for &$name {
            fn try_into_value(self) -> Result<$name, $crate::types::invalid_value::InvalidValue> {
                Ok(self.clone())
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                let value = <String as ::serde::Deserialize>::deserialize(deserializer)?;
                Self::new(value).map_err(<D::Error as ::serde::de::Error>::custom)
            }
        }
    };
}

macro_rules! percentage_newtype {
    ($(#[$meta:meta])* $name:ident, $label:literal, $expectation:literal, $min:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u8);

        impl $name {
            pub const MIN: Self = Self($min);
            pub const MAX: Self = Self(100);

            pub fn new(percent: u8) -> Result<Self, $crate::types::invalid_value::InvalidValue> {
                if ($min..=100).contains(&percent) {
                    Ok(Self(percent))
                } else {
                    Err($crate::types::invalid_value::InvalidValue::new(
                        $label,
                        percent.to_string(),
                        $expectation,
                    ))
                }
            }

            pub fn percent(self) -> u8 {
                self.0
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::types::invalid_value::InvalidValue;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let percent = value.parse::<u8>().map_err(|_| {
                    $crate::types::invalid_value::InvalidValue::new($label, value, $expectation)
                })?;
                Self::new(percent)
            }
        }

        impl TryFrom<u8> for $name {
            type Error = $crate::types::invalid_value::InvalidValue;

            fn try_from(percent: u8) -> Result<Self, Self::Error> {
                Self::new(percent)
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_u8(self.0)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                let percent = <f64 as ::serde::Deserialize>::deserialize(deserializer)?;
                if !(0.0..=100.0).contains(&percent) {
                    return Err(<D::Error as ::serde::de::Error>::custom(
                        $crate::types::invalid_value::InvalidValue::new(
                            $label,
                            percent.to_string(),
                            $expectation,
                        ),
                    ));
                }
                Self::new(percent.round() as u8).map_err(<D::Error as ::serde::de::Error>::custom)
            }
        }
    };
}

pub(crate) use percentage_newtype;
pub(crate) use string_newtype;
