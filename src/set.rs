use std::{collections::{hash_map::RandomState}, hash::BuildHasher, hash::Hash, borrow::Borrow};

use crate::ScopeMap;

#[derive(Clone)]
pub struct ScopeSet<T, S: BuildHasher = RandomState> {
  map: ScopeMap<T, (), S>
}

impl<T, S: Default + BuildHasher> Default for ScopeSet<T, S> {
  #[inline]
  fn default() -> Self {
    Self {
      map: Default::default()
    }
  }
}

impl<T> ScopeSet<T, RandomState> {
  #[inline]
  pub fn new() -> Self {
    Default::default()
  }

  #[inline]
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      map: ScopeMap::with_capacity(capacity)
    }
  }
}

impl<T, S: BuildHasher> ScopeSet<T, S> {
  #[inline]
  pub fn with_hasher(hash_builder: S) -> Self {
    Self {
      map: ScopeMap::with_hasher(hash_builder)
    }
  }

  #[inline]
  pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
    Self {
      map: ScopeMap::with_capacity_and_hasher(capacity, hash_builder)
    }
  }
}

impl<T, S: BuildHasher> ScopeSet<T, S> {
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.map.is_empty()
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    self.map.capacity()
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.map.len()
  }

  #[inline]
  pub fn layer_count(&self) -> usize {
    self.map.layer_count()
  }

  #[inline]
  pub fn push_layer(&mut self) {
    self.map.push_layer()
  }

  #[inline]
  pub fn pop_layer(&mut self) -> bool {
    self.map.pop_layer()
  }
}

impl<T: Eq + Hash, S: BuildHasher> ScopeSet<T, S> {
  #[inline]
  pub fn clear_all(&mut self) {
    self.map.clear_all()
  }

  #[inline]
  pub fn clear_top(&mut self) {
    self.map.clear_top()
  }

  #[inline]
  pub fn define(&mut self, key: T) {
    self.map.define(key, ());
  }

  #[inline]
  pub fn delete(&mut self, key: T) -> bool {
    self.map.delete(key)
  }

  #[inline]
  pub fn contains<Q: ?Sized>(&self, key: &Q) -> bool
  where
    T: Borrow<Q>,
    Q: Eq + Hash,
  {
    self.map.contains_key(key)
  }

  #[inline]
  pub fn contains_at_top<Q: ?Sized>(&self, key: &Q) -> bool 
  where
    T: Borrow<Q>,
    Q: Eq + Hash,
  {
    self.map.contains_key_at_top(key)
  }
}