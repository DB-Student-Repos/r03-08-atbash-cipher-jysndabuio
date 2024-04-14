use std::collections::HashMap;

/// "Encipher" with the Atbash cipher.
pub fn collection()-> HashMap<char, char> {
    let plain_chars = "abcdefghijklmnopqrstuvwxyz";
    let cipher_chars = "zyxwvutsrqponmlkjihgfedcba";
    let mut code_chars: HashMap<char, char> = HashMap::new();
    for (plain, cipher) in plain_chars.chars().zip(cipher_chars.chars()) {
        code_chars.insert(plain, cipher);
    }
    code_chars
}

pub fn encode(plain: &str) -> String {
    let word: String = plain.chars().filter(|&x| x.is_ascii_alphabetic()).collect()
    let code_chars = collection();
    let mut cipher = String::new(); 
    for x in word.chars(){
        match x {
            x.is_digit(10) => cipher.push(x),
            _ => cipher.push(*code_chars.get(&x).unwrap_or(&x)),
        }
    }
}
/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {

}
