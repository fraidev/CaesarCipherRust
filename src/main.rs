mod cipher;

fn main() {
    println!("Encrypt Result: {}", cipher::encrypt(String::from("ABC")));
    println!("Decrypt Result: {}", cipher::decrypt(String::from("ABC")));
}
