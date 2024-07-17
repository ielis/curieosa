use crate::api::CuriePrefix;
use crate::curies::parse_default_prefix2iri;
use crate::{CurieParts, CurieUtil};
use std::collections::HashMap;
use std::str;
use trie_rs::{Trie, TrieBuilder};

/// A [`CurieUtil`] implementation backed by a trie.
pub struct TrieCurieUtil {
    trie: Trie<u8>,
    prefix_to_expansion: HashMap<String, String>,
    expansion_to_prefix: HashMap<String, String>,
}

impl TrieCurieUtil {
    /// Create `TrieCurieUtil` from provided Prefix to IRI mapipngs.
    ///
    /// # Examples
    ///
    /// ```
    /// use curieosa::{TrieCurieUtil, CurieUtil, CurieParts};
    ///
    /// let prefix2iri = vec![
    ///   ("GENO", "http://purl.obolibrary.org/obo/GENO_"),
    ///   ("GO", "http://purl.obolibrary.org/obo/GO_"),
    ///   ("HP", "http://purl.obolibrary.org/obo/HP_"),
    /// ];
    ///
    /// let curie_util = TrieCurieUtil::new(&prefix2iri);
    ///
    /// assert!(curie_util.has_prefix("GENO"));
    /// assert_eq!(curie_util.get_expansion("GO"), Some("http://purl.obolibrary.org/obo/GO_"));
    ///
    /// let parts: CurieParts = curie_util.get_curie_data("http://purl.obolibrary.org/obo/GO_1234567").unwrap();
    /// assert_eq!(parts.get_prefix(), "GO");
    /// assert_eq!(parts.get_id(), "1234567");
    /// ```
    pub fn new(prefix2expansion: &[(&str, &str)]) -> Self {
        let mut builder = TrieBuilder::new();
        let mut prefix_to_expansion = HashMap::with_capacity(prefix2expansion.len());
        let mut expansion_to_prefix = HashMap::with_capacity(prefix2expansion.len());
        for (prefix, expansion) in prefix2expansion {
            builder.push(expansion);
            prefix_to_expansion.insert((*prefix).to_owned(), (*expansion).to_owned());
            expansion_to_prefix.insert((*expansion).to_owned(), (*prefix).to_owned());
        }

        TrieCurieUtil {
            trie: builder.build(),
            prefix_to_expansion,
            expansion_to_prefix,
        }
    }
}

impl CurieUtil for TrieCurieUtil {
    fn get_curie_data<'cu, 'i>(&'cu self, iri: &'i str) -> Option<CurieParts<'cu, 'i>> {
        let common_prefixes = self.trie.common_prefix_search(iri);

        // TODO - What if `iri_matches` has 2 or more items? Can that even happen?
        if let Some(iri_match) = common_prefixes
            .iter()
            .map(|u8s| str::from_utf8(u8s).expect("Invalid UTF bytes!"))
            .next()
        {
            // Just get the first match.

            let prefix = &self.expansion_to_prefix[iri_match];
            let cp = match iri_match.find(prefix) {
                Some(i) => CuriePrefix::FromIri(&iri[i..i + prefix.len()]),
                None => CuriePrefix::FromCurieUtil(prefix),
            };

            return Some(CurieParts::new(cp, &iri[iri_match.len()..iri.len()]));
        }

        None
    }

    fn get_expansion(&self, prefix: &str) -> Option<&str> {
        self.prefix_to_expansion.get(prefix).map(|s| s.as_ref())
    }

    fn has_prefix(&self, prefix: &str) -> bool {
        self.prefix_to_expansion.contains_key(prefix)
    }
}

/// Create `TrieCurieUtil` from default 'prefix -> expansion' mappings.
/// 
/// # Example
/// 
/// ```
/// use curieosa::TrieCurieUtil;
/// 
/// let _cu = TrieCurieUtil::default();
/// ``` 
impl Default for TrieCurieUtil {
    fn default() -> Self {
        let prefix2iri = parse_default_prefix2iri();
        TrieCurieUtil::new(&prefix2iri)
    }
}

// TODO - implement CurieUtilBuilder?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_trie_curie_util() {
        let cu = TrieCurieUtil::default();

        macro_rules! check_curie_parts {
            ($iri: literal, $prefix: literal, $id: literal) => {
                let cp = cu.get_curie_data($iri).unwrap();
                assert_eq!(cp.get_prefix(), $prefix);
                assert_eq!(cp.get_id(), $id);
            };
        }

        check_curie_parts!("http://purl.obolibrary.org/obo/HP_0001250", "HP", "0001250");
        check_curie_parts!("http://purl.obolibrary.org/obo/OMIM_256000", "OMIM", "256000");
        check_curie_parts!("https://www.ncbi.nlm.nih.gov/snp/rs1234567", "dbSNP", "rs1234567");
    }
}
