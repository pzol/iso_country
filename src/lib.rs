// What is ISO 3166-1?
//
// | ISO 3166-1 is part of the ISO 3166 standard published by the International
// | Organization for Standardization (ISO), and defines codes for the names of
// | countries, dependent territories, and special areas of geographical
// | interest.
// |
// | - [Wikipedia](http://en.wikipedia.org/wiki/ISO_3166-1)

#[cfg(feature = "serde")]
extern crate serde;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::{ fmt, str };
use std::error::Error;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/isodata.rs"));
include!(concat!(env!("OUT_DIR"), "/data.rs"));

#[derive(Debug)]
pub enum CountryParseError {
    InvalidCountryCode(String)
}

impl Error for CountryParseError {
    fn description(&self) -> &str { "error parsing country code" }
}

impl fmt::Display for CountryParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl str::FromStr for Country {
    type Err = CountryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match COUNTRY_CODE_SEARCH_TABLE.binary_search_by(|&(o, _)| o.cmp(s)) {
            Ok(pos) => Ok(COUNTRY_CODE_SEARCH_TABLE[pos].1),
            Err(_)  => Err(CountryParseError::InvalidCountryCode(s.to_string()))
        }
    }
}

impl fmt::Display for Country  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Country::Unspecified => Ok(()),
            _ => fmt::Debug::fmt(self, f)
        }
    }
}

lazy_static! {
    static ref INVERTED_COUNTRY_CODES: HashMap<Country, &'static str> = {
        let mut codes = HashMap::new();

        for &(country_name, country_code) in COUNTRY_CODE_SEARCH_TABLE {
            codes.insert(country_code, country_name);
        }

        codes
    };
}

#[cfg(feature = "serde")]
impl serde::Serialize for Country {
   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
   {
       let country_name = INVERTED_COUNTRY_CODES.get(self)
           .ok_or_else(|| serde::ser::Error::custom("Impossible, since all variants have their country name"))?;

       serializer.serialize_str(country_name)
   }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Country {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
      use serde::de::Visitor;
      use serde::de::Unexpected;
      use std::fmt;
      use std::str::FromStr;
      struct CountryVisitor;

      impl <'de> Visitor<'de> for CountryVisitor {
            type Value = Country;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                  formatter.write_str("valid 2 letter country code")
            }

            fn visit_str<E>(self, value: &str) -> Result<Country, E> where E: serde::de::Error {
                  match Country::from_str(value) {
                        Ok(country) => Ok(country),
                        Err(_) => Err(E::invalid_value(Unexpected::Str(value), &"2 letter country code")),
                  }
            }
      }

      deserializer.deserialize_str(CountryVisitor)
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use super::Country;

    macro_rules! assert_s {
        ($expr:expr) => ({
            let c : Country = $expr.parse().unwrap();
            assert_eq!($expr, c.to_string());
        })
    }

    #[test]
    fn from_to_str() {
        assert_s!("ZW");
        assert_s!("PL");
        assert_s!("");
    }

    #[test]
    fn name() {
        assert_eq!("Poland", Country::PL.name());
        assert_eq!("", Country::Unspecified.name());
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serializes() {
        assert_eq!(&serde_json::to_string(&Country::RU).unwrap(), "\"RU\"");
        assert_eq!(&serde_json::to_string(&Country::PL).unwrap(), "\"PL\"");
        assert_eq!(&serde_json::to_string(&Country::ES).unwrap(), "\"ES\"");
        assert_eq!(&serde_json::to_string(&Country::IT).unwrap(), "\"IT\"");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn deserializes() {
        assert_eq!(Country::RU, serde_json::from_slice(b"\"RU\"").unwrap());
        assert_eq!(Country::PL, serde_json::from_slice(b"\"PL\"").unwrap());
        assert_eq!(Country::ES, serde_json::from_slice(b"\"ES\"").unwrap());
        assert_eq!(Country::IT, serde_json::from_slice(b"\"IT\"").unwrap());
    }
}
