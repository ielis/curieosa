# curie-util

A crate for parsing Internationalized Resource Identifiers (IRIs)
into Compact Uniform Resource Identifiers (CURIEs).

Add the following into your `Cargo.toml` file to use `curie-util` in your code:

```toml
curie-util = {git = 'https://github.com/ielis/curie-util.git', tag = 'v0.0.1'}
```

**CREDIT**

The `curie-util` is a Rust port of the [curie-util](https://github.com/prefixcommons/curie-util) library 
written by Chris Mungall in Java.
The Rust port is intentionally very similar to the Java library, to simplify its usage.


## Examples

Get an implementation of [`CurieUtil`] backed by a trie with default prefix -> expansion mappings.

```rust
use curie_util::{TrieCurieUtil, CurieUtil, CurieParts};

let cu = TrieCurieUtil::default();

// Get curie parts from an IRI
let iri = "http://purl.obolibrary.org/obo/HP_0001250";
let cp: CurieParts<'_, '_> = cu.get_curie_data(iri).unwrap();

assert_eq!(cp.get_prefix(), "HP");
assert_eq!(cp.get_id(), "0001250");
assert!(false);
```

## Run tests

Run all tests by running:

```shell
cargo test
```

## Run benches

We use `criterion` to benchmark IRI parsing.

Run the following to run the bench suite:

```shell
cargo bench
```
