use serde::de::{Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Volumes(pub Vec<String>);

impl Serialize for Volumes {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Debug, Serialize)]
        struct Empty {}

        let mut state = serializer.serialize_map(Some(self.0.len()))?;
        for volume in &self.0 {
            state.serialize_entry(&volume, &Empty {})?;
        }

        state.end()
    }
}

impl<'de> Deserialize<'de> for Volumes {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(VolumesVisitor {})
    }
}
struct VolumesVisitor;
impl<'de> Visitor<'de> for VolumesVisitor {
    type Value = Volumes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: what do I put here
        formatter.write_str("TODO: idk what I put here")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let mut volumes = Vec::new();

        while let Some((volume, _)) = access.next_entry::<String, HashMap<(), ()>>()? {
            volumes.push(volume);
        }

        Ok(Volumes(volumes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod json {
        use super::*;
        use crate::test_helpers::assertions::*;

        #[test]
        fn serializes_correctly() {
            let volumes = Volumes(vec![
                "/var/job-result-data".to_string(),
                "/var/log/my-app-logs".to_string(),
            ]);
            let serialized = serde_json::to_string(&volumes).unwrap();
            assert_eq!(
                serialized,
                r#"{"/var/job-result-data":{},"/var/log/my-app-logs":{}}"#
            );
        }

        #[test]
        fn deserializes_correctly() {
            let raw = r#"{"/var/job-result-data":{},"/var/log/my-app-logs":{}}"#;
            let volumes: Volumes = serde_json::from_str(&raw).unwrap();
            assert_eq!(volumes.0.len(), 2);
            assert_consists_of(
                volumes.0,
                &vec![
                    "/var/job-result-data".to_string(),
                    "/var/log/my-app-logs".to_string(),
                ],
            );
        }
    }
}
