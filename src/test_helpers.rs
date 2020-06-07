#[cfg(test)]
pub mod assertions {
    use std::collections::HashMap;

    pub fn assert_map_len<K, V>(map: &HashMap<K, V>, expected: usize) {
        assert_eq!(map.len(), expected);
    }

    // TODO: pass in `K` or `&K`?
    // TODO: pass in `V` or `&V`?
    pub fn assert_map_contains<K, V>(map: &HashMap<K, V>, key: K, val: V)
    where
        K: std::cmp::Eq + std::hash::Hash,
        V: std::cmp::PartialEq + std::fmt::Debug,
    {
        assert_eq!(map.contains_key(&key), true);
        assert_eq!(map[&key], val);
    }
}

pub mod utils {
    use std::fs::{File, OpenOptions};
    use std::io::{Seek, Write};

    // TODO: return ref to file?
    pub fn create_temp_file_with_contents(name: &'static str, contents: &[u8]) -> File {
        let mut tmp_path = std::env::temp_dir();
        tmp_path.push("oci-image-spec-rs-tests");
        std::fs::create_dir_all(&tmp_path).unwrap();
        tmp_path.push(name);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(tmp_path)
            .unwrap();

        file.write_all(contents).unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        file
    }
}
