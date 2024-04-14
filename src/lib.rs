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
    let code_chars = collection();
    plain.chars().filter_map(|x| code_chars.get(&x).cloned()).collect()
}
/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {

}
