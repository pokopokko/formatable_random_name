use std::env::{self};

use rand::random;

const CONSONANT: char = 'c';
const VOWEL: char = 'v';

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let first_arg = args[1].clone();
        if first_arg == "-h" {
            println!("Usage: cargo run -- <format_string>\n\t[c] - Consonant\n\t[v] - Vowel");
            std::process::exit(1);
        }
    }

    let default_str = String::from("ccvccvc");

    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let the_string = {
        if args.len() >= 2 {
            args[1].clone()
        } else {
            default_str
        }
    };

    println!("{}", generate_random_str(&the_string, &consonants, &vowels));
}

fn generate_random_str(input_string: &String, consonants: &[char], vowels: &[char]) -> String {
    let mut output = String::new();
    for char in input_string.chars() {
        if char == CONSONANT {
            let rand_char = consonants[random::<usize>() % consonants.len()];
            output.push(rand_char);
        } else if char == VOWEL {
            let rand_char = vowels[random::<usize>() % vowels.len()];
            output.push(rand_char);
        }
    }

    output
}
