use chrono::prelude::{DateTime, TimeZone, Utc};
use serde::de::Deserializer;
use serde::Deserialize;

const ISO_8601_MILLIS_FORMAT: &'static str = "%+";

pub fn de_iso8601_string<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // TODO: see how `datefmt` is imlemented here: https://github.com/serde-rs/serde/issues/1444
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, ISO_8601_MILLIS_FORMAT)
        .map_err(serde::de::Error::custom)
}

pub fn de_opt_datetime_utc<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "de_iso8601_string")] DateTime<Utc>);

    let v = Option::deserialize(deserializer)?;
    Ok(v.map(|Wrapper(a)| a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_iso8601_string() {
        #[derive(Debug, Deserialize)]
        struct Test {
            #[serde(deserialize_with = "de_iso8601_string")]
            timestamp: DateTime<Utc>,
        }

        let timestamp = Utc::now();
        let mut formatted_raw = format!(r#"{{"timestamp":"{}"}}"#, &timestamp.to_rfc3339());
        let mut obj: Test = serde_json::from_str(&formatted_raw).unwrap();
        assert_eq!(obj.timestamp, timestamp);

        // ensure that deserialization works when passing `Z` as the time zone for UTC time
        formatted_raw = format!(
            r#"{{"timestamp":"{}"}}"#,
            &timestamp.to_rfc3339_opts(chrono::prelude::SecondsFormat::AutoSi, true)
        );
        obj = serde_json::from_str(&formatted_raw).unwrap();
        assert_eq!(obj.timestamp, timestamp);

        // ensure that deserialization works with automatically-selected precision timestamps (i.e.
        // arbitrary precision)
        formatted_raw = format!(
            r#"{{"timestamp":"{}"}}"#,
            &timestamp.to_rfc3339_opts(chrono::prelude::SecondsFormat::AutoSi, false)
        );
        obj = serde_json::from_str(&formatted_raw).unwrap();
        assert_eq!(
            obj.timestamp
                .to_rfc3339_opts(chrono::prelude::SecondsFormat::AutoSi, false),
            timestamp.to_rfc3339_opts(chrono::prelude::SecondsFormat::AutoSi, false)
        );
    }

    #[test]
    fn test_de_opt_datetime_utc() {
        #[derive(Debug, Deserialize)]
        struct Test {
            #[serde(default, deserialize_with = "de_opt_datetime_utc")]
            timestamp: Option<DateTime<Utc>>,
        }

        let timestamp = Utc::now();
        let mut formatted_raw = format!(r#"{{"timestamp":"{}"}}"#, &timestamp.to_rfc3339());
        let mut obj: Test = serde_json::from_str(&formatted_raw).unwrap();
        assert_eq!(obj.timestamp.unwrap(), timestamp);

        // ensure that deserialization works when passing `Z` as the time zone for UTC time
        formatted_raw = format!(
            r#"{{"timestamp":"{}"}}"#,
            &timestamp.to_rfc3339_opts(chrono::prelude::SecondsFormat::AutoSi, true)
        );
        obj = serde_json::from_str(&formatted_raw).unwrap();
        assert_eq!(obj.timestamp.unwrap(), timestamp);

        // ensure that deserialization works with automatically-selected precision timestamps (i.e.
        // arbitrary precision)
        formatted_raw = format!(
            r#"{{"timestamp":"{}"}}"#,
            &timestamp.to_rfc3339_opts(chrono::prelude::SecondsFormat::AutoSi, false)
        );
        obj = serde_json::from_str(&formatted_raw).unwrap();
        assert_eq!(
            obj.timestamp
                .unwrap()
                .to_rfc3339_opts(chrono::prelude::SecondsFormat::AutoSi, false),
            timestamp.to_rfc3339_opts(chrono::prelude::SecondsFormat::AutoSi, false)
        );
    }
}
