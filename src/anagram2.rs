use md5;
use std::fs;
use std::thread;
use std::sync::Arc;

const NTHREADS: usize = 5;

struct Word {
    string: String,
    stripped: String,
    len: usize
}
    
impl Word {
    // pub fn new<Stig: Into<String>>(word_in: Stig) -> Word {
    //     let mut word_sorted = word_in.into().as_bytes();
    pub fn new(word_in: & str) -> Word {
        // println!("{}", word_in);
        Word {
            string: word_in.to_string(),
            stripped: strip_blanks(word_in),
            len: strip_blanks(word_in).len()
            }
    }

    fn contains(&self, string_in: &str) -> bool {
        string_contains(&self.stripped, string_in)
    }
}

fn combine<'a>(words_in: &'a str, stripped_in: &'a str, md5_hash: &str, wordlist: &Vec<Word>, anagram: &Word, level: u8) {
    wordlist
        .iter()
        .map( |word| (format!("{} {}", words_in, word.string), format!("{}{}", stripped_in, word.string)) )
        .filter( |words| anagram.contains(&words.1) )
        .filter( |words| {
            if words.1.len() < anagram.stripped.len() {
                combine(&words.0, &words.1, md5_hash, wordlist, anagram, level + 1);
                false
            }
            else {
                // println!("{}:{}", level, words.0);
                words.1.len() == anagram.len
            }
        })
        .filter( |words| {
            if format!("{:x}", md5::compute(&words.0)) == md5_hash {
                println!("Found phrase: {}", words.0);
                panic!("done");
            }
            else {
                false
            }
        })
        .for_each(drop);
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

pub(crate) fn anagram(wordlist_file: &str, anagram: &str, _md5_hash1: &str) {
    println!("Version 2 ...");
    let anagram_word = Arc::new(Word::new(anagram));
    // let md5_hash: &str = "e4820b45d2277f3844eac66c903e84be";

    let mut wordlist = Vec::new();

    let wordlist_in = fs::read_to_string(wordlist_file).expect("Something went wrong reading the wordlist file");

    for word_in in wordlist_in.lines() {

        if anagram_word.contains(word_in) {
            wordlist.push(Word::new(word_in));
        }
    }

    // wordlist.sort_by( |a,b| b.string.len().cmp(&a.string.len()) );

    let mut threads = vec![];
    // let wordlist_arc = Arc::new(&wordlist);

    for i in 0..NTHREADS {
        // let wordlist1 = wordlist.clone();
        let anagram_word1 = anagram_word.clone();
        threads.push(thread::spawn(move || {
            println!("{}", anagram_word1.stripped);
            // combine(&wordlist[i].string, &wordlist[i].stripped, md5_hash, &wordlist, &wordlist[0], 2);
        }))
    }
}