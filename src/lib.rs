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
    let word: String = plain.chars()
                            .filter(|&x| x.is_ascii_alphabetic() || x.is_ascii_digit())
                            .collect::<String>()
                            .to_lowercase();

    let converted_word: String = word.chars()
                            .map(|x| {match x {
                                'a'..='z' => *code_chars.get(&x).unwrap_or(&x),
                                _ => x,
                            }})
                            .collect();

    let sliced_str: String = converted_word.chars()
        .enumerate()
        .map(|(i, c)| if i % 5 == 0 && i != 0 { format!(" {}",c) } else { c.to_string() })
        .collect();

    sliced_str
}
/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let code_chars = collection();
    let word: String = cipher.replace(" ", "");
    let mut converted_word = String::new();
    for  x in word.chars() {
        let decoded_char = match code_chars.get(&x) {
            Some(plain) => *plain,
            None => x,
        };
        converted_word.push(decoded_char);
    }
    converted_word

}
