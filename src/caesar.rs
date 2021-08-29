const SHIFT: usize = 1;

pub fn encrypt(plain_text: &[u8], alphabet: &[u8]) -> Vec<u8> {
    let mut cipher_text = vec![0_u8; plain_text.len()];

    plain_text
        .iter()
        .enumerate()
        .for_each(|(i, _)| cipher_text[i] = alphabet[(i + SHIFT) % alphabet.len()]);

    cipher_text
}

pub fn decrypt(cipher_text: &[u8], alphabet: &[u8]) -> Vec<u8> {
    let mut plain_text = vec![0_u8; cipher_text.len()];

    cipher_text
        .iter()
        .enumerate()
        .for_each(|(i, _)| plain_text[i] = alphabet[(alphabet.len() + i - SHIFT) % alphabet.len()]);

    plain_text
}
