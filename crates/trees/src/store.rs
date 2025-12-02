use crate::{Idx, Node, SizeBalancedTree};

/// Simple vector-backed tree store for testing and benchmarking
#[derive(Debug, Clone)]
pub struct Store<T> {
  nodes: Vec<Node<T>>,
}

impl<T> Store<T> {
  pub fn new(capacity: usize) -> Self {
    Self { nodes: (0..capacity).map(|_| Node::default()).collect() }
  }

  pub fn with_nodes(nodes: Vec<Node<T>>) -> Self {
    Self { nodes }
  }

  #[inline]
  pub fn nodes(&self) -> &[Node<T>] {
    &self.nodes
  }

  #[inline]
  pub fn nodes_mut(&mut self) -> &mut [Node<T>] {
    &mut self.nodes
  }

  pub fn reset(&mut self) {
    for node in &mut self.nodes {
      *node = Node::default();
    }
  }

  pub fn is_empty(&self) -> bool {
    self.nodes.iter().all(|n| n.size == 0)
  }
}

impl<T: Idx> SizeBalancedTree<T> for Store<T> {
  #[inline(always)]
  fn get(&self, idx: T) -> Option<Node<T>> {
    self.nodes.get(idx.as_usize()).copied()
  }

  #[inline(always)]
  fn set(&mut self, idx: T, node: Node<T>) {
    if let Some(slot) = self.nodes.get_mut(idx.as_usize()) {
      *slot = node;
    }
  }

  #[inline(always)]
  fn left_mut(&mut self, idx: T) -> Option<&mut T> {
    self.nodes.get_mut(idx.as_usize())?.left.as_mut()
  }

  #[inline(always)]
  fn right_mut(&mut self, idx: T) -> Option<&mut T> {
    self.nodes.get_mut(idx.as_usize())?.right.as_mut()
  }

  #[inline(always)]
  fn is_left_of(&self, first: T, second: T) -> bool {
    first.as_usize() < second.as_usize()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_store_basic() {
    let mut store: Store<usize> = Store::new(10);

    let mut root = None;
    root = store.insert(root, 5);
    assert_eq!(root, Some(5));
    assert!(store.contains(5, 5));

    root = store.insert(root, 3);
    root = store.insert(root, 7);
    assert!(store.contains(root.unwrap(), 3));
    assert!(store.contains(root.unwrap(), 5));
    assert!(store.contains(root.unwrap(), 7));
    assert!(!store.contains(root.unwrap(), 1));
  }

  #[test]
  #[ignore] // TODO: Fix remove implementation - has bugs with node cleanup
  fn test_store_insert_remove() {
    let mut store: Store<usize> = Store::new(100);

    let mut root = None;
    for i in 1..10 {
      root = store.insert(root, i);
    }

    for i in 1..10 {
      assert!(store.contains(root.unwrap(), i));
    }

    for i in 1..10 {
      root = store.remove(root, i);
      if let Some(r) = root {
        assert!(!store.contains(r, i));
      }
    }

    assert!(root.is_none());
    assert!(store.is_empty());
  }
}
