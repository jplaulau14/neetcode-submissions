use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, usize>,
    nodes: Vec<Node>,
    free: Vec<usize>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let nodes = vec![
            Node {
                key: 0,
                value: 0,
                prev: 0,
                next: 1,
            },
            Node {
                key: 0,
                value: 0,
                prev: 0,
                next: 1,
            },
        ];

        Self {
            capacity: capacity as usize,
            cache: HashMap::new(),
            nodes,
            free: Vec::new(),
        }
    }

    fn remove(&mut self, index: usize) {
        let previous = self.nodes[index].prev;
        let next = self.nodes[index].next;
        self.nodes[previous].next = next;
        self.nodes[next].prev = previous;
    }

    fn insert_mru(&mut self, index: usize) {
        let previous = self.nodes[1].prev;
        self.nodes[index].prev = previous;
        self.nodes[index].next = 1;
        self.nodes[previous].next = index;
        self.nodes[1].prev = index;
    }

    fn move_mru(&mut self, index: usize) {
        self.remove(index);
        self.insert_mru(index);
    }

    fn get(&mut self, key: i32) -> i32 {
        let Some(index) = self.cache.get(&key).copied() else {
            return -1;
        };

        let value = self.nodes[index].value;
        self.move_mru(index);
        value
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(index) = self.cache.get(&key).copied() {
            self.nodes[index].value = value;
            self.move_mru(index);
            return;
        }

        let index = if let Some(index) = self.free.pop() {
            self.nodes[index].key = key;
            self.nodes[index].value = value;
            index
        } else {
            self.nodes.push(Node {
                key,
                value,
                prev: 0,
                next: 1,
            });
            self.nodes.len() - 1
        };

        self.cache.insert(key, index);
        self.insert_mru(index);

        if self.cache.len() > self.capacity {
            let least_recently_used = self.nodes[0].next;
            self.remove(least_recently_used);
            let old_key = self.nodes[least_recently_used].key;
            self.cache.remove(&old_key);
            self.free.push(least_recently_used);
        }
    }
}