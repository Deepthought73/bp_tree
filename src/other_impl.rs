use crate::util::{ptr, Ptr};
use std::collections::LinkedList;
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Clone, Debug)]
struct LLNode<T>
where
    T: Ord + Clone,
{
    k: usize,
    keys: LinkedList<u64>,
    values: LinkedList<Ptr<T>>,
    children: Option<LinkedList<Ptr<LLNode<T>>>>,
}

impl<T> LLNode<T>
where
    T: Ord + Clone + Debug,
{
    fn new(k: usize) -> Self {
        Self {
            k,
            keys: LinkedList::new(),
            values: LinkedList::new(),
            children: None,
        }
    }

    fn insert(&mut self, key: u64, value: T) -> Option<(u64, Ptr<T>, LLNode<T>, LLNode<T>)> {
        let index = self.keys.iter().position(|it| it == &key).unwrap();

        if let Some(children) = &mut self.children {
            let child = children
                .iter()
                .enumerate()
                .find(|(i, _)| i == &index)
                .unwrap()
                .1;
            let new_child = child
                .deref()
                .borrow_mut()
                .insert(key, value);

            if let Some((key, value, left, right)) = new_child {
                *child.deref().borrow_mut() = left;
                children.insert(index + 1, ptr(right));
                self.keys.insert(index, key);
                self.values.insert(index, value);
            }
        } else {
            self.keys.insert(index, key);
            self.values.insert(index, ptr(value));
        }

        if self.keys.len() == self.k {
            let m = self.keys.len() / 2;
            let mut left = LLNode::new(self.k);
            left.keys = self.keys[..m - 1].to_vec();
            left.values = self.values[..m - 1].to_vec();

            let mut right = LLNode::new(self.k);
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

    fn get(&self, key: u64) -> Option<Ptr<T>> {
        if let Some(children) = &self.children {
            match self.keys.binary_search(&key) {
                Ok(pos) => Some(self.values[pos].clone()),
                Err(pos) => children[pos].deref().borrow().get(key),
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
                ret.extend(c.deref().borrow().to_vec());
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
                c.deref().borrow().print(indent + 3)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct LLBPTree<T>
where
    T: Ord + Clone + Debug,
{
    k: usize,
    root: LLNode<T>,
}

impl<T> LLBPTree<T>
where
    T: Ord + Clone + Debug,
{
    pub fn new(k: usize) -> Self {
        Self {
            k,
            root: LLNode::new(k),
        }
    }

    pub fn insert(&mut self, key: u64, value: T) {
        let new_root = self.root.insert(key, value);
        if let Some((key, value, left, right)) = new_root {
            let mut root = LLNode::new(self.k);
            root.keys.insert(0, key);
            root.values.insert(0, value);
            let mut v = Vec::with_capacity(root.k + 1);
            v.insert(0, ptr(left));
            v.insert(1, ptr(right));
            root.children = Some(v);
            self.root = root;
        }
    }

    pub fn get(&self, key: u64) -> Option<Ptr<T>> {
        self.root.get(key)
    }

    pub fn to_vec(&self) -> Vec<u64> {
        self.root.to_vec()
    }

    pub fn print(&self) {
        self.root.print(0)
    }
}
