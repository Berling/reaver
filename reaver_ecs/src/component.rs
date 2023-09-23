#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct Id(usize);

impl Id {
    pub(crate) const fn new(id: usize) -> Self {
        Self(id)
    }
}

impl From<Id> for usize {
    fn from(value: Id) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::component::Id;

    #[test]
    fn component_id() {
        let value: usize = Id::new(0).into();
        assert_eq!(value, 0usize);
    }
}
