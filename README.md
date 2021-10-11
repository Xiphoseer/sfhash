# sfhash

This crate contains the `SuperFastHash` (aka `Hsieh Hash`) function
presented at <http://www.azillionmonkeys.com/qed/hash.html>

```rust
use sfhash::digest;

let hash = digest("Hello World!".as_bytes());
assert_eq!(hash, 1774740540);
```

The hash value is initialized with the lenght of the input, so
the algorithm cannot be used incrementally.
