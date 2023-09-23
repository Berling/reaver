use core::slice;
use std::{
    alloc::Layout,
    any::TypeId,
    fmt::{Debug, Formatter, Result},
    mem, ptr,
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Copy, Clone)]
pub(crate) struct Descriptor {
    id: Id,
    type_id: TypeId,
    layout: Layout,
    drop: Option<unsafe fn(*mut u8, usize)>,
}

impl Descriptor {
    fn typeless_drop<T>() -> Option<unsafe fn(*mut u8, usize)> {
        mem::needs_drop::<T>().then_some(|ptr, len| unsafe {
            ptr::drop_in_place(slice::from_raw_parts_mut(ptr.cast::<T>(), len));
        })
    }

    pub fn new<T: 'static>() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: Id::new(NEXT_ID.fetch_add(1, Ordering::SeqCst)),
            type_id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
            drop: Self::typeless_drop::<T>(),
        }
    }

    pub const fn id(&self) -> Id {
        self.id
    }

    pub const fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub const fn layout(&self) -> Layout {
        self.layout
    }

    pub const fn drop(&self) -> Option<unsafe fn(*mut u8, usize)> {
        self.drop
    }
}

impl Debug for Descriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Descriptor")
            .field("id", &self.id)
            .field("type_id", &self.type_id)
            .field("layout", &self.layout)
            .finish_non_exhaustive()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct Id(usize);

impl Id {
    pub const fn new(id: usize) -> Self {
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
    use std::{alloc::Layout, any::TypeId};

    use crate::component::Id;

    use super::Descriptor;

    #[test]
    fn descriptor() {
        let descriptor = Descriptor::new::<String>();
        assert_eq!(descriptor.id(), Id::new(0));
        assert_eq!(descriptor.type_id(), TypeId::of::<String>());
        assert_eq!(descriptor.layout(), Layout::new::<String>());
        assert!(descriptor.drop().is_some());
    }

    #[test]
    fn id() {
        let id: usize = Id::new(0).into();
        assert_eq!(id, 0usize);
    }
}
