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

    // TODO: rewrite this somehow to not take ownership of `collection`
    pub fn assert_consists_of<S, T>(collection: S, expected_items: &[T])
    where
        S: std::iter::IntoIterator<Item = T> + std::fmt::Debug,
        T: std::cmp::PartialEq + std::fmt::Debug,
    {
        // saving this off for panic message
        let collection_string = format!("{:?}", &collection);

        let iter = collection.into_iter();
        for item in iter {
            assert!(
                expected_items.iter().find(|&x| x == &item).is_some(),
                "expected: {:?}, missing: {:?}",
                collection_string,
                &item
            );
        }
    }

    mod tests {
        use super::*;

        #[test]
        fn test_assert_consists_of() {
            assert_consists_of(vec![1, 2, 3], &vec![3, 1, 2]);
            assert_consists_of(vec!["1", "2", "3"], &vec!["3", "1", "2"]);
        }
    }
}
