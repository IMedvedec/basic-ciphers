const DEFAULT_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DEFAULT_KEY: &str = "ABC";

pub struct Config {
    alphabet: Vec<u8>,
    key: Vec<usize>,
}

impl Config {
    pub fn new() -> Config {
        let key_shifts: Vec<usize> = DEFAULT_KEY
            .to_uppercase()
            .as_bytes()
            .iter()
            .map(|x| *x as usize - 65_usize)
            .filter(|x| *x >= 0 && *x <= 25)
            .collect();

        Config {
            alphabet: DEFAULT_ALPHABET.as_bytes().to_vec(),
            key: key_shifts,
        }
    }

    fn get_symbol_position(&self, symbol: u8) -> usize {
        let mut symbol_alphabet_position = 0_usize;

        self.alphabet.iter().enumerate().for_each(|(i, x)| {
            if *x == symbol {
                symbol_alphabet_position = i
            }
        });

        symbol_alphabet_position
    }
}

pub fn encrypt(plain_text: &[u8], cfg: &Config) -> Vec<u8> {
    let mut cipher_text = vec![0_u8; plain_text.len()];
    let mut key_position = 0_usize;

    plain_text.iter().enumerate().for_each(|(i, symbol)| {
        cipher_text[i] = cfg.alphabet
            [(cfg.get_symbol_position(*symbol) + cfg.key[key_position]) % cfg.alphabet.len()];
        key_position = (key_position + 1) % cfg.key.len();
    });

    cipher_text
}

pub fn decrypt(cipher_text: &[u8], cfg: &Config) -> Vec<u8> {
    let mut plain_text = vec![0_u8; cipher_text.len()];
    let mut key_position = 0_usize;

    cipher_text.iter().enumerate().for_each(|(i, symbol)| {
        plain_text[i] = cfg.alphabet[(cfg.alphabet.len() + cfg.get_symbol_position(*symbol)
            - cfg.key[key_position])
            % cfg.alphabet.len()];

        key_position = (key_position + 1) % cfg.key.len();
    });

    plain_text
}
