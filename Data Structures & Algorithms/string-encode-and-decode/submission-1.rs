impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded = String::new();

        for value in strs {
            encoded.push_str(&value.len().to_string());
            encoded.push('#');
            encoded.push_str(&value);
        }

        encoded
    }

    pub fn decode(encoded: String) -> Vec<String> {
        let bytes = encoded.as_bytes();
        let mut result = Vec::new();
        let mut position = 0;

        while position < bytes.len() {
            let mut length = 0usize;

            while bytes[position] != b'#' {
                length = length * 10 + (bytes[position] - b'0') as usize;
                position += 1;
            }

            position += 1;

            let end = position + length;
            result.push(encoded[position..end].to_string());
            position = end;
        }

        result
    }
}