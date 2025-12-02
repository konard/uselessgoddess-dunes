use crate::{Idx, Node, Tree};

/// Treap (Tree + Heap) - Randomized binary search tree
///
/// A treap maintains BST property for keys and heap property for
/// priorities. Priority is derived from the key itself (using hash)
/// to ensure deterministic behavior. Uses the `size` field in Node
/// to store priority.
pub trait Treap<T: Idx>: Tree<T> {
  /// Get priority for a node (uses size field to store priority)
  #[inline]
  fn priority(&self, idx: T) -> Option<usize> {
    self.get(idx).map(|n| n.size)
  }

  /// Set priority for a node
  #[inline]
  fn set_priority(&mut self, idx: T, priority: usize) {
    if let Some(node) = self.get(idx) {
      self.set(idx, Node { size: priority, ..node });
    }
  }

  /// Calculate priority from index (simple hash function)
  #[inline]
  fn hash_priority(&self, idx: T) -> usize {
    // Simple hash: multiply by large prime and use low bits
    let val = idx.as_usize();
    val.wrapping_mul(2654435761) >> 1
  }

  /// Rotate left maintaining heap property
  #[must_use]
  fn rotate_left(&mut self, root: T) -> Option<T> {
    let right = self.right(root)?;
    self.set_right(root, self.left(right));
    self.set_left(right, Some(root));
    Some(right)
  }

  /// Rotate right maintaining heap property
  #[must_use]
  fn rotate_right(&mut self, root: T) -> Option<T> {
    let left = self.left(root)?;
    self.set_left(root, self.right(left));
    self.set_right(left, Some(root));
    Some(left)
  }

  /// Insert with treap balancing
  fn insert_treap(&mut self, root: Option<T>, idx: T) -> Option<T> {
    if root.is_none() {
      let priority = self.hash_priority(idx);
      self.set_priority(idx, priority);
      return Some(idx);
    }

    let root_val = root?;

    if self.is_left_of(idx, root_val) {
      // Initialize priority before inserting
      if self.priority(idx).is_none() {
        let priority = self.hash_priority(idx);
        self.set_priority(idx, priority);
      }

      let new_left = self.insert_treap(self.left(root_val), idx);
      self.set_left(root_val, new_left);

      // Rotate if left child has higher priority
      if let Some(left) = new_left
        && self.priority(left)? > self.priority(root_val)?
      {
        return self.rotate_right(root_val);
      }
      Some(root_val)
    } else if self.is_right_of(idx, root_val) {
      // Initialize priority before inserting
      if self.priority(idx).is_none() {
        let priority = self.hash_priority(idx);
        self.set_priority(idx, priority);
      }

      let new_right = self.insert_treap(self.right(root_val), idx);
      self.set_right(root_val, new_right);

      // Rotate if right child has higher priority
      if let Some(right) = new_right
        && self.priority(right)? > self.priority(root_val)?
      {
        return self.rotate_left(root_val);
      }
      Some(root_val)
    } else {
      // Node already exists, don't insert duplicate
      Some(root_val)
    }
  }

  /// Remove with treap balancing
  fn remove_treap(&mut self, root: Option<T>, idx: T) -> Option<T> {
    let root_val = root?;

    if self.is_left_of(idx, root_val) {
      let new_left = self.remove_treap(self.left(root_val), idx);
      self.set_left(root_val, new_left);
      Some(root_val)
    } else if self.is_right_of(idx, root_val) {
      let new_right = self.remove_treap(self.right(root_val), idx);
      self.set_right(root_val, new_right);
      Some(root_val)
    } else {
      // Found the node to remove
      match (self.left(root_val), self.right(root_val)) {
        (None, None) => {
          self.clear(root_val);
          None
        }
        (Some(_), None) => {
          let left = self.left(root_val);
          self.clear(root_val);
          left
        }
        (None, Some(_)) => {
          let right = self.right(root_val);
          self.clear(root_val);
          right
        }
        (Some(left), Some(right)) => {
          // Rotate the child with higher priority to root
          if self.priority(left)? > self.priority(right)? {
            let new_root = self.rotate_right(root_val)?;
            let new_right = self.remove_treap(self.right(new_root), idx);
            self.set_right(new_root, new_right);
            Some(new_root)
          } else {
            let new_root = self.rotate_left(root_val)?;
            let new_left = self.remove_treap(self.left(new_root), idx);
            self.set_left(new_root, new_left);
            Some(new_root)
          }
        }
      }
    }
  }
}

/// Vector-backed Treap store
#[derive(Debug, Clone)]
pub struct TreapStore<T> {
  nodes: Vec<Node<T>>,
}

impl<T> TreapStore<T> {
  pub fn new(capacity: usize) -> Self {
    Self { nodes: (0..capacity).map(|_| Node::default()).collect() }
  }

  #[inline]
  pub fn nodes(&self) -> &[Node<T>] {
    &self.nodes
  }

  pub fn reset(&mut self) {
    for node in &mut self.nodes {
      *node = Node::default();
    }
  }
}

impl<T: Idx> Tree<T> for TreapStore<T> {
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

  fn insert(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.insert_treap(root, idx)
  }

  fn remove(&mut self, root: Option<T>, idx: T) -> Option<T> {
    self.remove_treap(root, idx)
  }
}

impl<T: Idx> Treap<T> for TreapStore<T> {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_treap_basic() {
    let mut treap: TreapStore<usize> = TreapStore::new(10);

    let mut root = None;
    root = treap.insert(root, 5);
    assert_eq!(root, Some(5));
    assert!(treap.contains(5, 5));

    root = treap.insert(root, 3);
    root = treap.insert(root, 7);
    assert!(treap.contains(root.unwrap(), 3));
    assert!(treap.contains(root.unwrap(), 5));
    assert!(treap.contains(root.unwrap(), 7));
    assert!(!treap.contains(root.unwrap(), 1));
  }

  #[test]
  fn test_treap_insert_remove() {
    let mut treap: TreapStore<usize> = TreapStore::new(100);

    let mut root = None;
    for i in 1..10 {
      root = treap.insert(root, i);
    }

    for i in 1..10 {
      assert!(treap.contains(root.unwrap(), i));
    }

    for i in 1..10 {
      root = treap.remove(root, i);
      if let Some(r) = root {
        assert!(!treap.contains(r, i));
      }
    }

    assert!(root.is_none());
  }

  #[test]
  fn test_treap_priorities() {
    let treap: TreapStore<usize> = TreapStore::new(10);

    // Test that hash_priority produces different values
    let p1 = treap.hash_priority(1);
    let p2 = treap.hash_priority(2);
    let p3 = treap.hash_priority(3);

    assert_ne!(p1, p2);
    assert_ne!(p2, p3);
    assert_ne!(p1, p3);
  }
}
