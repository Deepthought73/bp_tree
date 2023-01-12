use rand::{thread_rng, RngCore};
use std::cell::RefCell;
use std::mem::size_of;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Instant;

type Ptr<T> = Rc<RefCell<T>>;
type NodePtr<T> = Ptr<Node<T>>;

fn ptr<T>(a: T) -> Ptr<T> {
    Rc::new(RefCell::new(a))
}

fn find_position<T>(list: &[T], item: &T, size: usize) -> usize
    where
        T: Ord,
{
    if size == 0 {
        0
    } else {
        let mut k = size / 2;
        let mut i = k;
        if item < &list[0] {
            0
        } else if item > list.last().unwrap() {
            size
        } else {
            while &list[i] > item || &list[i + 1] < item {
                if k > 1 {
                    k /= 2;
                }
                if &list[i] < item {
                    i += k;
                } else {
                    i -= k;
                }
            }
            i + 1
        }
    }
}

fn array_insert<T>(array: &mut [T], index: usize, value: T, size: usize) {
    if index >= size {
        array[size] = value;
    } else {
        array[index..].rotate_right(1);
        array[index] = value;
    }
}

const K: usize = 30; // K = 170 fits exactly to a 4 KiB page

// #[repr(align(4096))] // force a node to be on one page, but faster without forces alignment
#[derive(Clone, Debug)]
struct Node<T>
    where
        T: Ord + Clone,
{
    size: usize,
    keys: [u64; K],
    values: [Option<Ptr<T>>; K],
    children: [Option<NodePtr<T>>; K + 1],
}

impl<T> Node<T>
    where
        T: Ord + Clone,
{
    fn new() -> Self {
        Self {
            size: 0,
            keys: [0; K],
            values: [(); K].map(|_| None),
            children: [(); K + 1].map(|_| None),
        }
    }

    fn insert(&mut self, key: u64, value: T) -> Option<(u64, Node<T>, Node<T>)> {
        let index = find_position(&self.keys, &key, self.size);

        if self.children[0].is_some() {
            let new_child = self.children[index]
                .as_ref()
                .unwrap()
                .deref()
                .borrow_mut()
                .insert(key, value);
            if let Some((key, left, right)) = new_child {
                self.children[index] = Some(ptr(left));
                array_insert(
                    &mut self.children,
                    index + 1,
                    Some(ptr(right)),
                    self.size + 1,
                );
                array_insert(&mut self.keys, index, key, self.size);
                self.size += 1;
            }
        } else {
            array_insert(&mut self.keys, index, key, self.size);
            array_insert(&mut self.values, index, Some(ptr(value)), self.size);
            self.size += 1;
        }

        if self.size == K {
            let m = self.size / 2;
            let mut left = Node::new();
            left.size = m - 1;
            left.keys[..left.size].copy_from_slice(&self.keys[..m - 1]);

            let mut right = Node::new();
            right.size = K - m;
            right.keys[..right.size].copy_from_slice(&self.keys[m..]);

            if self.children[0].is_some() {
                left.children[..left.size + 1].clone_from_slice(&self.children[..m]);
                right.children[..right.size + 1].clone_from_slice(&self.children[m..]);
            }
            Some((self.keys[m - 1].clone(), left, right))
        } else {
            None
        }
    }

    fn get(&self, key: u64) -> Option<Ptr<T>> {
        if self.children[0].is_some() {
            match self.keys[..self.size].binary_search(&key) {
                Ok(pos) => {
                    Some(self.values[pos].as_ref().unwrap().clone())
                }
                Err(pos) => {
                    self.children[pos].as_ref().unwrap().deref().borrow().get(key)
                }
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
}

#[derive(Clone, Debug)]
struct BPTree<T>
    where
        T: Ord + Clone,
{
    root: Node<T>,
}

impl<T> BPTree<T>
    where
        T: Ord + Clone,
{
    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn insert(&mut self, key: u64, value: T) {
        let new_root = self.root.insert(key, value);
        if let Some((key, left, right)) = new_root {
            let mut root = Node::new();
            root.keys[0] = key;
            root.children[0] = Some(ptr(left));
            root.children[1] = Some(ptr(right));
            root.size = 1;
            self.root = root;
        }
    }

    fn get(&self, key: u64) -> Option<Ptr<T>> {
        self.root.get(key)
    }

    fn to_vec(&self) -> Vec<u64> {
        self.root.to_vec()
    }
}

fn main() {
    println!("size of node: {:?} B", size_of::<Node<u32>>());

    let n = 1000000;

    let mut rand = thread_rng();
    let mut to_add = vec![];
    let timer = Instant::now();
    for _ in 0..n {
        let ne = rand.next_u64();
        to_add.push(ne);
    }
    to_add.sort();
    println!("creating {} random values: {:?}", n, timer.elapsed());

    let mut t: BPTree<u64> = BPTree::new();
    let timer = Instant::now();
    for it in to_add.iter() {
        t.insert(*it, 0);
    }
    println!("finished inserting: {:?}", timer.elapsed());

    println!("is tree correct: {}", to_add == t.to_vec());

    println!("{:?}", t.get(42));
    t.insert(42, 42);
    println!("{:?}", t.get(42));
    *t.get(42).unwrap().deref().borrow_mut() *= 2;
    println!("{:?}", t.get(42));
}
