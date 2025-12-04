use crate::{Idx, Node, Tree};

/// Size-Balanced Tree - provides operations on tree stored in slice
///
/// Extends the base Tree trait with size-balancing operations.
/// Uses subtree size to maintain balance (Chinese student's tree algorithm).
pub trait SizeBalanced<T: Idx>: Tree<T> {
  /// Get size of subtree rooted at index
  #[inline]
  fn size(&self, idx: T) -> Option<usize> {
    self.get(idx).map(|n| n.size)
  }

  /// Set size of subtree
  #[inline]
  fn set_size(&mut self, idx: T, size: usize) {
    if let Some(node) = self.get(idx) {
      self.set(idx, Node { size, ..node });
    }
  }

  /// Get size of left subtree
  #[inline]
  fn left_size(&self, idx: T) -> Option<usize> {
    self.left(idx).and_then(|idx| self.size(idx))
  }

  /// Get size of right subtree
  #[inline]
  fn right_size(&self, idx: T) -> Option<usize> {
    self.right(idx).and_then(|idx| self.size(idx))
  }

  /// Increment subtree size
  #[inline]
  fn inc_size(&mut self, idx: T) {
    if let Some(size) = self.size(idx) {
      self.set_size(idx, size + 1)
    }
  }

  /// Decrement subtree size
  #[inline]
  fn dec_size(&mut self, idx: T) {
    if let Some(size) = self.size(idx) {
      self.set_size(idx, size - 1)
    }
  }

  /// Recalculate and fix subtree size based on children
  #[inline]
  fn fix_size(&mut self, idx: T) {
    let size =
      self.left_size(idx).unwrap_or(0) + self.right_size(idx).unwrap_or(0) + 1;
    self.set_size(idx, size)
  }

  #[must_use]
  fn rotate_left(&mut self, root: T) -> Option<T> {
    let right = self.right(root)?;
    self.set_right(root, self.left(right));
    self.set_left(right, Some(root));
    self.set_size(right, self.size(root)?);
    self.fix_size(root);
    Some(right)
  }

  #[must_use]
  fn rotate_right(&mut self, root: T) -> Option<T> {
    let left = self.left(root)?;
    self.set_left(root, self.right(left));
    self.set_right(left, Some(root));
    self.set_size(left, self.size(root)?);
    self.fix_size(root);
    Some(left)
  }

  /// Insert index into tree using SBT balancing, returns new root
  fn insert_sbt(&mut self, root: Option<T>, idx: T) -> Option<T> {
    if let Some(mut root_val) = root {
      unsafe { self.insert_impl(&mut root_val, idx)? }
      Some(root_val)
    } else {
      self.set_size(idx, 1);
      Some(idx)
    }
  }

  /// Remove index from tree using SBT balancing
  ///
  /// Returns new root (None if tree empty)
  fn remove_sbt(&mut self, root: Option<T>, idx: T) -> Option<T> {
    let mut root_val = root?;
    if unsafe { self.remove_impl(&mut root_val, idx)? } {
      None
    } else {
      Some(root_val)
    }
  }

  /// Internal insert implementation using pointer for in-place updates
  ///
  /// # Safety
  ///
  /// The `root` pointer must be valid and point to a value from
  /// `left_mut` or `right_mut`. No other tree node refs allowed.
  unsafe fn insert_impl(&mut self, mut root: *mut T, idx: T) -> Option<()> {
    loop {
      if self.is_left_of(idx, *root) {
        let Some(left_ref) = self.left_mut(*root) else {
          self.inc_size(*root);
          self.set_size(idx, 1);
          self.set_left(*root, Some(idx));
          return Some(());
        };
        let left = left_ref as *mut T;

        let left_size = self.size(*left)?;
        let right_size = self.right_size(*root).unwrap_or(0);

        if self.is_left_of(idx, *left) {
          if left_size >= right_size {
            *root = self.rotate_right(*root)?;
          } else {
            self.inc_size(*root);
            root = left;
          }
        } else {
          let lr_size = self.right_size(*left).unwrap_or(0);
          if lr_size >= right_size {
            if lr_size == 0 && right_size == 0 {
              self.set_left(idx, Some(*left));
              self.set_right(idx, Some(*root));
              self.set_size(idx, left_size + 2);
              self.set_left(*root, None);
              self.set_size(*root, 1);
              *root = idx;
              return Some(());
            }
            *left = self.rotate_left(*left)?;
            *root = self.rotate_right(*root)?;
          } else {
            self.inc_size(*root);
            root = left;
          }
        }
      } else {
        let Some(right_ref) = self.right_mut(*root) else {
          self.inc_size(*root);
          self.set_size(idx, 1);
          self.set_right(*root, Some(idx));
          return Some(());
        };
        let right = right_ref as *mut T;

        let right_size = self.size(*right)?;
        let left_size = self.left_size(*root).unwrap_or(0);

        if self.is_right_of(idx, *right) {
          if right_size >= left_size {
            *root = self.rotate_left(*root)?;
          } else {
            self.inc_size(*root);
            root = right;
          }
        } else {
          let rl_size = self.left_size(*right).unwrap_or(0);
          if rl_size >= left_size {
            if rl_size == 0 && left_size == 0 {
              self.set_left(idx, Some(*root));
              self.set_right(idx, Some(*right));
              self.set_size(idx, right_size + 2);
              self.set_right(*root, None);
              self.set_size(*root, 1);
              *root = idx;
              return Some(());
            }
            *right = self.rotate_right(*right)?;
            *root = self.rotate_left(*root)?;
          } else {
            self.inc_size(*root);
            root = right;
          }
        }
      }
    }
  }

