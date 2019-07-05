# hsieh-hash

This crate contains the `Hsieh Hash` or `SuperFastHash` function
created by Paul Hsieh and presented at <http://www.azillionmonkeys.com/qed/hash.html>

```rust
use hsieh_hash::digest;

let hash = digest("Hello World!".as_bytes());
assert_eq!(hash, 1774740540);
```

The hash value is initialized with the lenght of the input, so
the algorithm cannot be used incrementally.
