use std::fs;

const ANAGRAM: &str = "poultry outwits ants";
const ANAGRAM_WORD: &str = "poultryoutwitsants";
const WORDLIST_FILE: &str = "C:\\Users\\120637\\cargo\\anagram\\wordlist";

struct Word {
    string: String,
    sorted: String
}

    // let anagram_bytes: &[u8] = ANAGRAM.replace(" ", "").as_bytes();
    
impl Word {
    pub fn new<Stig: Into<String>>(word_in: Stig) -> Word {
        Word {
            string: word_in.into(),
            sorted: word_in.into().chars().sort()
            }
    }

    fn contains(&self, word_in: &str) -> bool {
        let mut word_in_chars: Vec<char> = word_in.chars().collect();
        word_in_chars.sort();
        for 
        self.list.contains(word)
    }
}

fn vec_compare(va: &[char], vb: &[char]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| a == b)
}

fn main() {
    let mut chars: [char] = ANAGRAM_WORD.chars(); //.collect();
    chars.sort();
    let anagram_word = Word::new(ANAGRAM_WORD);

    let mut wordlist: Vec<Word> = Vec::new();

    let wordlist_in = fs::read_to_string(WORDLIST_FILE)
        .expect("Something went wrong reading the wordlist file");
    for word_in in wordlist_in.lines() {

        if anagram_word.contains(word_in) {
            wordlist.push(Word::new(word_in));
        }
    }
}