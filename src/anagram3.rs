use std::fs;
use std::thread;
use md5;

const NTHREADS: usize = 5;


struct Word {
    string: String,
    stripped: String
}
    
impl Word {
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


struct Solver {
    wordlist: Vec<Word>,
    anagram: Word,
    md5_hash: String,
    range_seq: usize
}

impl Solver {
    pub fn new(wordlist_file: &str, anagram_in: &str, md5_hash: &str, range_seq: usize) -> Solver {
        let anagram = Word::new(anagram_in);
        Solver {
            wordlist: Solver::make_wordlist(wordlist_file, &anagram),
            anagram: anagram,
            md5_hash: md5_hash.to_string(),
            range_seq: range_seq
        }
    }

    fn make_wordlist(wordlist_file: &str, anagram: &Word) -> Vec<Word> {
        let mut wordlist: Vec<Word> = Vec::new();
        let wordlist_in = fs::read_to_string(wordlist_file).expect("Something went wrong reading the wordlist file");
        for word_in in wordlist_in.lines() {

            if anagram.contains(word_in) {
                wordlist.push(Word::new(word_in));
            }
        }
        println!("Wordlist count: {}", wordlist.len());
        wordlist
    }

    fn solve_it(&self) {
        for word in &self.wordlist {
            self.combine(&(word.string.to_string(), word.stripped.to_string()), 2);
        }
    }

    fn combine(&self, words_in: &(String, String), level: u8) {
        self.wordlist
            .iter()
            .map( |word| (format!("{} {}", words_in.0, word.string), format!("{}{}", words_in.1, word.string)) )
            .filter( |words| self.anagram.contains(&words.1) )
            .filter( |words| {
                if words.1.len() < self.anagram.stripped.len() {
                    self.combine(words, level + 1);
                    false
                }
                else { true }
            })
            .for_each( |words| {
                println!("{}:{}:{}", self.range_seq, level, words.0);
                if format!("{:x}", md5::compute(&words.0)) == self.md5_hash {
                    println!("Found phrase: {}", words.0);
                    panic!("done");
                };
            });
    }
}


pub(crate) fn anagram(wordlist_file: &'static str, anagram: &'static str, md5_hash: &'static str) {
    println!("Version 3 ...");

    let mut threads = vec![];
    
    for i in 0..NTHREADS {
        threads.push(thread::spawn(move || {
            Solver::new(wordlist_file, anagram, md5_hash, i).solve_it();
        }))
    }

    for thread in threads {
        let _ = thread.join();
    }
}