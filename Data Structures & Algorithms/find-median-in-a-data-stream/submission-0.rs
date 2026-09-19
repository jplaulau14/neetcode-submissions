use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            lo: BinaryHeap::new(),
            hi: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.lo.push(num);
        let top = self.lo.pop().unwrap();
        self.hi.push(Reverse(top));
        if self.hi.len() > self.lo.len() {
            let Reverse(top) = self.hi.pop().unwrap();
            self.lo.push(top);
        }
    }

    fn find_median(&self) -> f64 {
        let low = *self.lo.peek().unwrap() as f64;
        if self.lo.len() > self.hi.len() {
            low
        } else {
            let Reverse(high) = *self.hi.peek().unwrap();
            (low + high as f64) / 2.0
        }
    }
}