use crate::util::{ptr, Ptr};
use std::cmp::{max_by, Ordering};
use std::fmt::Debug;
use std::mem;

#[derive(Clone, Debug)]
struct VecNode<T>
    where
        T: Clone + Default,
{
    k: usize,
    entries: Vec<(u64, T)>,
    children: Option<Vec<VecNode<T>>>,
}

impl<T> VecNode<T>
    where
        T: Clone + Debug + Default,
{
    fn new(k: usize) -> Self {
        Self {
            k,
            entries: Vec::with_capacity(k),
            children: None,
        }
    }

    fn insert(&mut self, entry: (u64, T)) -> Option<((u64, T), VecNode<T>, VecNode<T>)> {
        let index = self.entries.binary_search_by(|e| entry.0.cmp(&e.0)).err().unwrap();

        if let Some(children) = &mut self.children {
            let new_child = children[index].insert(entry);
            if let Some((key, left, right)) = new_child {
                children[index] = left;
                children.insert(index + 1, right);
                self.entries.insert(index, key);
            }
        } else {
            self.entries.insert(index, entry);
        }

        if self.entries.len() == self.k {
            let m = self.entries.len() / 2;
            let mut left = VecNode::new(self.k);

            let keys_r = self.entries.split_off(m);

            let key_ret = self.entries.pop().unwrap();

            left.entries = mem::take(&mut self.entries);

            let mut right = VecNode::new(self.k);
            right.entries = keys_r;

            if let Some(children) = &mut self.children {
                let mut children = mem::take(children);
                let children_r = children.split_off(m);
                left.children = Some(children);
                right.children = Some(children_r);
            }
            Some((key_ret, left, right))
        } else {
            None
        }
    }

    fn get(&self, key: u64) -> Option<(u64, T)> {
        match self.entries.binary_search_by(|e| key.cmp(&e.0)) {
            Ok(pos) => Some(self.entries[pos].clone()),
            Err(pos) => {
                if let Some(children) = &self.children {
                    children[pos].get(key)
                } else {
                    None
                }
            }
        }
    }

    fn to_vec(&self) -> Vec<u64> {
        if let Some(children) = &self.children {
            let mut ret = vec![];
            let mut keys = self.entries.iter();
            let mut children = children.iter();
            while let Some(c) = children.next() {
                ret.extend(c.to_vec());
                if let Some(next) = keys.next() {
                    ret.push(next.0.clone())
                }
            }
            ret
        } else {
            self.entries
                .iter()
                .map(|(key, _)| key.clone())
                .collect()
        }
    }

    fn print(&self, indent: usize) {
        println!("{: >width$}{:?}", "", self.entries, width = indent);
        if let Some(children) = &self.children {
            for c in children.iter() {
                c.print(indent + 3)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct VecBPTree<T>
    where
        T: Clone + Debug + Default,
{
    k: usize,
    root: VecNode<T>,
}

impl<T> VecBPTree<T>
    where
        T: Clone + Debug + Default,
{
    pub fn new(k: usize) -> Self {
        Self {
            k,
            root: VecNode::new(k),
        }
    }

    pub fn insert(&mut self, key: u64, value: T) {
        let new_root = self.root.insert((key, value));
        if let Some((key, left, right)) = new_root {
            let mut root = VecNode::new(self.k);
            root.entries.insert(0, key);
            let mut v = Vec::with_capacity(root.k + 1);
            v.insert(0, left);
            v.insert(1, right);
            root.children = Some(v);
            self.root = root;
        }
    }

    pub fn get(&self, key: u64) -> Option<(u64, T)> {
        self.root.get(key)
    }

    pub fn to_vec(&self) -> Vec<u64> {
        self.root.to_vec()
    }

    pub fn print(&self) {
        self.root.print(0)
    }
}
