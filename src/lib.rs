use std::{collections::VecDeque, fmt::Debug};

use rand::Rng;

#[repr(transparent)]
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct SortedArray<T: PartialOrd + Clone> {
    data: VecDeque<T>
}

impl <T: PartialOrd + Clone> SortedArray<T> {

    pub fn new() -> Self {
        return Self {
            data: VecDeque::new(),
        };
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        return Self {
            data: VecDeque::with_capacity(capacity),
        }
    }

    pub fn new_random(len: i32, min: i32, max: i32) -> SortedArray<i32> {

        let mut arr: Vec<i32> = Vec::with_capacity(len.try_into().unwrap());
        arr.fill_with(|| rand::thread_rng().gen_range(min..max));
        return arr.into();

    }

    /// Adds a value.
    /// Ensures the value gets added in a spot which ensures the array is always sorted.
    pub fn add(&mut self, value: T) -> () {

        let length = self.data.len();

        if length == 0 {
            self.data.push_front(value);
            return;
        }

        let mut low: i32 = 0;
        let mut mid: i32 = 0; // doesn't have to be initialized but rust doesn't know that
        let mut hig: i32 = (length - 1).try_into().unwrap(); // idgaf if this is too big

        while low <= hig {

            mid = (low + hig) / 2;

            match self.get_i32(mid) {
                itm if value < *itm => hig = mid - 1,
                itm if value > *itm => low = mid + 1,
                itm if value == *itm => break,
                _ => panic!(),
            }

        }

        if value > *self.get_i32(mid) { mid += 1; }

        self.data.insert(mid.try_into().unwrap(), value);

        return;

    }

    /// Method for removing the `i`th value from the array
    pub fn remove(&mut self, i: usize) -> Option<T> {
        return self.data.remove(i);
    }

    /// Method for retreiving value using an i32
    /// Panics if i can't be converted to usize.
    pub fn get(&self, i: usize) -> &T {
        return &self.data[i];
    }

    pub fn get_i32(&self, i: i32) -> &T {
        return &self.data[i.try_into().expect("Can't convert i32 to usize!")];
    }

    pub fn get_inner(&self) -> &VecDeque<T> {
        return &self.data;
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn contains(&self, value: &T) -> Option<usize> {

        let length = self.data.len();

        if length == 0 {
            return None;
        }

        let mut low: i32 = 0;
        let mut mid: i32;
        let mut hig: i32 = (length - 1).try_into().unwrap(); // idgaf if this is too big

        while low <= hig {

            mid = (low + hig) / 2;

            match self.get_i32(mid) {
                itm if *value <  *itm => hig = mid - 1,
                itm if *value >  *itm => low = mid + 1,
                itm if *value == *itm => return Some(mid.try_into().unwrap()),
                _ => panic!(),
            }

        }

        return None;

    }

}

impl <T: PartialOrd + Clone> From<Vec<T>> for SortedArray<T> {

    fn from(value: Vec<T>) -> Self {

        let mut out_arr = SortedArray::new();

        for itm in value {
            out_arr.add(itm);
        }

        return out_arr;

    }

}