  /// Internal remove implementation - returns true if tree becomes empty
  ///
  /// Uses standard BST deletion without preventive rotations.
  /// According to SBT reference (Chen Qifeng), deletion is the same as
  /// normal BST and doesn't require maintain operations.
  ///
  /// # Safety
  ///
  /// The `root` pointer must be valid and point to a value from
  /// `left_mut` or `right_mut`. No other tree node refs allowed.
  unsafe fn remove_impl(&mut self, root: *mut T, idx: T) -> Option<bool> {
    let _original_root = *root;

    // Special case: removing the root node
    if *root == idx {
      let left = self.left(*root);
      let right = self.right(*root);

      match (left, right) {
        (None, None) => {
          // Tree will be empty - clear tree structure and size
          self.set_left(*root, None);
          self.set_right(*root, None);
          self.set_size(*root, 0);
          return Some(true);
        }
        (Some(left_child), None) => {
          *root = left_child;
          // Clear removed node's tree structure and size
          self.set_left(idx, None);
          self.set_right(idx, None);
          self.set_size(idx, 0);
          return Some(false);
        }
        (None, Some(right_child)) => {
          *root = right_child;
          // Clear removed node's tree structure and size
          self.set_left(idx, None);
          self.set_right(idx, None);
          self.set_size(idx, 0);
          return Some(false);
        }
        _ => {
          // Has two children, will be handled by remove_node
        }
      }
    }

    // Find and remove the node using standard BST deletion
    self.remove_node(root, idx)?;
    Some(false)
  }

  /// Standard BST node removal - returns true if node was found and removed
  ///
  /// # Safety
  ///
  /// The `root` pointer must be valid
  unsafe fn remove_node(&mut self, root: *mut T, idx: T) -> Option<bool> {
    if idx == *root {
      // Found the node to remove
      let left = self.left(*root);
      let right = self.right(*root);

      match (left, right) {
        (None, None) => {
          // Leaf node - parent should set their pointer to None
          // We can't do that here since we don't have access to parent
          // Just clear this node's structure
          self.set_left(*root, None);
          self.set_right(*root, None);
          self.set_size(*root, 0);
          Some(true)
        }
        (Some(left_child), None) => {
          // Replace with left child
          *root = left_child;
          self.set_left(idx, None);
          self.set_right(idx, None);
          self.set_size(idx, 0);
          Some(true)
        }
        (None, Some(right_child)) => {
          // Replace with right child
          *root = right_child;
          self.set_left(idx, None);
          self.set_right(idx, None);
          self.set_size(idx, 0);
          Some(true)
        }
        (Some(_), Some(_)) => {
          // Two children - find successor/predecessor
          let left_size = self.left_size(*root).unwrap_or(0);
          let right_size = self.right_size(*root).unwrap_or(0);

          let replacement = if left_size > right_size {
            let left_ptr = self.left_mut(*root).map(|r| r as *mut T)?;
            self.rightest(*left_ptr)
          } else {
            let right_ptr = self.right_mut(*root).map(|r| r as *mut T)?;
            self.leftest(*right_ptr)
          };

          // Remove replacement from its position
          if left_size > right_size {
            let left_ptr = self.left_mut(*root).map(|r| r as *mut T)?;
            self.remove_node(left_ptr, replacement)?;
          } else {
            let right_ptr = self.right_mut(*root).map(|r| r as *mut T)?;
            self.remove_node(right_ptr, replacement)?;
          }

          // Move replacement to current position
          let old_left = self.left(*root);
          let old_right = self.right(*root);
          self.set_left(replacement, old_left);
          self.set_right(replacement, old_right);
          self.fix_size(replacement);

          *root = replacement;

          // Clear removed node
          self.set_left(idx, None);
          self.set_right(idx, None);
          self.set_size(idx, 0);

          Some(true)
        }
      }
    } else if self.is_left_of(idx, *root) {
      // Search left subtree
      if let Some(left_ptr) = self.left_mut(*root).map(|r| r as *mut T) {
        let removed = self.remove_node(left_ptr, idx)?;
        if removed {
          self.fix_size(*root);
        }
        Some(removed)
      } else {
        Some(false) // Not found
      }
    } else {
      // Search right subtree
      if let Some(right_ptr) = self.right_mut(*root).map(|r| r as *mut T) {
        let removed = self.remove_node(right_ptr, idx)?;
        if removed {
          self.fix_size(*root);
        }
        Some(removed)
      } else {
        Some(false) // Not found
      }
    }
  }
}
