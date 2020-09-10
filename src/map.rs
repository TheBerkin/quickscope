use std::{collections::{HashMap, HashSet, hash_map::RandomState}, hash::BuildHasher, hash::Hash, borrow::Borrow, borrow::BorrowMut};

use indexmap::IndexMap;
use smallvec::{SmallVec, smallvec};

pub struct ScopeMap<K, V, S: BuildHasher = RandomState> {
  map: IndexMap<K, SmallVec<[V; 1]>, S>,
  layers: SmallVec<[HashSet<usize>; 1]>,
}

impl<K, V, S: Default + BuildHasher> Default for ScopeMap<K, V, S> {
  #[inline]
  fn default() -> Self {
    Self::with_hasher(Default::default())
  }
}

impl<K, V> ScopeMap<K, V, RandomState> {
  #[inline]
  pub fn new() -> ScopeMap<K, V, RandomState> {
    Self {
      map: Default::default(),
      layers: smallvec![Default::default()]
    }
  }

  #[inline]
  pub fn with_capacity(capacity: usize) -> ScopeMap<K, V, RandomState> {
    Self::with_capacity_and_hasher(capacity, Default::default())
  }
}

impl<K, V, S: BuildHasher> ScopeMap<K, V, S> {
  #[inline]
  pub fn with_hasher(hash_builder: S) -> Self {
    Self {
      map: IndexMap::with_hasher(hash_builder),
      layers: smallvec![Default::default()],
    }
  }

  #[inline]
  pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
    Self {
      map: IndexMap::with_capacity_and_hasher(capacity, hash_builder),
      layers: smallvec![Default::default()],
    }
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    self.map.capacity()
  }
}

impl<K, V> ScopeMap<K, V> {
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.map.len()
  }

  #[inline]
  pub fn scope_count(self) -> usize {
    self.layers.len()
  }

  #[inline]
  pub fn add_layer(&mut self) {
    self.layers.push(Default::default())
  }

  #[inline]
  pub fn remove_layer(&mut self) -> bool {
    if self.layers.len() > 1 {
      for stack_index in self.layers.pop().unwrap() {
        if let Some((_key, stack)) = self.map.get_index_mut(stack_index) {
          stack.pop();
        }
      }
      return true
    }
    false
  }
}

impl<K: Eq + Hash, V> ScopeMap<K, V> {
  pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
  where 
    K: Borrow<Q>,
    Q: Eq + Hash,
  {
    self.map.get(key).and_then(|v| v.last())
  }

  pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
  where 
    K: BorrowMut<Q>,
    Q: Eq + Hash,
  {
    self.map.get_mut(key).and_then(|v| v.last_mut())
  }

  #[inline]
  pub fn insert(&mut self, key: K, value: V) {
    // Check if the key exists and is in the current layer
    if self.map.contains_key(&key) && self.layers.last().map_or(false, |layer| layer.contains(&self.map.get_index_of(&key).unwrap())) {
      // Key exists in current layer
      if let Some(top) = self.map.get_mut(&key).and_then(|stack| stack.last_mut()) {
        *top = value;
      }
    } else {
      // Key doesn't exist in current layer

      // First check that the map has a stack for this key
      let stack_index = if let Some((stack_index, _key, stack)) = self.map.get_full_mut(&key) {
        // Push the value onto the stack for the key
        stack.push(value);
        stack_index
      } else {
        // If there's no stack, make one and give back its index in the map
        self.map.insert_full(key, smallvec![value]).0
      };
      // Add the key to the key set for the current layer
      self.layers.last_mut().unwrap().insert(stack_index);
    }
  }

  #[inline]
  pub fn remove(&mut self, key: K) {
    if let Some((index, _key, stack)) = self.map.get_full_mut(&key) {
      if self.layers.last_mut().unwrap().remove(&index) {
        stack.pop();
      }
    }
  }
}
