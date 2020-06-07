use std::fmt::Display;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeMap};
use serde::de::{Deserializer, Visitor, MapAccess};

#[derive(Debug)]
pub struct ExposedPorts {
    pub port_protocol_map: HashMap<i32, Option<PortProtocol>>,
}

impl Serialize for ExposedPorts {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Debug, Serialize)]
        struct Empty {}

        let mut state = serializer.serialize_map(Some(self.port_protocol_map.len()))?;
        for (k, v) in &self.port_protocol_map {
            match v {
                Some(port_protocol) => {
                    state.serialize_entry(&format!("{}/{}", k, port_protocol), &Empty {})?;
                }
                None => {
                    state.serialize_entry(&format!("{}", k), &Empty {})?;
                }
            }
        }

        state.end()
    }
}

impl<'de> Deserialize<'de> for ExposedPorts {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ExposedPortsVisitor {})
    }
}
struct ExposedPortsVisitor;
impl<'de> Visitor<'de> for ExposedPortsVisitor {
    type Value = ExposedPorts;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: what do I put here
        formatter.write_str("TODO: idk what I put here")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let mut port_protocol_map: HashMap<i32, Option<PortProtocol>> = HashMap::new();

        while let Some((port_protocol, _)) = access.next_entry::<String, HashMap<(), ()>>()? {
            let tokens = port_protocol.split("/").collect::<Vec<&str>>();
            if tokens.len() > 1 {
                let port: i32 = tokens[0].parse().unwrap();
                let protocol = tokens[1];
                match protocol {
                    "tcp" => {
                        port_protocol_map.insert(port, Some(PortProtocol::TCP));
                    }
                    "udp" => {
                        port_protocol_map.insert(port, Some(PortProtocol::UDP));
                    }
                    _ => { /*TODO: idk lol*/ }
                }
            } else {
                let port: i32 = tokens[0].parse().unwrap();
                port_protocol_map.insert(port, None);
            }
        }

        Ok(ExposedPorts {
            port_protocol_map: port_protocol_map,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PortProtocol {
    TCP,
    UDP,
}
impl Display for PortProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut to_display = format!("{:?}", self);
        to_display.make_ascii_lowercase();
        write!(f, "{}", to_display)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::assertions::*;

    mod json {
        use super::*;

        #[test]
        fn serializes_correctly() {
            let mut port_protocol_map = HashMap::new();
            port_protocol_map.insert(11111, Some(PortProtocol::TCP));
            port_protocol_map.insert(22222, Some(PortProtocol::UDP));
            port_protocol_map.insert(33333, None);
            let exposed_ports = ExposedPorts {
                port_protocol_map: port_protocol_map,
            };

            let serialized = serde_json::to_string(&exposed_ports).unwrap();
            let possible_serializations = vec![
                r#"{"11111/tcp":{},"22222/udp":{},"33333":{}}"#,
                r#"{"11111/tcp":{},"33333":{},"22222/udp":{}}"#,
                r#"{"22222/udp":{},"11111/tcp":{},"33333":{}}"#,
                r#"{"22222/udp":{},"33333":{},"11111/tcp":{}}"#,
                r#"{"33333":{},"11111/tcp":{},"22222/udp":{}}"#,
                r#"{"33333":{},"22222/udp":{},"11111/tcp":{}}"#,
            ];

            // loop over all possible serializations because serializations for each possible ordering
            // of underlying hash map's ordering of items
            let mut was_ever_serialized_correctly = false;
            for possible_serialization in &possible_serializations {
                let result = std::panic::catch_unwind(|| {
                    assert_eq!(&serialized, &possible_serialization.to_string());
                });
                if result.is_ok() {
                    was_ever_serialized_correctly = true;
                    break;
                }
            }
            assert_eq!(was_ever_serialized_correctly, true);
        }

        #[test]
        fn deserializes_correctly() {
            let raw = r#"{"11111/tcp":{},"22222/udp":{},"33333":{}}"#;
            let exposed_ports: ExposedPorts = serde_json::from_str(&raw).unwrap();

            assert_map_len(&exposed_ports.port_protocol_map, 3);
            assert_map_contains(
                &exposed_ports.port_protocol_map,
                11111,
                Some(PortProtocol::TCP),
            );
            assert_map_contains(
                &exposed_ports.port_protocol_map,
                22222,
                Some(PortProtocol::UDP),
            );
            assert_map_contains(&exposed_ports.port_protocol_map, 33333, None);
        }
    }
}
