impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::{HashSet, VecDeque};

        let mut words: HashSet<String> = word_list.into_iter().collect();
        if !words.contains(&end_word) {
            return 0;
        }
        words.remove(&begin_word);
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(begin_word);
        let mut length = 1;
        while !queue.is_empty() {
            let level = queue.len();
            for _ in 0..level {
                let word = queue.pop_front().unwrap();
                let mut letters = word.into_bytes();
                for i in 0..letters.len() {
                    let saved = letters[i];
                    for letter in b'a'..=b'z' {
                        if letter == saved {
                            continue;
                        }
                        letters[i] = letter;
                        let nxt = std::str::from_utf8(&letters).unwrap();
                        if words.remove(nxt) {
                            if nxt == end_word {
                                return length + 1;
                            }
                            queue.push_back(nxt.to_string());
                        }
                    }
                    letters[i] = saved;
                }
            }
            length += 1;
        }
        0
    }
}