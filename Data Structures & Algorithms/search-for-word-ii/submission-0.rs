struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    word: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: std::array::from_fn(|_| None),
            word: None,
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for byte in word.bytes() {
            let index = (byte - b'a') as usize;
            node = node.children[index]
                .get_or_insert_with(|| Box::new(Self::new()))
                .as_mut();
        }
        node.word = Some(word);
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::new();
        for word in words {
            root.insert(word);
        }
        let rows = board.len();
        let cols = board[0].len();
        let mut result = Vec::new();
        for row in 0..rows {
            for col in 0..cols {
                Self::search_board(
                    &mut board,
                    row,
                    col,
                    &mut root,
                    &mut result,
                );
            }
        }
        result
    }

    fn search_board(
        board: &mut [Vec<char>],
        row: usize,
        col: usize,
        node: &mut TrieNode,
        result: &mut Vec<String>,
    ) {
        let letter = board[row][col];
        if letter == '#' {
            return;
        }
        let index = (letter as u8 - b'a') as usize;
        let Some(child) = node.children[index].as_mut() else {
            return;
        };
        if let Some(word) = child.word.take() {
            result.push(word);
        }
        board[row][col] = '#';
        if row > 0 {
            Self::search_board(board, row - 1, col, child, result);
        }
        if row + 1 < board.len() {
            Self::search_board(board, row + 1, col, child, result);
        }
        if col > 0 {
            Self::search_board(board, row, col - 1, child, result);
        }
        if col + 1 < board[row].len() {
            Self::search_board(board, row, col + 1, child, result);
        }
        board[row][col] = letter;
        let remove_child = child.word.is_none()
            && child.children.iter().all(|grandchild| grandchild.is_none());
        if remove_child {
            node.children[index] = None;
        }
    }
}