//! Stock asset paths

crate::types::string_newtype!(
    /// Path to a stock asset shipped with the device.
    StockPath,
    "stock asset path",
    "`shared/` followed by one or more of [a-z0-9_.]",
    crate::types::validate::stock_path
);
