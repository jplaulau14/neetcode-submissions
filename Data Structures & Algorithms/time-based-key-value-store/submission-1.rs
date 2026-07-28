use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store
            .entry(key)
            .or_default()
            .push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let Some(history) = self.store.get(&key) else {
            return String::new();
        };

        let mut left = 0usize;
        let mut right = history.len();

        while left < right {
            let middle = left + (right - left) / 2;

            if history[middle].0 <= timestamp {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        if left == 0 {
            String::new()
        } else {
            history[left - 1].1.clone()
        }
    }
}