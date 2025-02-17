/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        if !self.is_empty() {
            let last_idx = self.tail_idx();
            self.ajust_up(last_idx);
        }
    }

    fn tail_idx(&self) -> usize {
        self.count
    }

    fn ajust_up(&mut self, idx: usize) {
        let mut parent_idx = self.parent_idx(idx);
        let mut idx = idx;
        while idx > 1
            && (self.comparator)(
                self.items.get(idx).unwrap(),
                self.items.get(parent_idx).unwrap(),
            )
        {
            self.items.swap(idx, parent_idx);
            idx = parent_idx;
            parent_idx = self.parent_idx(idx);
        }
    }

    fn ajust_down(&mut self, idx: usize) {
        let mut idx = idx;
        let mut child_idx;
        let mut left_idx = self.left_child_idx(idx);
        let mut right_idx = self.right_child_idx(idx);
        while left_idx < self.len() {
            if right_idx <= self.len()
                && (self.comparator)(
                    self.items.get(left_idx).unwrap(),
                    self.items.get(right_idx).unwrap(),
                )
            {
                child_idx = left_idx;
            } else {
                child_idx = right_idx;
            }

            if (self.comparator)(
                self.items.get(idx).unwrap(),
                self.items.get(child_idx).unwrap(),
            ) {
                break;
            }

            self.items.swap(idx, child_idx);
            idx = child_idx;
            left_idx = self.left_child_idx(idx);
            right_idx = self.right_child_idx(idx);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        } else {
            let tail = self.tail_idx();
            self.items.swap(1, tail);
            let v = self.items.pop();
            self.count -= 1;
            self.ajust_down(1);
            // self.print_heap();
            v
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
