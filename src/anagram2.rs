use md5;
use std::fs;
use std::thread;
// use std::sync::Arc;

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

fn combine(words_in: &str, stripped_in: &str, md5_hash: &str, wordlist: &Vec<Word>, anagram: &Word, level: u8) -> bool {
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
        .map( |_words| "" )
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

fn brute_force_search(md5_hash: &str, wordlist: &Vec<Word>, anagram: &Word) {

}

pub(crate) fn anagram(wordlist_file: &str, anagram: &str, md5_hash: &'static str) {
    println!("Version 2 ...");
    let anagram_word = Word::new(anagram);

    let wordlist: Vec<Word> = Vec::new();

    let wordlist_in = fs::read_to_string(wordlist_file).expect("Something went wrong reading the wordlist file");

    for word_in in wordlist_in.lines() {

        if anagram_word.contains(word_in) {
            wordlist.push(Word::new(word_in));
        }
    }
    let wl = &wordlist;
    println!("Wordlist count: {}", wl.len());
    println!("Anagram md5 hash: {:x}", md5::compute(anagram));

    // wordlist.sort_by( |a,b| b.string.len().cmp(&a.string.len()) );

    let mut threads = vec![];
    // let wordlist_arc = Arc::new(&wordlist);

    for i in 0..NTHREADS {
        // let wordlist_arc_i = wordlist_arc.clone();
        // let word = wl.nth(i).unwrap();
        threads.push(thread::spawn(move || {
            // let wl = wordlist;
            brute_force_search(md5_hash, &wordlist, &anagram_word);
            combine(&wordlist[i].string, &wordlist[i].stripped, md5_hash, &wordlist, &wordlist[0], 2);
        }))
    }

    // for word in &wordlist {
    //     let a = thread::spawn(move || {
    //         combine(&(&word.string, &word.stripped), &wordlist, &anagram_word, 2);
    //     });
    // }

    for word in &wordlist {
        combine(&word.string, &word.stripped, md5_hash, &wordlist, &anagram_word, 2);
    }

    // wordlist
    //     .iter()
    //     .map( |word| {
    //         println!("{}", &word.string);
    //         &word.string
    //     })
    //     .filter( |word| combine(&(word.to_string(), strip_blanks(word)), &wordlist, &anagram_word, 2))


    //     .for_each( |word| println!("Found: {}", word));
}

// fn load_wordlist(wordlist_file: &str, anagram_word: &Word) -> &Vec<Word> {
//     let mut wordlist: Vec<Word> = Vec::new();

//     let wordlist_in = fs::read_to_string(wordlist_file).expect("Something went wrong reading the wordlist file");

//     for word_in in wordlist_in.lines() {

//         if anagram_word.contains(word_in) {
//             wordlist.push(Word::new(word_in));
//         }
//     }
//     let wl = &wordlist;
//     println!("Wordlist count: {}", wl.len());
//     &wordlist
// }


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
