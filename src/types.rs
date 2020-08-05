#[cfg(feature = "decimal")]
use rust_decimal::prelude::*;

/// The precision level of all trading prices is based on significant figures. All pairs on Bitfinex use up to 5 significant digits and up to 8 decimals (e.g. 1.2345, 123.45, 1234.5, 0.00012345). Prices submit with a precision larger than 5 will be cut by the API.
#[cfg(feature = "decimal")]
pub type Price = Decimal;

/// The amount field allows up to 8 decimals. Anything exceeding this will be rounded to the 8th decimal.
#[cfg(feature = "decimal")]
pub type Amount = Decimal;



/// The precision level of all trading prices is based on significant figures. All pairs on Bitfinex use up to 5 significant digits and up to 8 decimals (e.g. 1.2345, 123.45, 1234.5, 0.00012345). Prices submit with a precision larger than 5 will be cut by the API.
#[cfg(not(feature = "decimal"))]
pub type Price = f64;

/// The amount field allows up to 8 decimals. Anything exceeding this will be rounded to the 8th decimal.
#[cfg(not(feature = "decimal"))]
pub type Amount = f64;