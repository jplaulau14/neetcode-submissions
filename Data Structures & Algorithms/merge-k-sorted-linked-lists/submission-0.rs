use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct HeapEntry {
    value: i32,
    sequence: usize,
    node: Box<ListNode>,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.sequence == other.sequence
    }
}

impl Eq for HeapEntry {}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .value
            .cmp(&self.value)
            .then_with(|| other.sequence.cmp(&self.sequence))
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        let mut sequence = 0;

        for node in lists.into_iter().flatten() {
            heap.push(HeapEntry {
                value: node.val,
                sequence,
                node,
            });
            sequence += 1;
        }

        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut tail = &mut dummy;

        while let Some(entry) = heap.pop() {
            let mut node = entry.node;
            let next = node.next.take();

            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();

            if let Some(next_node) = next {
                heap.push(HeapEntry {
                    value: next_node.val,
                    sequence,
                    node: next_node,
                });
                sequence += 1;
            }
        }

        dummy.next
    }
}