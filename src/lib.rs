use std::collections::HashMap;
use std::hash::Hash;

pub struct Trie<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    nodes: HashMap<K, Trie<K, V>>,
    value: Option<V>,
}

impl<K, V> Trie<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    pub fn new() -> Trie<K, V> {
        Trie {
            nodes: HashMap::new(),
            value: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn insert(&mut self, path: impl IntoIterator<Item = K>, value: V) {
        let mut iter = path.into_iter();
        self.insert_iter(&mut iter, value)
    }

    fn insert_iter(&mut self, iter: &mut dyn Iterator<Item = K>, value: V) {
        match iter.next() {
            Some(k) => self
                .nodes
                .entry(k)
                .or_insert(Trie::new())
                .insert(iter, value),
            None => {
                self.value = Some(value);
            }
        }
    }

    pub fn get(&self, path: impl IntoIterator<Item = K>) -> Option<V> {
        let mut iter = path.into_iter();
        self.get_iter(&mut iter)
    }

    fn get_iter(&self, path: &mut dyn Iterator<Item = K>) -> Option<V> {
        match path.next() {
            Some(k) => self.nodes.get(&k).and_then(|x| x.get_iter(path)),
            None => self.value.clone(),
        }
    }
}

#[macro_export]
macro_rules! trie {
    () => {
        Trie::new()
    };

    ($($key:expr => $value:expr,)+) => {
        trie!($($key => $value),+)
    };
    ($($key:expr => $value:expr),*) => {
        {
            let mut _trie = Trie::new();
            $(
                let _ = _trie.insert($key, $value);
            )*
            _trie
        }
    };
}

#[cfg(test)]
mod tests {
    use super::{trie, Trie};

    #[test]
    fn is_empty() {
        let mut trie = Trie::new();
        assert_eq!(trie.is_empty(), true);
        trie.insert(vec![1], "foo");
        assert_eq!(trie.is_empty(), false);
    }

    #[test]
    fn macro_test() {
        let trie = trie! {
            vec![1, 2] => 1,
            vec![1, 2, 3] => 2,
        };
        assert_eq!(trie.get(vec![1, 2]), Some(1));
        assert_eq!(trie.get(vec![1, 2, 3]), Some(2));
    }

    #[test]
    fn get_shallow() {
        let mut trie = Trie::new();
        trie.insert(vec![1], "foo");

        assert_eq!(trie.get(vec![1]), Some("foo"));
    }

    #[test]
    fn get_shallow_none() {
        let mut trie = Trie::new();
        trie.insert(vec![1], "foo");

        assert_eq!(trie.get(vec![2]), None);
    }

    #[test]
    fn get_deep() {
        let mut trie = Trie::new();
        trie.insert(vec![1, 2, 3], "foo");

        assert_eq!(trie.get(vec![1]), None);
        assert_eq!(trie.get(vec![1, 2]), None);
        assert_eq!(trie.get(vec![1, 2, 3]), Some("foo"));
    }

    #[test]
    fn get_deep_none() {
        let mut trie = Trie::new();
        trie.insert(vec![1, 2], "foo");

        assert_eq!(trie.get(vec![1]), None);
        assert_eq!(trie.get(vec![1, 2]), Some("foo"));
        assert_eq!(trie.get(vec![1, 2, 3]), None);
    }
}
