const SHIFT: usize = 1;

pub fn encrypt(input: &[u8], alphabet: &[u8]) -> Vec<u8> {
    let mut output = vec![0_u8; input.len()];

    input
        .iter()
        .enumerate()
        .for_each(|(i, _)| output[i] = alphabet[(i + SHIFT) % alphabet.len()]);

    return output;
}
