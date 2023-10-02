pub(crate) trait SparseIndex {
    fn sparse_index(&self) -> usize;
}

impl SparseIndex for usize {
    fn sparse_index(&self) -> usize {
        *self
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub(crate) struct SparseSet<T> {
    sparse: Vec<Option<usize>>,
    dense: Vec<T>,
}

impl<T: SparseIndex> SparseSet<T> {
    pub const fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        value.sparse_index() < self.sparse.len()
            && unsafe { self.sparse.get_unchecked(value.sparse_index()).is_some() }
    }

    pub fn insert(&mut self, value: T) -> bool {
        if self.contains(&value) {
            return false;
        }
        if value.sparse_index() >= self.sparse.len() {
            self.sparse.resize_with(value.sparse_index() + 1, || None);
        }
        *unsafe { self.sparse.get_unchecked_mut(value.sparse_index()) } = Some(self.dense.len());
        self.dense.push(value);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::SparseSet;

    #[test]
    fn sparse_set() {
        let mut sparse_set = SparseSet::<usize>::new();

        assert!(!sparse_set.contains(&19));
        assert!(sparse_set.insert(19));
        assert!(sparse_set.contains(&19));
        assert!(!sparse_set.insert(19));
        assert!(!sparse_set.contains(&13));
        assert!(sparse_set.insert(13));
        assert!(sparse_set.contains(&13));
    }
}
