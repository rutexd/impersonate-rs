use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

/// Supported browser profiles for impersonation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Browser {
    Chrome99,
    Chrome100,
    Chrome101,
    Chrome104,
    Chrome107,
    Chrome110,
    Chrome116,
    Chrome119,
    Chrome120,
    Chrome123,
    Chrome124,
    Chrome131,
    Chrome133a,
    Chrome136,
    Chrome142,
    Chrome99Android,
    Chrome131Android,
    Edge99,
    Edge101,
    Safari15_3,
    Safari15_5,
    Safari17_0,
    Safari17_2Ios,
    Safari18_0,
    Safari18_0Ios,
    Safari18_4,
    Safari18_4Ios,
    Safari260,
    Safari260Ios,
    Safari2601,
    Firefox133,
    Firefox135,
    Firefox144,
    Tor145,
}

impl Browser {
    /// Returns the string representation expected by curl-impersonate.
    pub fn as_str(&self) -> &'static str {
        match self {
            Browser::Chrome99 => "chrome99",
            Browser::Chrome100 => "chrome100",
            Browser::Chrome101 => "chrome101",
            Browser::Chrome104 => "chrome104",
            Browser::Chrome107 => "chrome107",
            Browser::Chrome110 => "chrome110",
            Browser::Chrome116 => "chrome116",
            Browser::Chrome119 => "chrome119",
            Browser::Chrome120 => "chrome120",
            Browser::Chrome123 => "chrome123",
            Browser::Chrome124 => "chrome124",
            Browser::Chrome131 => "chrome131",
            Browser::Chrome133a => "chrome133a",
            Browser::Chrome136 => "chrome136",
            Browser::Chrome142 => "chrome142",
            Browser::Chrome99Android => "chrome99_android",
            Browser::Chrome131Android => "chrome131_android",
            Browser::Edge99 => "edge99",
            Browser::Edge101 => "edge101",
            Browser::Safari15_3 => "safari15_3",
            Browser::Safari15_5 => "safari15_5",
            Browser::Safari17_0 => "safari17_0",
            Browser::Safari17_2Ios => "safari17_2_ios",
            Browser::Safari18_0 => "safari18_0",
            Browser::Safari18_0Ios => "safari18_0_ios",
            Browser::Safari18_4 => "safari18_4",
            Browser::Safari18_4Ios => "safari18_4_ios",
            Browser::Safari260 => "safari260",
            Browser::Safari260Ios => "safari260_ios",
            Browser::Safari2601 => "safari2601",
            Browser::Firefox133 => "firefox133",
            Browser::Firefox135 => "firefox135",
            Browser::Firefox144 => "firefox144",
            Browser::Tor145 => "tor145",
        }
    }
}

/// Error returned when parsing a browser string fails.
#[derive(Debug, Error)]
#[error("Invalid browser type: {0}")]
pub struct ParseBrowserError(String);

impl FromStr for Browser {
    type Err = ParseBrowserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "chrome99" => Ok(Browser::Chrome99),
            "chrome100" => Ok(Browser::Chrome100),
            "chrome101" => Ok(Browser::Chrome101),
            "chrome104" => Ok(Browser::Chrome104),
            "chrome107" => Ok(Browser::Chrome107),
            "chrome110" => Ok(Browser::Chrome110),
            "chrome116" => Ok(Browser::Chrome116),
            "chrome119" => Ok(Browser::Chrome119),
            "chrome120" => Ok(Browser::Chrome120),
            "chrome123" => Ok(Browser::Chrome123),
            "chrome124" => Ok(Browser::Chrome124),
            "chrome131" => Ok(Browser::Chrome131),
            "chrome133a" => Ok(Browser::Chrome133a),
            "chrome136" => Ok(Browser::Chrome136),
            "chrome142" => Ok(Browser::Chrome142),
            "chrome99_android" => Ok(Browser::Chrome99Android),
            "chrome131_android" => Ok(Browser::Chrome131Android),
            "edge99" => Ok(Browser::Edge99),
            "edge101" => Ok(Browser::Edge101),
            "safari15_3" => Ok(Browser::Safari15_3),
            "safari15_5" => Ok(Browser::Safari15_5),
            "safari17_0" => Ok(Browser::Safari17_0),
            "safari17_2_ios" => Ok(Browser::Safari17_2Ios),
            "safari18_0" => Ok(Browser::Safari18_0),
            "safari18_0_ios" => Ok(Browser::Safari18_0Ios),
            "safari18_4" => Ok(Browser::Safari18_4),
            "safari18_4_ios" => Ok(Browser::Safari18_4Ios),
            "safari260" => Ok(Browser::Safari260),
            "safari260_ios" => Ok(Browser::Safari260Ios),
            "safari2601" => Ok(Browser::Safari2601),
            "firefox133" => Ok(Browser::Firefox133),
            "firefox135" => Ok(Browser::Firefox135),
            "firefox144" => Ok(Browser::Firefox144),
            "tor145" => Ok(Browser::Tor145),
            // Normalize aliases
            "chrome" => Ok(Browser::Chrome142),
            "edge" => Ok(Browser::Edge101),
            "safari" => Ok(Browser::Safari2601),
            "safari_ios" => Ok(Browser::Safari260Ios),
            "firefox" => Ok(Browser::Firefox144),
            "tor" => Ok(Browser::Tor145),
            _ => Err(ParseBrowserError(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_parsing() {
        assert_eq!(Browser::from_str("chrome100").unwrap(), Browser::Chrome100);
        assert_eq!(Browser::from_str("chrome").unwrap(), Browser::Chrome142);
        assert!(Browser::from_str("invalid").is_err());
    }

    #[test]
    fn test_as_str() {
        assert_eq!(Browser::Chrome99.as_str(), "chrome99");
        assert_eq!(Browser::Tor145.as_str(), "tor145");
    }
}
