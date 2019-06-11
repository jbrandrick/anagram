use std::fs;
use md5;

struct Word {
    string: String,
    stripped: String
}
    
impl Word {
    // pub fn new<Stig: Into<String>>(word_in: Stig) -> Word {
    //     let mut word_sorted = word_in.into().as_bytes();
    pub fn new(word_in: &str) -> Word {
        Word {
            string: word_in.to_string(),
            stripped: strip_blanks(word_in)
            }
    }

    fn contains(&self, string_in: &str) -> bool {
        string_contains(&self.stripped, string_in)
    }
}

fn strip_blanks(original : &str) -> String {
    original.chars().filter( |&c| c != ' ' ).collect()
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

fn combine(words_in: &(String, String), md5_hash: &str, wordlist: &Vec<Word>, anagram: &Word, level: u8) {
    wordlist
        .iter()
        .map( |word| (format!("{} {}", words_in.0, word.string), format!("{}{}", words_in.1, word.string)) )
        .filter( |words| anagram.contains(&words.1) )
        .filter( |words| {
            if words.1.len() < anagram.stripped.len() {
                combine(words, md5_hash, wordlist, anagram, level + 1);
                false
            }
            else { true }
                // println!("{}:{}", level, words.0);
                // words.1.len() == anagram.stripped.len()
            // }
        })
        .filter( |words| {
            println!("{}:{}", level, words.0);
            if format!("{:x}", md5::compute(&words.0)) == md5_hash {
                println!("Found phrase: {}", words.0);
                panic!("done");
            }
            else { false }
        })
        .for_each(drop);
}

pub(crate) fn anagram(wordlist_file: &str, anagram: &str, md5_hash: &str) {
    println!("Version 1 ...");
    let anagram_word = Word::new(anagram);

    let mut wordlist: Vec<Word> = Vec::new();
    let wordlist_in = fs::read_to_string(wordlist_file).expect("Something went wrong reading the wordlist file");

    for word_in in wordlist_in.lines() {

        if anagram_word.contains(word_in) {
            wordlist.push(Word::new(word_in));
        }
    }
    println!("Wordlist count: {}", wordlist.len());

    // wordlist.sort_by( |a,b| b.string.len().cmp(&a.string.len()) );

    for word in &wordlist {
        combine(&(word.string.to_string(), word.stripped.to_string()), md5_hash, &wordlist, &anagram_word, 2);
    }
}