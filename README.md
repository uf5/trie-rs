# Prefix tree implemented in rust.

![](https://upload.wikimedia.org/wikipedia/commons/b/be/Trie_example.svg)

A simple library that provides a prefix tree ([trie](https://en.wikipedia.org/wiki/Trie)) implementation. It uses generic types for both keys and values.

```rust
pub struct Trie<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    nodes: HashMap<K, Trie<K, V>>,
    value: Option<V>,
}
```

# Usage

```rust
use trie::{Trie, trie};

let mut t = Trie::new();
t.insert("foo".bytes(), 0.1);
assert_eq!(t.get("foo".bytes()), Some(0.1));
assert_eq!(t.get("not defined".bytes()), None);

// Trie supports reasignment.
let mut t = Trie::new();
t.insert("foo".bytes(), 1);
assert_eq!(t.get("foo".bytes()), Some(1));
t.insert("foo".bytes(), 2);
assert_eq!(t.get("foo".bytes()), Some(2));

// You can also create a Trie using the trie! macro.
let mut t = trie!();
t.insert("foobar".bytes(), 123);
t.insert("barfoo".bytes(), 456);
assert_eq!(t.get("foobar".bytes()), Some(123));
assert_eq!(t.get("barfoo".bytes()), Some(456));


// The trie! macro is also capable of creating a Trie with already assigned values.
let t = trie! {
    vec![1, 2] => "aaa",
    vec![1, 2, 3] => "bbb",
};
assert_eq!(t.get(vec![1, 2]), Some("aaa"));
assert_eq!(t.get(vec![1, 2, 3]), Some("bbb"));
```
