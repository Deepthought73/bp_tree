use crate::util::{array_insert, ptr, Ptr};
use std::fmt::Debug;
use std::ops::Deref;

const K: usize = 64; // K = 170 fits exactly to a 4 KiB page

// #[repr(align(4096))] // force a node to be on one page, but faster without forces alignment
#[derive(Clone, Debug)]
struct ArrayNode<T>
where
    T: Ord + Clone,
{
    size: usize,
    keys: [u64; K],
    values: [Option<Ptr<T>>; K],
    children: [Option<Ptr<ArrayNode<T>>>; K + 1],
}

impl<T> ArrayNode<T>
where
    T: Ord + Clone + Debug,
{
    fn new() -> Self {
        Self {
            size: 0,
            keys: [0; K],
            values: [(); K].map(|_| None),
            children: [(); K + 1].map(|_| None),
        }
    }

    fn insert(&mut self, key: u64, value: T) -> Option<(u64, Ptr<T>, ArrayNode<T>, ArrayNode<T>)> {
        let index = self.keys[..self.size].binary_search(&key).err().unwrap();

        if self.children[0].is_some() {
            let new_child = self.children[index]
                .as_ref()
                .unwrap()
                .deref()
                .borrow_mut()
                .insert(key, value);
            if let Some((key, value, left, right)) = new_child {
                self.children[index] = Some(ptr(left));
                array_insert(
                    &mut self.children,
                    index + 1,
                    Some(ptr(right)),
                    self.size + 1,
                );
                array_insert(&mut self.keys, index, key, self.size);
                array_insert(&mut self.values, index, Some(value), self.size);
                self.size += 1;
            }
        } else {
            array_insert(&mut self.keys, index, key, self.size);
            array_insert(&mut self.values, index, Some(ptr(value)), self.size);
            self.size += 1;
        }

        if self.size == K {
            let m = self.size / 2;
            let mut left = ArrayNode::new();
            left.size = m - 1;
            left.keys[..left.size].copy_from_slice(&self.keys[..m - 1]);
            left.values[..left.size].clone_from_slice(&self.values[..m - 1]);

            let mut right = ArrayNode::new();
            right.size = K - m;
            right.keys[..right.size].copy_from_slice(&self.keys[m..]);
            right.values[..right.size].clone_from_slice(&self.values[m..]);

            if self.children[0].is_some() {
                left.children[..=left.size].clone_from_slice(&self.children[..m]);
                right.children[..=right.size].clone_from_slice(&self.children[m..]);
            }
            Some((
                self.keys[m - 1].clone(),
                self.values[m - 1].clone().unwrap(),
                left,
                right,
            ))
        } else {
            None
        }
    }

    fn get(&self, key: u64) -> Option<Ptr<T>> {
        if self.children[0].is_some() {
            match self.keys[..self.size].binary_search(&key) {
                Ok(pos) => Some(self.values[pos].as_ref().unwrap().clone()),
                Err(pos) => self.children[pos]
                    .as_ref()
                    .unwrap()
                    .deref()
                    .borrow()
                    .get(key),
            }
        } else if let Ok(pos) = self.keys[..self.size].binary_search(&key) {
            Some(self.values[pos].as_ref().unwrap().clone())
        } else {
            None
        }
    }

    fn to_vec(&self) -> Vec<u64> {
        if self.children[0].is_some() {
            let mut ret = vec![];
            let mut keys = self.keys[..self.size].iter();
            let mut children = self.children.iter();
            while let Some(Some(c)) = children.next() {
                ret.extend(c.deref().borrow().to_vec());
                if let Some(next) = keys.next() {
                    ret.push(next.clone())
                }
            }
            ret
        } else {
            self.keys[..self.size].to_vec()
        }
    }

    fn print(&self, indent: usize) {
        println!(
            "{: >width$}{:?}",
            "",
            &self.keys[..self.size],
            width = indent
        );
        for c in self.children.iter() {
            if let Some(c) = c {
                c.deref().borrow().print(indent + 3)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ArrayBPTree<T>
where
    T: Ord + Clone + Debug,
{
    root: ArrayNode<T>,
}

impl<T> ArrayBPTree<T>
where
    T: Ord + Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            root: ArrayNode::new(),
        }
    }

    pub fn insert(&mut self, key: u64, value: T) {
        let new_root = self.root.insert(key, value);
        if let Some((key, value, left, right)) = new_root {
            let mut root = ArrayNode::new();
            root.keys[0] = key;
            root.values[0] = Some(value);
            root.children[0] = Some(ptr(left));
            root.children[1] = Some(ptr(right));
            root.size = 1;
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
