use busylib::{
    AccessKey, AppName, AssetPath, Brightness, ClientBuilder, Color, DeviceName, ElementId, Error,
    LogName, Opacity, Priority, StockPath, StoragePath, Text, TimeOfDay, Timestamp, TimezoneName,
    Token, Volume,
};

#[test]
fn accepts_valid_names_and_paths() {
    assert!(AppName::new("my_app-1.0").is_ok());
    assert!(ElementId::new("0").is_ok());

    assert!(AssetPath::new("nested/dir/data.png").is_ok());
    assert!(StockPath::new("shared/icon.png").is_ok());

    assert!(StoragePath::new("/ext").is_ok());
    assert!(StoragePath::new("/ext/").is_ok());
    assert!(StoragePath::new("/ext/dir/file-1.txt").is_ok());

    assert!(DeviceName::new("BUSY bar").is_ok());
    assert!(Text::new("Hello, World!").is_ok());
    assert!(LogName::new("dump-1").is_ok());
    assert!(TimezoneName::new("Europe Amsterdam").is_ok());
}

#[test]
fn rejects_invalid_names_and_paths() {
    assert!(AppName::new("").is_err());
    assert!(AppName::new("my app").is_err());
    assert!(AppName::new("my/app").is_err());

    assert!(AssetPath::new("data file.png").is_err());
    assert!(StockPath::new("icon.png").is_err());
    assert!(StockPath::new("shared/Icon.PNG").is_err());

    assert!(StoragePath::new("/etc/passwd").is_err());
    assert!(StoragePath::new("ext/file").is_err());
    assert!(StoragePath::new("/extra/file").is_err());

    assert!(DeviceName::new("").is_err());
    assert!(DeviceName::new("a".repeat(21)).is_err());
    assert!(DeviceName::new("back`tick").is_err());

    assert!(Text::new("").is_err());
    assert!(Text::new("caf\u{e9}").is_err());
    assert!(LogName::new("dump.txt").is_err());
    assert!(TimezoneName::new("1Berlin").is_err());
}

#[test]
fn describes_what_a_value_should_look_like() {
    let error = AppName::new("my app").unwrap_err();

    assert_eq!(error.label(), "application name");
    assert_eq!(
        error.to_string(),
        "invalid application name `my app`: expected one or more of [a-zA-Z0-9._-]"
    );
}

#[test]
fn validates_timestamps_against_the_api_pattern() {
    assert!(Timestamp::new("2025-10-02T14:30:45Z").is_ok());
    assert!(Timestamp::new("2025-10-02T14:30:45+02:00").is_ok());
    assert!(Timestamp::new("2025-10-02T14:30:45+0100").is_ok());
    assert!(Timestamp::new("2025-10-02T14:30:45+01").is_ok());

    assert!(Timestamp::new("2025-10-02T14:30:45").is_err());
    assert!(Timestamp::new("1999-10-02T14:30:45Z").is_err());
    assert!(Timestamp::new("2025-13-02T14:30:45Z").is_err());
    assert!(Timestamp::new("2025-10-32T14:30:45Z").is_err());
    assert!(Timestamp::new("2025-10-02T24:30:45Z").is_err());
    assert!(Timestamp::new("2025-10-02T14:60:45Z").is_err());
    assert!(Timestamp::new("2025-10-02 14:30:45Z").is_err());
    assert!(Timestamp::new("2025-10-02T14:30:45+2400").is_err());
    assert!(Timestamp::new("2025-10-02T14:30:45+01:60").is_err());
}

#[test]
fn validates_times_of_day() {
    assert!(TimeOfDay::new("00:00").is_ok());
    assert!(TimeOfDay::new("23:59").is_ok());

    assert!(TimeOfDay::new("24:00").is_err());
    assert!(TimeOfDay::new("8:00").is_err());
    assert!(TimeOfDay::new("08:60").is_err());
}

