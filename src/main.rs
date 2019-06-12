use std::env;

mod anagram1;
mod anagram2;
mod anagram3;

const WORDLIST_FILE: &str = "C:\\Users\\120637\\git\\anagram\\wordlist";
// const WORDLIST_FILE: &str = "/Users/John/Documents/GitHub/anagram/wordlist";
const ANAGARAM: &str = "poultry outwits ants";
const MD5_HASH_EASY: &str = "e4820b45d2277f3844eac66c903e84be";
// const MD5_HASH_MEDIUM: &str = "23170acc097c24edb98fc5488ab033fe";
// const MD5_HASH_HARD: &str = "665e5bcb0c20062fe8abaaf4628bb154";

fn main() {
    let args: Vec<String> = env::args().collect();

    let version: i32 = args[1].trim().parse().unwrap_or(0);
    println!("Running version {}", version);

    match version {
        1 => crate::anagram1::anagram(WORDLIST_FILE, ANAGARAM, MD5_HASH_EASY),
        2 => crate::anagram2::anagram(WORDLIST_FILE, ANAGARAM, MD5_HASH_EASY),
        3 => crate::anagram3::anagram(WORDLIST_FILE, ANAGARAM, MD5_HASH_EASY),
        _ => println!("We're not there yet."),
    }
}