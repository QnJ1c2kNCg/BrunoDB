use std::sync::{Arc, Mutex};

use super::Tree;

#[derive(Default)]
pub struct RbTree {
    root: Option<Arc<Node>>,
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Red,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn inverse(&self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

// TODO: Make generic
// TODO: Careful with pub's
// TODO: Make this better :)
// TODO: Simplify the mutex story
struct Node {
    color: Mutex<Color>,
    left_child: Mutex<Option<Arc<Node>>>,
    right_child: Mutex<Option<Arc<Node>>>,
    parent: Mutex<Option<Arc<Node>>>,
    pub key: u64,
    value: String,
}

impl Node {
    fn new(key: u64, value: String, color: Color, parent: Option<Arc<Node>>) -> Self {
        Self {
            color: Mutex::new(color),
            left_child: Mutex::new(None),
            right_child: Mutex::new(None),
            parent: Mutex::new(parent),
            key,
            value,
        }
    }

    fn is_black(&self) -> bool {
        matches!(*self.color.lock().expect("lock poisoned"), Color::Black)
    }

    fn is_red(&self) -> bool {
        !self.is_black()
    }

    fn parent(&self) -> Option<Arc<Node>> {
        self.parent.lock().expect("lock poisoned").clone()
    }

    fn set_parent(&self, node: Option<Arc<Node>>) {
        *self.parent.lock().expect("lock poisoned") = node
    }

    fn color(&self) -> Color {
        *self.color.lock().expect("lock poisoned")
    }

    fn set_color(&self, color: Color) {
        *self.color.lock().expect("lock poisoned") = color
    }

    fn child(&self, direction: Direction) -> Option<Arc<Node>> {
        match direction {
            Direction::Right => self.right_child.lock().expect("lock poisoned").clone(),
            Direction::Left => self.left_child.lock().expect("lock poisoned").clone(),
        }
    }

    fn set_child(&self, node: Option<Arc<Node>>, direction: Direction) {
        match direction {
            Direction::Right => *self.right_child.lock().expect("lock poisoned") = node,
            Direction::Left => *self.left_child.lock().expect("lock poisoned") = node,
        }
    }

    fn direction(&self) -> Direction {
        // TODO: Remove unwrap
        let parent = self.parent().unwrap();
        if let Some(left_child) = parent.child(Direction::Left) {
            if left_child.key == self.key {
                Direction::Left
            } else {
                Direction::Right
            }
        } else {
            Direction::Right
        }
    }

    fn sibling(&self) -> Option<Arc<Node>> {
        if let Some(parent) = self.parent() {
            return match self.direction() {
                Direction::Right => parent.child(Direction::Left),
                Direction::Left => parent.child(Direction::Right),
            };
        }
        None
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

    fn add_child_to_node(parent_node: Arc<Node>, new_node: Arc<Node>) {
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
                *left_child = Some(new_node);
            }
        } else {
            // TODO: Remove unwrap
            let mut right_child = parent_node.right_child.lock().unwrap();
            // Insert as right child
            if right_child.is_some() {
                panic!("Tried inserting a left child when one already exists")
            } else {
                *right_child = Some(new_node);
            }
        }
    }

    fn rotate_dir_root(&mut self, subtree_root: Arc<Node>, direction: Direction) {
        let grand_parent = subtree_root.parent();
        let s = subtree_root.child(direction.inverse()).unwrap();

        let c = s.child(direction);
        subtree_root.set_child(c.clone(), direction.inverse());
        if let Some(c) = c {
            c.set_parent(Some(subtree_root.clone()));
        }

        s.set_child(Some(subtree_root.clone()), direction);
        subtree_root.set_parent(Some(s.clone()));
        s.set_parent(grand_parent.clone());

        if let Some(grand_parent) = grand_parent {
            let dir = if grand_parent.child(Direction::Right).is_some()
                && subtree_root.key == grand_parent.child(Direction::Right).unwrap().key
            {
                Direction::Right
            } else {
                Direction::Left
            };
            grand_parent.set_child(Some(s), dir);
        } else {
            self.root = Some(s);
        }
    }
}

impl Tree for RbTree {
    fn insert(&mut self, key: u64, value: String) {
        if let Some(root) = &self.root {
            // Find parent for new node
            let mut parent = RbTree::find_new_node_parent(Arc::clone(root), key);
            let mut new_node =
                Arc::new(Node::new(key, value, Color::Red, Some(Arc::clone(&parent))));
            RbTree::add_child_to_node(Arc::clone(&parent), Arc::clone(&new_node));

            // Here we begin checking if we need to re-balance / re-color
            loop {
                if parent.is_black() {
                    // Case_I1: The parent is black
                    return;
                }

                // From now, we know that parent is red
                let grand_parent = match parent.parent() {
                    Some(grand_parent) => grand_parent,
                    None => {
                        // Case_I4: No grand parent
                        // Parent is the root of the tree, since the new node we just inserted
                        // is red, we need to recolor
                        parent.set_color(Color::Black);
                        return;
                    }
                };

                // From now, parent is red and grand parent exists
                let uncle = match parent.sibling() {
                    Some(uncle) if uncle.is_red() => uncle,
                    _ => {
                        // Case_I56: Parent is red, uncle is (considered) black

                        let p_dir = parent.direction();
                        let n_dir = new_node.direction();

                        // We want to check if the new node is an _inner_ child
                        if p_dir != n_dir {
                            // Case_I5: Parent is red, uncle is (considered) black and the new node
                            // is an inner grandchild of grand parent
                            self.rotate_dir_root(parent.clone(), p_dir);
                            new_node = parent;
                            parent = grand_parent.child(p_dir).unwrap();
                        }

                        // Case_I6: Parent is red, uncle is (considered) black and the new node
                        // is an outer grandchild of grand parent
                        self.rotate_dir_root(grand_parent.clone(), p_dir.inverse());
                        parent.set_color(Color::Black);
                        grand_parent.set_color(Color::Red);
                        return;
                    }
                };

                // Case_I2: Parent and uncle are red
                parent.set_color(Color::Black);
                uncle.set_color(Color::Black);
                grand_parent.set_color(Color::Red);

                // We now we to iterate one black level higher (=2 tree levels)
                new_node = grand_parent;
                match new_node.parent() {
                    Some(p) => parent = p,
                    None => break,
                }
            }

            // Case_I3: After the loop, the new node is now the root and red
            return;
        } else {
            // The tree is empty, insert the new root (root is black)
            self.root = Some(Arc::new(Node::new(key, value, Color::Black, None)));
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
            s.push_str(&format!(" ({:?})", &node.color()));

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
                s.push_str(&format!(" ({:?})", &node.color()));

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

    #[test]
    fn insertion2() {
        let mut tree = RbTree::default();
        tree.insert(10, String::from("bruno"));
        tree.insert(20, String::from("bruno"));
        tree.insert(30, String::from("bruno"));
        tree.insert(15, String::from("bruno"));
        tree.pretty_print()
    }
}
