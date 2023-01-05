pub mod rb_tree;

pub trait Tree {
    fn insert(&mut self, key: u64, value: String);
}
