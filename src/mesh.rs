
use na;

use std::marker;
use std::cmp::Ordering;

/// references
///  hedge: https://github.com/photex/hedge/blob/master/src/core/mod.rs
///  indexlist: https://github.com/steveklabnik/indexlist/blob/master/src/lib.rs


pub type Tag = usize;
pub type Offset = usize;
pub type Generation = usize;
pub type Position = na::Vector3<f64>;
pub type Normal = na::Vector3<f64>;

pub const INVALID_COMPONENT_OFFSET: Offset = 0;

/// An interface for asserting the validity of components and indices of the mesh.
pub trait IsValid {
    /// A general blanket test for validity
    fn is_valid(&self) -> bool;
}

/// Marker trait for index types.
#[derive(Default, Debug, Clone, Eq)]
pub struct Index<T> {
    pub offset: Offset,
    pub generation: Generation,
    _marker: marker::PhantomData<T>,
}

impl<T: Clone> Copy for Index<T> {}

impl<T> PartialOrd for Index<T> {
    fn partial_cmp(&self, other: &Index<T>) -> Option<Ordering> {
        // Only the offset should matter when it comes to ordering
        self.offset.partial_cmp(&other.offset)
    }
}

impl<T> PartialEq for Index<T> {
    fn eq(&self, other: &Index<T>) -> bool {
        self.offset.eq(&other.offset) && self.generation.eq(&other.generation)
    }
}

impl<T> Index<T> {
    pub fn new(offset: Offset) -> Index<T> {
        Index {
            offset,
            generation: 0,
            _marker: marker::PhantomData::default(),
        }
    }

    pub fn with_generation(offset: Offset, generation: Generation) -> Index<T> {
        Index {
            offset,
            generation,
            _marker: marker::PhantomData::default(),
        }
    }
}






