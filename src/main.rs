mod array_impl;
mod other_impl;
mod util;
mod vec_impl;

use crate::array_impl::ArrayBPTree;
use crate::vec_impl::VecBPTree;
use rand::{thread_rng, RngCore};
use std::collections::BTreeSet;
use std::time::Instant;

fn main() {
    let n = 1000000;

    let mut rand = thread_rng();
    let mut to_add = vec![];
    let timer = Instant::now();
    for _ in 0..n {
        let ne = rand.next_u64();
        to_add.push(ne);
    }
    println!("creating {} random values: {:?}", n, timer.elapsed());

    let mut t1 = VecBPTree::new(64);
    let timer = Instant::now();
    for it in to_add.iter() {
        t1.insert(*it, *it);
    }
    println!("finished inserting (vec): {:?}", timer.elapsed());

    //let mut t2 = ArrayBPTree::new();
    //let timer = Instant::now();
    //for it in to_add.iter() {
    //    t2.insert(*it, *it);
    //}
    //println!("finished inserting (array): {:?}", timer.elapsed());

    // to_add.sort();
    // println!("is tree correct (vec): {}", to_add == t1.to_vec());
    // println!("is tree correct (array): {}", to_add == t2.to_vec());
}

// ~300ms
