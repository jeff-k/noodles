//! VCF record filters.

use std::io;

use indexmap::IndexSet;

use crate::Header;

const PASS: &str = "PASS";

/// VCF record filters (`FILTER`).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Filters(IndexSet<String>);

impl Filters {
    /// Creates a PASS filter.
    pub fn pass() -> Self {
        [String::from(PASS)].into_iter().collect()
    }

    /// Returns whether this is a PASS filter.
    pub fn is_pass(&self) -> bool {
        self.0
            .first()
            .map(|filter| filter == PASS)
            .unwrap_or_default()
    }
}

impl AsRef<IndexSet<String>> for Filters {
    fn as_ref(&self) -> &IndexSet<String> {
        &self.0
    }
}

impl AsMut<IndexSet<String>> for Filters {
    fn as_mut(&mut self) -> &mut IndexSet<String> {
        &mut self.0
    }
}

impl Extend<String> for Filters {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl FromIterator<String> for Filters {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut filters = Self::default();
        filters.extend(iter);
        filters
    }
}

impl crate::variant::record::Filters for Filters {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter<'a, 'h: 'a>(
        &'a self,
        _: &'h Header,
    ) -> Box<dyn Iterator<Item = io::Result<&'a str>> + 'a> {
        Box::new(self.0.iter().map(|filter| Ok(filter.as_ref())))
    }
}

impl crate::variant::record::Filters for &Filters {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter<'a, 'h: 'a>(
        &'a self,
        _: &'h Header,
    ) -> Box<dyn Iterator<Item = io::Result<&'a str>> + 'a> {
        Box::new(self.0.iter().map(|filter| Ok(filter.as_ref())))
    }
}
