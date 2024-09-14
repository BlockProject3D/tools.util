// Copyright (c) 2024, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! A map with the key stored as part of the value.

use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::Arc;

/// The main index type to implement for each type to be stored in an IndexMap.
pub trait Index {
    /// The type of the key.
    type Key: ?Sized + Hash + PartialEq + Eq;

    /// The index function which returns a reference to the key stored in the object.
    fn index(&self) -> &Self::Key;
}

impl<T: Index> Index for Rc<T> {
    type Key = T::Key;

    fn index(&self) -> &Self::Key {
        (**self).index()
    }
}

impl<T: Index> Index for Arc<T> {
    type Key = T::Key;

    fn index(&self) -> &Self::Key {
        (**self).index()
    }
}

#[derive(Clone, Debug)]
struct Item<V>(V);

impl<V: Index> Hash for Item<V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.index().hash(state);
    }
}

impl<V: Index> PartialEq for Item<V> {
    fn eq(&self, other: &Self) -> bool {
        self.0.index().eq(other.0.index())
    }
}

impl<V: Index> Eq for Item<V> {}

impl<V: Index<Key=str>> Borrow<str> for Item<V> {
    fn borrow(&self) -> &V::Key {
        self.0.index()
    }
}

impl<V: Index<Key=usize>> Borrow<usize> for Item<V> {
    fn borrow(&self) -> &V::Key {
        self.0.index()
    }
}

/// The main IndexMap data-structure type.
///
/// This map type uses a [HashSet] to store the underlying items.
/// The underlying items are wrapped in a custom struct, hidden from the public API, to workaround
/// Rust broken coherence and WTF other stupid similar rules.
#[derive(Default, Clone, Debug)]
pub struct IndexMap<V>(HashSet<Item<V>>);

impl<V> IndexMap<V> {
    /// Creates a new instance of an [IndexMap].
    pub fn new() -> IndexMap<V> {
        IndexMap(HashSet::new())
    }

    /// Creates a new instance of an [IndexMap] with a given capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity`: the capacity of the new [IndexMap].
    ///
    /// returns: IndexedMap<V>
    pub fn with_capacity(capacity: usize) -> IndexMap<V> {
        Self(HashSet::with_capacity(capacity))
    }
}

impl<V: Index> IndexMap<V> {
    /// Inserts a new item in this [IndexMap]
    ///
    /// # Arguments
    ///
    /// * `value`: the value to be inserted.
    ///
    /// returns: ()
    pub fn insert(&mut self, value: V) {
        self.0.insert(Item(value));
    }

    /// Gets an element stored in this [IndexMap] from its key.
    ///
    /// # Arguments
    ///
    /// * `key`: the key of the element to look for.
    ///
    /// returns: Option<&V>
    #[allow(private_bounds)] // Because Rust is a piece of shit!!
    pub fn get(&self, key: &V::Key) -> Option<&V> where Item<V>: Borrow<V::Key> {
        self.0.get(key).map(|v| &v.0)
    }
}

impl<'a, V: Index> std::ops::Index<&'a V::Key> for IndexMap<V> where Item<V>: Borrow<V::Key> {
    type Output = V;

    fn index(&self, index: &'a V::Key) -> &Self::Output {
        self.get(index).unwrap()
    }
}
