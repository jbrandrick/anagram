use std::fs;
use md5;

const WORDLIST_FILE: &str = "wordlist";
const MD5_HASH_EASY: &str = "e4820b45d2277f3844eac66c903e84be";
// const MD5_HASH_MEDIUM: &str = "23170acc097c24edb98fc5488ab033fe";
// const MD5_HASH_HARD: &str = "665e5bcb0c20062fe8abaaf4628bb154";

struct Word {
    string: String,
    stripped: String
}
    
impl Word {
    // pub fn new<Stig: Into<String>>(word_in: Stig) -> Word {
    //     let mut word_sorted = word_in.into().as_bytes();
    pub fn new(word_in: & str) -> Word {
        // println!("{}", word_in);
        Word {
            string: word_in.to_string(),
            stripped: strip_blanks(word_in)
            }
    }

    fn contains(&self, string_in: &str) -> bool {
        string_contains(&self.stripped, string_in)
    }
}

fn combine(words_in: &(String, String), wordlist: &Vec<Word>, anagram: &Word, level: u8) -> bool {
    wordlist
        .iter()
        .map( |word| (format!("{} {}", words_in.0, word.string), format!("{}{}", words_in.1, word.string)) )
        .filter( |words| {
            if words.1.len() < anagram.stripped.len() {
                combine(words, wordlist, anagram, level + 1);
                false
            }
            else {
                words.1.len() == anagram.stripped.len()
            }
        })
        .filter( |words| anagram.contains(&words.1) )
        .filter( |words| {
            if format!("{:x}", md5::compute(&words.0)) == MD5_HASH_EASY {
                println!("Found phrase: {}", words.0);
                panic!("done");
            }
            else {
                true
            }
        })
        .map( |words| words.0 )
        .collect::<String>()
        .len() > 0
}

fn string_contains(haystack_in: &str, needle: &str) -> bool {
    let mut haystack: String = haystack_in.to_string();
     needle
        .chars()
        .filter( |&c|
        {
            let found = haystack.find(c);
            if found.is_some() {
                haystack.remove(found.unwrap());
                false
            }
            else { true }
        })
        .collect::<String>()
        .len() == 0
}

fn strip_blanks(original : &str) -> String {
    original.chars().filter( |&c| c != ' ' ).collect()
}

fn main() {
    let anagram: &str = "poultry outwits ants";
    let anagram_word = Word::new(anagram);

    let mut wordlist: Vec<Word> = Vec::new();

    let wordlist_in = fs::read_to_string(WORDLIST_FILE).expect("Something went wrong reading the wordlist file");

    for word_in in wordlist_in.lines() {

        if anagram_word.contains(word_in) {
            wordlist.push(Word::new(word_in));
        }
    }
    println!("Wordlist count: {}", wordlist.len());
    println!("Anagram md5 hash: {:x}", md5::compute(anagram));

    wordlist.sort_by( |a,b| b.string.len().cmp(&a.string.len()) );

    wordlist
        .iter()
        .map( |word| {
            println!("{}", &word.string);
            &word.string
        })
        .filter( |word| combine(&(word.to_string(), strip_blanks(word)), &wordlist, &anagram_word, 2))
        .for_each( |word| println!("Found: {}", word));
}


/*
fn vec_compare(va: &[u8], vb: &[u8]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| a == b)
}

fn vec_contains(va: &[u8], vb: &[u8]) -> bool { // va contains vb
     va.iter()
       .zip(vb) // zip stops at the shortest
       .all(|(a,b)| a == b)
}
*/