pub struct PrefixTree {
    children: [Option<Box<PrefixTree>>; 26],
    is_word: bool,
}

impl PrefixTree {
    pub fn new() -> Self {
        Self {
            children: std::array::from_fn(|_| None),
            is_word: false,
        }
    }

    pub fn insert(&mut self, word: String) {
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
        self.find(word.as_bytes())
            .map_or(false, |node| node.is_word)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix.as_bytes()).is_some()
    }

    fn find(&self, text: &[u8]) -> Option<&Self> {
        let mut node = self;

        for &byte in text {
            let index = (byte - b'a') as usize;
            node = node.children[index].as_deref()?;
        }

        Some(node)
    }
}