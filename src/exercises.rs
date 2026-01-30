use rand::seq::SliceRandom;
use rand::Rng;

const COMMON_WORDS: &[&str] = &[
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "it", "for", "not", "on", "with",
    "he", "as", "you", "do", "at", "this", "but", "his", "by", "from", "they", "we", "say", "her",
    "she", "or", "an", "will", "my", "one", "all", "would", "there", "their", "what", "so", "up",
    "out", "if", "about", "who", "get", "which", "go", "me", "when", "make", "can", "like", "time",
    "no", "just", "him", "know", "take", "people", "into", "year", "your", "good", "some", "could",
    "them", "see", "other", "than", "then", "now", "look", "only", "come", "its", "over", "think",
    "also", "back", "after", "use", "two", "how", "our", "work", "first", "well", "way", "even",
    "new", "want", "because", "any", "these", "give", "day", "most", "us",
];

const CODE_PATTERNS: &[&str] = &[
    "const", "let", "var", "function", "return", "if", "else", "for", "while", "break", "continue",
    "class", "import", "export", "async", "await", "try", "catch", "throw", "new", "this", "super",
    "extends", "implements", "interface", "type", "enum", "public", "private", "protected", "static",
    "void", "null", "undefined", "true", "false", "=>", "===", "!==", "&&", "||", "++", "--",
    "{}", "[]", "()", "<>", "/*", "*/", "//", "/**", "*/", "${}", "`", "->", "::", "fn", "mut",
    "struct", "impl", "trait", "pub", "use", "mod", "crate", "self", "super", "enum", "match",
    "Some", "None", "Ok", "Err", "Vec", "String", "Result", "Option",
];

pub enum ExerciseMode {
    RandomWords,
    Code,
    Targeted(Vec<String>),
    #[allow(dead_code)]
    Custom(String),
}

pub fn generate_exercise(mode: &ExerciseMode, word_count: usize) -> String {
    let mut rng = rand::thread_rng();

    match mode {
        ExerciseMode::RandomWords => {
            let words: Vec<&str> = (0..word_count)
                .map(|_| *COMMON_WORDS.choose(&mut rng).unwrap())
                .collect();
            words.join(" ")
        }
        ExerciseMode::Code => {
            let patterns: Vec<&str> = (0..word_count)
                .map(|_| *CODE_PATTERNS.choose(&mut rng).unwrap())
                .collect();
            patterns.join(" ")
        }
        ExerciseMode::Targeted(bigrams) => {
            if bigrams.is_empty() {
                return generate_exercise(&ExerciseMode::RandomWords, word_count);
            }

            // Generate text heavily featuring problem bigrams
            let mut text = String::new();
            for _ in 0..word_count {
                let bigram = bigrams.choose(&mut rng).unwrap();

                // Find or create words containing this bigram
                let words_with_bigram: Vec<&str> = COMMON_WORDS
                    .iter()
                    .filter(|w| w.contains(bigram.as_str()))
                    .copied()
                    .collect();

                if !words_with_bigram.is_empty() {
                    text.push_str(words_with_bigram.choose(&mut rng).unwrap());
                } else {
                    // Generate nonsense word with the bigram
                    let prefix: String = (0..rng.gen_range(1..3))
                        .map(|_| rng.gen_range(b'a'..=b'z') as char)
                        .collect();
                    let suffix: String = (0..rng.gen_range(1..3))
                        .map(|_| rng.gen_range(b'a'..=b'z') as char)
                        .collect();
                    text.push_str(&format!("{}{}{}", prefix, bigram, suffix));
                }
                text.push(' ');
            }
            text.trim().to_string()
        }
        ExerciseMode::Custom(text) => text.clone(),
    }
}

#[allow(dead_code)]
pub fn generate_bigram_drill(bigram: &str, reps: usize) -> String {
    let mut text = String::new();
    for i in 0..reps {
        text.push_str(bigram);
        if i < reps - 1 {
            text.push(' ');
        }
    }
    text
}

#[allow(dead_code)]
pub fn generate_key_drill(ch: char, reps: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut text = String::new();

    for i in 0..reps {
        // Alternate the problematic key with random keys
        text.push(ch);
        if i < reps - 1 {
            text.push(rng.gen_range(b'a'..=b'z') as char);
            text.push(' ');
        }
    }
    text
}
