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

        // abc...xyz: lower_min_length
        // ABC...XYZ: upper_min_length
        // 012...789: number_min_length
        // ^&*...#$%: simbol_chars
        // ab..yzAB..YZ01..89^&..$%: length - SUM(*_min_length]
        let mut alphabet_conditions: Vec<AlphabetCondition> = vec![
            AlphabetCondition::new((b'a'..=b'z').collect(), lower_min_length),
            AlphabetCondition::new((b'A'..=b'Z').collect(), upper_min_length),
            AlphabetCondition::new((b'0'..=b'9').collect(), number_min_length),
            AlphabetCondition::new(simbol_chars, simbol_min_length),
        ];
        let all_chars = AlphabetCondition {
            chars: alphabet_conditions
                .iter()
                .filter(|x| x.min_length > 0)
                .flat_map(|x| x.chars.clone())
                .collect(),
            min_length: length
                - alphabet_conditions
                    .iter()
                    .map(|x| x.min_length)
                    .sum::<usize>(),
        };

        alphabet_conditions.push(all_chars);

        Alphabets {
            conditions: alphabet_conditions,
        }
    }
    pub fn gen_password(&self, rng: &mut ThreadRng) -> String {
        let mut password = self
            .conditions
            .iter()
            .flat_map(|x| x.choose_chars(rng))
            .collect::<Vec<u8>>();

        // Shuffle
        password.shuffle(rng);

        // Complete
        String::from_utf8(password).unwrap()
    }
}
