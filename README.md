# curie-util

A crate for parsing Internationalized Resource Identifiers (IRIs) 
into Compact Uniform Resource Identifiers (CURIEs).

## Example

Get an implementation of [`CurieUtil`] backed by a trie with default prefix -> expansion mappings.

```rust
use curie_util::{TrieCurieUtil, CurieUtil, CurieParts};

let cu = TrieCurieUtil::default();

// Get curie parts from an IRI
let iri = "http://purl.obolibrary.org/obo/HP_0001250";
let cp: CurieParts<'_, '_> = cu.get_curie_data(iri).unwrap();

assert_eq!(cp.get_prefix(), "HP");
assert_eq!(cp.get_id(), "0001250");
```

## Benches

We use `criterion` to benchmark IRI parsing.
Run `cargo bench` to run the bench suite.
