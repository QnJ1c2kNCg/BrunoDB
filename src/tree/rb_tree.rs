use std::sync::{Arc, Mutex};

use super::Tree;

#[derive(Default)]
pub struct RbTree {
    root: Option<Arc<Node>>,
}

enum Color {
    Red,
    Black,
}

// TODO: Make generic
// TODO: Careful with pub's
// TODO: Make this better :)
struct Node {
    color: Color,
    left_child: Mutex<Option<Arc<Node>>>,
    right_child: Mutex<Option<Arc<Node>>>,
    pub key: u64,
    value: String,
}

impl Node {
    fn new(key: u64, value: String, color: Color) -> Self {
        Self {
            color,
            left_child: Mutex::new(None),
            right_child: Mutex::new(None),
            key,
            value,
        }
    }
}

impl RbTree {
    fn find_new_node_parent(start_node: Arc<Node>, new_node_key: u64) -> Arc<Node> {
        if new_node_key == start_node.key {
            start_node
        } else if new_node_key < start_node.key {
            // TODO: Remove unwrap
            let left_child = start_node.left_child.lock().unwrap();
            if let Some(ref left_child) = *left_child {
                Self::find_new_node_parent(Arc::clone(left_child), new_node_key)
            } else {
                // TODO: Ugly...
                drop(left_child);
                start_node
            }
        } else {
            // TODO: Remove unwrap
            let right_child = start_node.right_child.lock().unwrap();
            if let Some(ref right_child) = *right_child {
                Self::find_new_node_parent(Arc::clone(right_child), new_node_key)
            } else {
                // TODO: Ugly...
                drop(right_child);
                start_node
            }
        }
    }

    fn add_child_to_node(parent_node: Arc<Node>, new_node: Node) {
        if new_node.key == parent_node.key {
            // We are inserting an existing value, overwrite the content
            todo!()
        } else if new_node.key < parent_node.key {
            // TODO: Remove unwrap
            let mut left_child = parent_node.left_child.lock().unwrap();
            // Insert as left child
            if left_child.is_some() {
                panic!("Tried inserting a left child when one already exists")
            } else {
                *left_child = Some(Arc::new(new_node));
            }
        } else {
            // TODO: Remove unwrap
            let mut right_child = parent_node.right_child.lock().unwrap();
            // Insert as right child
            if right_child.is_some() {
                panic!("Tried inserting a left child when one already exists")
            } else {
                *right_child = Some(Arc::new(new_node));
            }
        }
    }
}

impl Tree for RbTree {
    fn insert(&mut self, key: u64, value: String) {
        if let Some(root) = &self.root {
            // Find parent for new node
            let parent_node = RbTree::find_new_node_parent(Arc::clone(root), key);
            RbTree::add_child_to_node(parent_node, Node::new(key, value, Color::Red))
        } else {
            // The tree is empty, insert the new root
            self.root = Some(Arc::new(Node::new(key, value, Color::Red)));
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::tree::Tree;

    use super::{Node, RbTree};

    impl RbTree {
        fn traverse_pre_order(node: Option<Arc<Node>>) -> String {
            if node.is_none() {
                return String::from("");
            }

            let node = node.unwrap();

            let mut s = node.key.to_string();

            let pointer_right = String::from("└──");

            let pointer_left = if node.right_child.lock().unwrap().is_some() {
                String::from("├──")
            } else {
                String::from("└──")
            };

            RbTree::traverse_node(
                &mut s,
                String::from(""),
                pointer_left,
                node.left_child.lock().unwrap().clone(),
                node.right_child.lock().unwrap().is_some(),
            );

            RbTree::traverse_node(
                &mut s,
                String::from(""),
                pointer_right,
                node.right_child.lock().unwrap().clone(),
                false,
            );

            s
        }

        fn traverse_node(
            s: &mut String,
            padding: String,
            pointer: String,
            node: Option<Arc<Node>>,
            has_right_sibling: bool,
        ) {
            if let Some(node) = node {
                s.push_str("\n");
                s.push_str(&padding);
                s.push_str(&pointer);
                s.push_str(&node.key.to_string());

                let mut new_padding = padding;
                if has_right_sibling {
                    new_padding.push_str("│  ")
                } else {
                    new_padding.push_str("   ")
                }

                let pointer_right = String::from("└──");

                let pointer_left = if node.right_child.lock().unwrap().is_some() {
                    String::from("├──")
                } else {
                    String::from("└──")
                };

                RbTree::traverse_node(
                    s,
                    new_padding.clone(),
                    pointer_left,
                    node.left_child.lock().unwrap().clone(),
                    node.right_child.lock().unwrap().is_some(),
                );
                RbTree::traverse_node(
                    s,
                    new_padding,
                    pointer_right,
                    node.right_child.lock().unwrap().clone(),
                    false,
                )
            }
        }
        fn pretty_print(&self) {
            println!("{}", RbTree::traverse_pre_order(self.root.clone()))
        }
    }

    #[test]
    fn insertion() {
        let mut tree = RbTree::default();
        for i in 50..100 {
            tree.insert(i, String::from("bruno"));
        }
        for i in 0..50 {
            tree.insert(i, String::from("bruno"));
        }
        for i in 101..150 {
            tree.insert(i, String::from("bruno"));
        }
        tree.pretty_print()
    }
}
