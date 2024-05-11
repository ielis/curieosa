/* 
Module with bundled CURIE -> IRI pairs.
*/

/// Embedded text file with common Prefix -> IRI mappings.
const PREFIX2IRI: &str = include_str!("curies.txt");

/// Get a `Vec`` with default `prefix` -> `IRI` mappings.
pub(crate) fn parse_default_prefix2iri() -> Vec<(&'static str, &'static str)> {
    let mut prefix2iri = Vec::new();

    for line in PREFIX2IRI.lines() {
        // `#`` as a simple comment will not work. Some CURIEs have `#` as a part.
        // However, we doctored the file to ensure `# ` represents a comment
        let end = line.find("# ").unwrap_or(line.len());
        
        let (relevant, _) = line.split_at(end);
        if relevant.is_empty() {
            continue; // a blank line
        }

        if let Some(idx) = relevant.find(':') {
            let (prefix, iri) = relevant.split_at(idx);
            let prefix = prefix.trim();

            // OK because `:` is the 0th character and we found it in the `if-let` clause.
            let iri = iri[1..].trim();

            prefix2iri.push((prefix, iri));
        }
    }

    prefix2iri
}


#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_it() {
        let mut curies = parse_default_prefix2iri();
        curies.sort();
        for (prefix, _) in curies.iter() {
            eprintln!("{prefix:?}");
        }
        assert_eq!(curies.len(), 174);

        let prefix2iri: HashMap<_, _> = curies.into_iter().collect();

        assert_eq!(prefix2iri["BT"], "http://c.biothings.io/#");
        assert_eq!(prefix2iri["HP"], "http://purl.obolibrary.org/obo/HP_");
        assert_eq!(prefix2iri["NCIT"], "http://purl.obolibrary.org/obo/NCIT_");

        assert_eq!(prefix2iri.len(), 174);
    }
}
