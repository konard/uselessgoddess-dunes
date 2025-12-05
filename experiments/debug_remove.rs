// Test script to reproduce the remove issue found by proptest
use trees::{SizeBalanced, Tree};

#[derive(Debug, Clone)]
struct Store<T> {
    nodes: Vec<Option<Node<T>>>,
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<usize>,
    right: Option<usize>,
    size: usize,
}

impl<T: Ord + Copy> Store<T> {
    fn new(capacity: usize) -> Self {
        Store {
            nodes: vec![None; capacity],
        }
    }

    fn alloc(&mut self, value: T) -> Option<usize> {
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if node.is_none() {
                *node = Some(Node {
                    value,
                    left: None,
                    right: None,
                    size: 1,
                });
                return Some(i);
            }
        }
        None
    }

    fn count_nodes(&self, root: Option<usize>) -> usize {
        if let Some(idx) = root {
            if let Some(node) = &self.nodes[idx] {
                1 + self.count_nodes(node.left) + self.count_nodes(node.right)
            } else {
                0
            }
        } else {
            0
        }
    }
}

impl<T: Ord + Copy> SizeBalanced<usize> for Store<T> {
    type Value = T;

    fn size(&self, idx: usize) -> Option<usize> {
        self.nodes.get(idx)?.as_ref().map(|n| n.size)
    }

    fn set_size(&mut self, idx: usize, size: usize) {
        if let Some(Some(node)) = self.nodes.get_mut(idx) {
            node.size = size;
        }
    }

    fn left(&self, idx: usize) -> Option<usize> {
        self.nodes.get(idx)?.as_ref()?.left
    }

    fn set_left(&mut self, idx: usize, left: Option<usize>) {
        if let Some(Some(node)) = self.nodes.get_mut(idx) {
            node.left = left;
        }
    }

    fn right(&self, idx: usize) -> Option<usize> {
        self.nodes.get(idx)?.as_ref()?.right
    }

    fn set_right(&mut self, idx: usize, right: Option<usize>) {
        if let Some(Some(node)) = self.nodes.get_mut(idx) {
            node.right = right;
        }
    }

    fn alloc(&mut self, value: Self::Value) -> Option<usize> {
        self.alloc(value)
    }

    fn compare(&self, a: Self::Value, b: Self::Value) -> std::cmp::Ordering {
        a.cmp(&b)
    }

    fn value(&self, idx: usize) -> Option<Self::Value> {
        self.nodes.get(idx)?.as_ref().map(|n| n.value)
    }
}

fn main() {
    // Failing test case from proptest
    let values = vec![72, 86, 54, 61, 19, 84, 1, 41, 49, 56, 97, 2, 26, 77, 11, 34, 94, 82, 92];
    let mut store: Store<usize> = Store::new(100);

    println!("Inserting {} values", values.len());
    let mut root = None;
    for &v in &values {
        println!("  Inserting {}", v);
        root = store.insert(root, v);
        if let Some(r) = root {
            println!("    Tree size: {}, actual nodes: {}", store.size(r).unwrap_or(0), store.count_nodes(root));
        }
    }

    println!("\nAfter all insertions:");
    if let Some(r) = root {
        println!("  Tree size metadata: {}", store.size(r).unwrap_or(0));
        println!("  Actual node count: {}", store.count_nodes(root));
    }

    println!("\nRemoving all values:");
    for &v in &values {
        println!("  Removing {}", v);
        root = store.remove(root, v);
        if let Some(r) = root {
            println!("    Tree size: {}, actual nodes: {}", store.size(r).unwrap_or(0), store.count_nodes(root));
        } else {
            println!("    Tree is None (empty)");
        }
    }

    println!("\nFinal state:");
    if root.is_none() {
        println!("  Tree root is None - SUCCESS");
    } else {
        println!("  Tree root is Some({}) - FAILURE!", root.unwrap());
        let actual_count = store.count_nodes(root);
        println!("  Actual nodes remaining: {}", actual_count);
        if let Some(r) = root {
            println!("  Root node: {:?}", store.nodes[r]);
        }
    }
}
