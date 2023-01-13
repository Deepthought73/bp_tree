use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub type Ptr<T> = Rc<RefCell<T>>;

pub fn ptr<T>(a: T) -> Ptr<T> {
    Rc::new(RefCell::new(a))
}

pub fn array_insert<T: Debug>(array: &mut [T], index: usize, value: T, size: usize) {
    if index >= size {
        array[size] = value;
    } else {
        array[index..].rotate_right(1);
        array[index] = value;
    }
}
