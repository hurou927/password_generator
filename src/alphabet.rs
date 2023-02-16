
use rand::{rngs::ThreadRng, seq::SliceRandom};

pub struct AlphabetCondition {
    chars: Vec<u8>,
    min_length: usize,
}

impl AlphabetCondition {
    pub fn new(chras: Vec<u8>, min_length: usize) -> Self {
        AlphabetCondition {
            chars: chras,
            min_length,
        }
    }
    pub fn choose_chars(&self, rng: &mut ThreadRng) -> Vec<u8> {
        let mut cs = Vec::new();
        for _ in 0..self.min_length {
            if let Some(&c) = self.chars.choose(rng) {
                cs.push(c);
            }
        }
        cs
    }
}

pub struct Alphabets {
    conditions: Vec<AlphabetCondition>,
    all_alphabet: AlphabetCondition,
}

impl Alphabets {
    pub fn new(
        length: usize,
        lower_min_length: usize,
        upper_min_length: usize,
        number_min_length: usize,
        simbol_min_length: usize,
        simbol_chars: Vec<u8>,
    ) -> Alphabets {
        let alphabet_conditions: Vec<AlphabetCondition> = vec![
            AlphabetCondition::new((b'a'..=b'z').collect(), lower_min_length),
            AlphabetCondition::new((b'A'..=b'Z').collect(), upper_min_length),
            AlphabetCondition::new((b'0'..=b'9').collect(), number_min_length),
            AlphabetCondition::new(simbol_chars, simbol_min_length),
        ];
        // 残りの文字数文の文字を対象となるすべての文字群から選択する
        let rest: Vec<u8> = alphabet_conditions
            .iter()
            .filter(|x| x.min_length > 0)
            .flat_map(|x| x.chars.clone())
            .collect();
        let rest = AlphabetCondition {
            chars: rest,
            min_length: length
                - alphabet_conditions
                    .iter()
                    .map(|x| x.min_length)
                    .sum::<usize>(),
        };
        Alphabets {
            conditions: alphabet_conditions,
            all_alphabet: rest,
        }

    }
    pub fn gen_password(&self, rng: &mut ThreadRng) -> String {
        // 各文字グループから最低長文の文字をランダムに選択する
        let mut password = self
            .conditions
            .iter()
            .flat_map(|x| x.choose_chars(rng))
            .collect::<Vec<u8>>();

        password.extend(self.all_alphabet.choose_chars(rng));

        // Shuffle
        password.shuffle(rng);

        // Complete
        String::from_utf8(password).unwrap()
    }
}