#[test]
fn validates_access_keys_without_echoing_them() {
    assert!(AccessKey::new("1234").is_ok());
    assert!(AccessKey::new("1234567890").is_ok());

    assert!(AccessKey::new("123").is_err());
    assert!(AccessKey::new("12345678901").is_err());
    assert!(AccessKey::new("12a4").is_err());

    let error = AccessKey::new("12").unwrap_err();

    assert_eq!(
        error.to_string(),
        "invalid access key `<redacted>`: expected 4 to 10 digits"
    );
    assert_eq!(
        format!("{:?}", AccessKey::new("1234").unwrap()),
        "AccessKey(<redacted>)"
    );
}

#[test]
fn keeps_tokens_out_of_debug_output() {
    let token = Token::new("example_busybar_tok").unwrap();

    assert_eq!(format!("{token:?}"), "Token(<redacted>)");

    assert!(Token::new("").is_err());
    assert!(Token::new("with space").is_err());
}

#[test]
fn bounds_percentages() {
    assert_eq!(Volume::new(100).unwrap().percent(), 100);
    assert!(Volume::new(101).is_err());

    assert_eq!(Opacity::MIN.percent(), 0);

    assert!(Priority::new(0).is_err());
    assert_eq!(Priority::MIN.percent(), 1);
    assert_eq!(Priority::MAX.percent(), 100);
}

#[test]
fn parses_volumes_reported_as_floats() {
    assert_eq!(
        serde_json::from_str::<Volume>("49.6").unwrap(),
        Volume::new(50).unwrap()
    );

    assert!(serde_json::from_str::<Volume>("120").is_err());
}

#[test]
fn parses_and_renders_brightness() {
    assert_eq!(Brightness::parse("auto").unwrap(), Brightness::Auto);
    assert_eq!(Brightness::parse("0").unwrap(), Brightness::Level(0));
    assert_eq!(Brightness::parse("100").unwrap(), Brightness::Level(100));

    assert!(Brightness::parse("101").is_err());
    assert!(Brightness::parse("").is_err());

    assert_eq!(Brightness::Auto.to_string(), "auto");
    assert_eq!(Brightness::level(42).unwrap().to_string(), "42");
    assert!(Brightness::level(200).is_err());
}

#[test]
fn parses_and_renders_colors() {
    let color = Color::parse("#aabbccdd").unwrap();

    assert_eq!(color, Color::rgba(0xaa, 0xbb, 0xcc, 0xdd));
    assert_eq!(color.to_string(), "#AABBCCDD");
    assert_eq!(Color::WHITE.to_string(), "#FFFFFFFF");
    assert_eq!(Color::TRANSPARENT.alpha, 0);

    assert!(Color::parse("aabbccdd").is_err());
    assert!(Color::parse("#aabbcc").is_err());
    assert!(Color::parse("#aabbccdz").is_err());
}

#[test]
fn accepts_owned_borrowed_and_typed_values() {
    let name = AppName::new("my_app").unwrap();

    assert!(busylib::DisplayElements::new(&name).is_ok());
    assert!(busylib::DisplayElements::new(name.clone()).is_ok());
    assert!(busylib::DisplayElements::new("my_app".to_owned()).is_ok());
    assert!(busylib::DisplayElements::new("my_app").is_ok());

    assert_eq!(name.as_str(), "my_app");
}

#[test]
fn rejects_unusable_base_urls() {
    let error = ClientBuilder::new("busy.local").unwrap_err();

    assert!(matches!(error, Error::BaseUrl { .. }));
    assert_eq!(error.to_string(), "invalid base URL `busy.local`");
    assert_eq!(
        std::error::Error::source(&error).unwrap().to_string(),
        "relative URL without a base"
    );

    assert!(ClientBuilder::new("ftp://busy.local").is_err());
    assert!(ClientBuilder::new("http://busy.local/?debug=1").is_err());
    assert!(ClientBuilder::new("http://busy.local#top").is_err());

    assert!(ClientBuilder::new("http://busy.local").is_ok());
    assert!(ClientBuilder::new("http://192.168.1.50:8080/proxy").is_ok());
    assert!(ClientBuilder::new("https://busy.local/").is_ok());
}
