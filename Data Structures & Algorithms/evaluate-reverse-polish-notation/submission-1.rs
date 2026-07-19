impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(tokens.len());

        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    let result = match token.as_str() {
                        "+" => left + right,
                        "-" => left - right,
                        "*" => left * right,
                        _ => left / right,
                    };

                    stack.push(result);
                }
                _ => stack.push(token.parse().unwrap()),
            }
        }

        stack.pop().unwrap()
    }
}