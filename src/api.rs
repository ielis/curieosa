/// `CurieUtil` can expand prefix into an expansion, check if a prefix can be expanded,
/// and find CURIE subparts in an IRI.
/// 
/// `CurieUtil` is an object safe trait.
pub trait CurieUtil {
    /// Find CURIE `prefix` and `id` in an IRI.
    ///
    /// # Example
    ///
    /// ```
    /// use curie_util::{TrieCurieUtil, CurieParts, CurieUtil};
    ///
    /// // Create curie util backed by a trie with default prefix -> IRI mappings.
    /// let curie_util = TrieCurieUtil::default();
    ///
    /// let iri = "http://purl.obolibrary.org/obo/HP_0000118";
    /// let curie_data = curie_util.get_curie_data(iri).unwrap();
    ///
    /// assert_eq!(curie_data.get_prefix(), "HP");
    /// assert_eq!(curie_data.get_id(), "0000118");
    /// ```
    fn get_curie_data<'cu, 'i>(&'cu self, iri: &'i str) -> Option<CurieParts<'cu, 'i>>;

    /// Get expansion for a prefix.
    ///
    /// # Example
    ///
    /// ```
    /// use curie_util::{TrieCurieUtil, CurieParts, CurieUtil};
    ///
    /// let curie_util = TrieCurieUtil::default();
    /// let expansion = curie_util.get_expansion("HP");
    ///
    /// assert!(expansion.is_some());
    /// assert_eq!(expansion.unwrap(), "http://purl.obolibrary.org/obo/HP_")
    /// ```
    fn get_expansion(&self, prefix: &str) -> Option<&str>;

    /// Check if Curie util has expansion for a prefix.
    ///
    /// # Example
    ///
    /// ```
    /// use curie_util::{TrieCurieUtil, CurieParts, CurieUtil};
    ///
    /// let curie_util = TrieCurieUtil::default();
    ///
    /// assert!(curie_util.has_prefix("HP"));
    /// assert!(!curie_util.has_prefix("FOO"));
    /// ```
    fn has_prefix(&self, prefix: &str) -> bool {
        self.get_expansion(prefix).is_some()
    }

    // TODO - add iterator with prefixes?
}

/// An enum pointing to the CURIE *prefix* source.
///
/// The prefix source `str` is either in the `CurieUtil` (`'cu`)
/// or in the IRI (`'i`).
#[derive(Clone, Debug)]
pub enum CuriePrefix<'cu, 'i> {
    FromCurieUtil(&'cu str),
    FromIri(&'i str),
}

/// Create a `CuriePrefix` from a `&str` assuming that `str`
/// has the usual lifetime of an IRI.
impl<'i> From<&'i str> for CuriePrefix<'_, 'i> {
    fn from(value: &'i str) -> Self {
        CuriePrefix::FromIri(value)
    }
}

/// A simple POD with references to a CURIE *prefix* that lives either
/// as long as Curie Util or as the IRI, and to a CURIE *id* that lives
/// as long as the IRI.
#[derive(Clone, Debug)]
pub struct CurieParts<'cu, 'i>
where
// 'cu: 'i,
{
    prefix: CuriePrefix<'cu, 'i>,
    id: &'i str,
}

impl<'cu, 'i> CurieParts<'cu, 'i> {
    /// Create new curie parts.
    pub fn new(prefix: CuriePrefix<'cu, 'i>, id: &'i str) -> Self {
        CurieParts { prefix, id }
    }

    /// Get slice corresponding to CURIE prefix.
    ///
    /// ```
    /// # use curie_util::{CurieParts, CuriePrefix};
    ///
    /// let cp = CuriePrefix::from("HP");
    /// let parts = CurieParts::new(cp, "1234567");
    ///
    /// assert_eq!(parts.get_prefix(), "HP");
    /// ```
    pub fn get_prefix(&self) -> &str {
        match &self.prefix {
            CuriePrefix::FromIri(a) => a,
            CuriePrefix::FromCurieUtil(a) => a,
        }
    }

    /// Get slice corresponding to CURIE id.
    ///
    /// ```
    /// # use curie_util::{CurieParts, CuriePrefix};
    ///
    /// let cp = CuriePrefix::from("HP");
    /// let parts = CurieParts::new(cp, "1234567");
    ///
    /// assert_eq!(parts.get_id(), "1234567");
    /// ```
    pub fn get_id(&self) -> &str {
        self.id
    }
}


#[cfg(test)]
mod test {
    use super::CurieUtil;

    #[test]
    fn curie_util_is_object_safe() {
        // A test that should not compile unless `CurieUtil` is object safe!
        let _a: Option<&dyn CurieUtil> = None;
    }

}
