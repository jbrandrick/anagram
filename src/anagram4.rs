use std::cmp::Ordering;
use std::fs;
use std::ops::Add;
use std::thread;
use md5;

const NTHREADS: usize = 10;


pub(crate) fn anagram(wordlist_file: &'static str, anagram: &'static str, md5_hash: &'static str) {
    println!("Version 4 ...");

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

fn string_contains(haystack_in: &str, needle: &str) -> bool {
    let mut haystack: String = haystack_in.to_string();
     needle
        .chars()
        .skip_while( |&c| c == ' ')
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


struct Word {
    string: String,
    freq: [u8;26],
    len: usize
}
    
impl Word {
    pub fn new(word_in: &str) -> Word {
        let mut freq_in = [0;26];
        word_in
            .chars()
            .skip_while( |&c| c == ' ')
            .for_each( |c| {
                if c >= 'a' {
                    let diff = (c as usize) - ('a' as usize);
                    if diff < 26 {
                        freq_in[(c as usize) - ('a' as usize)] += 1;
                    }
                }
            });
        let result = Word {
                        string: word_in.to_owned(),
                        freq: freq_in.to_owned(),
                        len: word_in.len()
                        };
        result
    }

    fn plus(&self, other: &Word) -> Word {
        Word::new(&format!("{} {}", self.string, other.string))
    }

    fn contains(&self, string_in: &str) -> bool {
        string_contains(&self.string, string_in)
    }
}

impl Add for Word {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Word::new(&format!("{} {}", self.string, other.string))
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len &&
            self.freq.iter().zip(other.freq.iter()).all( |(x, y)| x == y )
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.len > other.len && self.freq.iter().zip(other.freq.iter()).all( |(x, y)| x >= y ) {
            return Some(Ordering::Greater);
            }
        else if self.len == other.len && self.freq.iter().zip(other.freq.iter()).all( |(x, y)| x == y ) {
            return Some(Ordering::Equal);
            }
        else { return Some(Ordering::Less) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_eq() {
        let w1 = Word::new("abc");
        let w2 = Word::new("abc");
        let w3 = Word::new("abd");
        let w4 = Word::new("abbc");

        assert_eq!(w1.eq(&w2), true);
        assert_eq!(w1.eq(&w3), false);
        assert_eq!(w1.eq(&w4), false);
    }

    #[test]
    fn test_partial_cmp(){
        let w1 = Word::new("poultry outwits ants");
        let w2 = Word::new("a");
        let w3 = Word::new("a a");
        let w4 = Word::new("a a a");

        assert_eq!(w1.partial_cmp(&w2), Some(Ordering::Greater));
        assert_eq!(w1.partial_cmp(&w3), Some(Ordering::Less));
        assert_eq!(w1.partial_cmp(&w4), Some(Ordering::Less));
    }
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
            if anagram.contains(&word_in) {
                wordlist.push(Word::new(word_in));
            }
        }
        println!("Wordlist count: {}", wordlist.len());
        wordlist
    }

    fn solve_it(&self) {
        let min = self.range_seq * self.wordlist.len() / NTHREADS;
        let max = min + self.wordlist.len() / NTHREADS;
        for word in &self.wordlist[min..max] {
            self.combine(word, 2);
        }
    }

    fn combine(&self, word_in: &Word, level: u8) {
        self.wordlist
            .iter()
            .map( |word| word_in.plus(word) )
            .filter( |word_combined| self.anagram.contains(&word_combined.string) )
            .for_each( |word_combined| {
                match self.anagram.partial_cmp(&word_combined).unwrap() {
                    Ordering::Equal => {
                        println!("{}:{}:{}", self.range_seq, level, word_combined.string);
                        if format!("{:x}", md5::compute(&word_combined.string)) == self.md5_hash {
                            println!("Found phrase: {}", word_combined.string);
                            panic!("done");
                        };
                    },
                    Ordering::Greater => self.combine(&word_combined, level + 1),
                    Ordering::Less => {}
                }
            })
    }
}