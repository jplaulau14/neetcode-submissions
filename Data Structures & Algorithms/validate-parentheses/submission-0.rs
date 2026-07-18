impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut brackets = s.into_bytes();
        let mut top = 0usize;

        for index in 0..brackets.len() {
            let bracket = brackets[index];

            match bracket {
                b'(' => {
                    brackets[top] = b')';
                    top += 1;
                }
                b'[' => {
                    brackets[top] = b']';
                    top += 1;
                }
                b'{' => {
                    brackets[top] = b'}';
                    top += 1;
                }
                _ => {
                    if top == 0 {
                        return false;
                    }

                    top -= 1;

                    if brackets[top] != bracket {
                        return false;
                    }
                }
            }
        }

        top == 0
    }
}