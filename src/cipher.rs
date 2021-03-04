enum Operation {
    Sum,
    Subtract,
}

fn shift(input: String, shift_count: i32, operation: Operation) -> String {
    let alphabet = ('A'..='Z').map(char::from).collect::<Vec<_>>();

    let operator = |n: i32| match operation {
        Operation::Sum => n + shift_count,
        Operation::Subtract => n - shift_count,
    };

    let modulus = |a: i32, b: i32| (((a) % b + b) % b) as u8;

    let upper_input = input.to_uppercase();

    upper_input.chars()
        .map(|letter| {
            (modulus(operator(alphabet.iter().position(|x| *x == letter).unwrap() as i32), 26) + 65) as char
        }).collect()
}

pub fn encrypt(input: String) -> String {
    shift(input, 3, Operation::Subtract)
}

pub fn decrypt(input: String) -> String {
    shift(input, 3, Operation::Sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cipher_must_encrypt() {
        assert_eq!("ABC", encrypt(String::from("DEF")));
        assert_eq!("XYZ", encrypt(String::from("ABC")));
    }

    #[test]
    fn cipher_must_decrypt() {
        assert_eq!("DEF", decrypt(String::from("ABC")));
        assert_eq!("ABC", decrypt(String::from("XYZ")));
    }
}
