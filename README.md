# curieosa

![crates.io](https://img.shields.io/crates/v/curieosa.svg)
![https://github.com/ielis/curieosa/actions/workflows/ci.yml](https://github.com/ielis/curieosa/actions/workflows/ci.yml/badge.svg)

A crate for parsing Internationalized Resource Identifiers (IRIs)
into Compact Uniform Resource Identifiers (CURIEs).

Add the following into your `Cargo.toml` file to use `curieosa` in your code:

```toml
curieosa = "0.1.0"
```

**CREDIT**

The `curieosa` is heavily inspired by the [curie-util](https://github.com/prefixcommons/curie-util) library 
written by Chris Mungall in Java.
The Rust port is intentionally very similar to the Java library, to simplify its usage.


## Examples

Get an implementation of [`CurieUtil`] backed by a trie with default prefix -> expansion mappings.

```rust
use curieosa::{TrieCurieUtil, CurieUtil, CurieParts};

let cu = TrieCurieUtil::default();

// Get curie parts from an IRI
let iri = "http://purl.obolibrary.org/obo/HP_0001250";
let cp: CurieParts<'_, '_> = cu.get_curie_data(iri).unwrap();

assert_eq!(cp.get_prefix(), "HP");
assert_eq!(cp.get_id(), "0001250");
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
