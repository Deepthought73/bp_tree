use crate::util::{ptr, Ptr};
use std::collections::LinkedList;
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Clone, Debug)]
struct VecNode<T>
where
    T: Ord + Clone,
{
    k: usize,
    keys: Vec<u64>,
    values: Vec<T>,
    children: Option<Vec<VecNode<T>>>,
}

impl<T> VecNode<T>
where
    T: Ord + Clone + Debug,
{
    fn new(k: usize) -> Self {
        Self {
            k,
            keys: Vec::with_capacity(k),
            values: Vec::with_capacity(k),
            children: None,
        }
    }

    fn insert(&mut self, key: u64, value: T) -> Option<(u64, T, VecNode<T>, VecNode<T>)> {
        let index = self.keys.binary_search(&key).err().unwrap();

        if let Some(children) = &mut self.children {
            let new_child = children[index].insert(key, value);
            if let Some((key, value, left, right)) = new_child {
                children[index] = left;
                children.insert(index + 1, right);
                self.keys.insert(index, key);
                self.values.insert(index, value);
            }
        } else {
            self.keys.insert(index, key);
            self.values.insert(index, value);
        }

        if self.keys.len() == self.k {
            let m = self.keys.len() / 2;
            let mut left = VecNode::new(self.k);
            left.keys = self.keys[..m - 1].to_vec();
            left.values = self.values[..m - 1].to_vec();

            let mut right = VecNode::new(self.k);
            right.keys = self.keys[m..].to_vec();
            right.values = self.values[m..].to_vec();

            if let Some(children) = &self.children {
                left.children = Some(children[..m].to_vec());
                right.children = Some(children[m..].to_vec());
            }
            Some((
                self.keys[m - 1].clone(),
                self.values[m - 1].clone(),
                left,
                right,
            ))
        } else {
            None
        }
    }

    fn get(&self, key: u64) -> Option<T> {
        if let Some(children) = &self.children {
            match self.keys.binary_search(&key) {
                Ok(pos) => Some(self.values[pos].clone()),
                Err(pos) => children[pos].get(key),
            }
        } else if let Ok(pos) = self.keys.binary_search(&key) {
            Some(self.values[pos].clone())
        } else {
            None
        }
    }

    fn to_vec(&self) -> Vec<u64> {
        if let Some(children) = &self.children {
            let mut ret = vec![];
            let mut keys = self.keys.iter();
            let mut children = children.iter();
            while let Some(c) = children.next() {
                ret.extend(c.to_vec());
                if let Some(next) = keys.next() {
                    ret.push(next.clone())
                }
            }
            ret
        } else {
            self.keys.to_vec()
        }
    }

    fn print(&self, indent: usize) {
        println!("{: >width$}{:?}", "", self.keys, width = indent);
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
    T: Ord + Clone + Debug,
{
    k: usize,
    root: VecNode<T>,
}

impl<T> VecBPTree<T>
where
    T: Ord + Clone + Debug,
{
    pub fn new(k: usize) -> Self {
        Self {
            k,
            root: VecNode::new(k),
        }
    }

    pub fn insert(&mut self, key: u64, value: T) {
        let new_root = self.root.insert(key, value);
        if let Some((key, value, left, right)) = new_root {
            let mut root = VecNode::new(self.k);
            root.keys.insert(0, key);
            root.values.insert(0, value);
            let mut v = Vec::with_capacity(root.k + 1);
            v.insert(0, left);
            v.insert(1, right);
            root.children = Some(v);
            self.root = root;
        }
    }

    pub fn get(&self, key: u64) -> Option<T> {
        self.root.get(key)
    }

    pub fn to_vec(&self) -> Vec<u64> {
        self.root.to_vec()
    }

    pub fn print(&self) {
        self.root.print(0)
    }
}
