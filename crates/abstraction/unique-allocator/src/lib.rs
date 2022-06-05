#![feature(box_syntax)]
mod internal;
mod pool;
mod ptr;

pub use ptr::{Intern, InternedPtr};

use std::{borrow::Borrow, fmt::Debug, hash::Hash, marker::PhantomData};
use sync_utils::ARwLock;

use internal::UniqueAllocatorInternal;

pub struct UniqueAllocator<T, Owned = T, Ptr = InternedPtr<T>>
where
    T: Hash + Eq + 'static + ?Sized,
    Ptr: Intern<Thing = T>,
    Owned: Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
{
    internal: ARwLock<UniqueAllocatorInternal<T, Owned, Ptr>>,
    phantom: PhantomData<T>,
}

impl<T, Owned, Id> Clone for UniqueAllocator<T, Owned, Id>
where
    T: Hash + Eq + 'static + ?Sized,
    Id: Intern<Thing = T>,
    Owned: Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
{
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T, Owned, Ptr> UniqueAllocator<T, Owned, Ptr>
where
    T: Hash + Eq + 'static + ?Sized,
    Ptr: Intern<Thing = T>,
    Owned: Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
{
    pub fn empty() -> Self {
        Self {
            internal: ARwLock::new(UniqueAllocatorInternal::default()),
            phantom: PhantomData,
        }
    }

    pub fn new_from<I: 'static>(ids: &[I]) -> Self
    where
        Ptr: for<'a> From<&'a I>,
    {
        Self {
            internal: ARwLock::new(UniqueAllocatorInternal::new_from(ids)),
            phantom: PhantomData,
        }
    }

    pub fn new(ids: &[Ptr]) -> Self {
        Self {
            internal: ARwLock::new(UniqueAllocatorInternal::new(ids)),
            phantom: PhantomData,
        }
    }

    pub fn alloc(&self, owned: Owned) -> Ptr
    where
        T: Debug,
    {
        let result = match self
            .internal
            .read(|internal| internal.ids.get(owned.borrow()).map(|id| *id))
        {
            Some(id) => id,
            None => {
                self.internal
                    .write(|internal| match internal.ids.get(owned.borrow()) {
                        Some(id) => *id, // this step is lest the value has changed
                        None => {
                            let owned: &Owned = unsafe { &*internal.things.alloc(owned) };
                            let ptr: *const T = owned.borrow();
                            let id: Ptr = unsafe { &*ptr }.into();
                            internal.ids.insert(owned.clone(), id);
                            id
                        }
                    })
            }
        };
        return result;
    }

    pub fn alloc_from_ref(&self, t: &T) -> Ptr {
        let result = match self
            .internal
            .read(|internal| internal.ids.get(t).map(|id| *id))
        {
            Some(id) => id,
            None => {
                self.internal.write(|internal| match internal.ids.get(t) {
                    Some(id) => *id, // this step is lest the value has changed
                    None => {
                        let owned: &Owned = unsafe { &*internal.things.alloc(t.into()) };
                        let ptr: *const T = owned.borrow();
                        let id = unsafe { &*ptr }.into();
                        internal.ids.insert(owned.clone(), id);
                        id
                    }
                })
            }
        };
        return result;
    }

    pub fn id_iter(&self) -> impl Iterator<Item = Ptr> {
        self.internal
            .read(|internal| internal.id_iter().collect::<Vec<Ptr>>())
            .into_iter()
    }
}
