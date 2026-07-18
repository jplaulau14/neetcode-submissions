struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        let minimum = self.stack.last().map_or(val, |&(_, minimum)| val.min(minimum));
        self.stack.push((val, minimum));
    }

    fn pop(&mut self) {
        let _ = self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}