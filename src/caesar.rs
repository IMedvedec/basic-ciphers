const DEFAULT_SHIFT: usize = 3;
const DEFAULT_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Config<'a> {
    shift: usize,
    alphabet: &'a [u8],
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        Config {
            shift: DEFAULT_SHIFT,
            alphabet: DEFAULT_ALPHABET.as_bytes(),
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

    plain_text.iter().enumerate().for_each(|(i, symbol)| {
        cipher_text[i] =
            cfg.alphabet[(cfg.get_symbol_position(*symbol) + cfg.shift) % cfg.alphabet.len()]
    });

    cipher_text
}

pub fn decrypt(cipher_text: &[u8], cfg: &Config) -> Vec<u8> {
    let mut plain_text = vec![0_u8; cipher_text.len()];

    cipher_text.iter().enumerate().for_each(|(i, symbol)| {
        plain_text[i] = cfg.alphabet[(cfg.alphabet.len() + cfg.get_symbol_position(*symbol)
            - cfg.shift)
            % cfg.alphabet.len()]
    });

    plain_text
}
