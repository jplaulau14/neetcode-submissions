pub struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    is_word: bool,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self {
            children: std::array::from_fn(|_| None),
            is_word: false,
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut node = self;
        for byte in word.bytes() {
            let index = (byte - b'a') as usize;
            node = node.children[index]
                .get_or_insert_with(|| Box::new(Self::new()))
                .as_mut();
        }
        node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        self.matches(word.as_bytes(), 0)
    }

    fn matches(&self, word: &[u8], index: usize) -> bool {
        if index == word.len() {
            return self.is_word;
        }
        if word[index] == b'.' {
            return self.children.iter().any(|child| {
                child
                    .as_deref()
                    .map_or(false, |node| node.matches(word, index + 1))
            });
        }
        let child_index = (word[index] - b'a') as usize;
        self.children[child_index]
            .as_deref()
            .map_or(false, |node| node.matches(word, index + 1))
    }
}