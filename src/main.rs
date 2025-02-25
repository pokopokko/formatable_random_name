use std::env::{self};

use rand::random;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut fixed_letters = String::new();

    if args.len() >= 2 {
        // print help information
        if args[1] == "-h"
            || args[1] == "--help"
            || (!args[1].contains("c") && !args[1].contains("v"))
        {
            println!("Usage: cargo run -- <format_string>\nOR (for a binary build of the program)\nrandom_name <format_string>\n\nHow to format:\n\t[c] - A 'c' in the format string represents a consonant to be in this position.\n\t[v] - A 'v' in the format string represents a vowel to be in this position.\nNote: inputting no string will result in the default string \"ccvccvc\" being used.");
            std::process::exit(1);
        }
        if args.len() >= 3 {
            fixed_letters = args[2].clone();
        }
    }

    // default string for random generation in case the user did not input one in.
    let default_str = String::from("cvcvc");

    let the_string = {
        if args.len() >= 2 {
            args[1].clone()
        } else {
            default_str
        }
    };

    let generated_string = generate_name_string(&the_string);
    // only print if a string was actually generated
    if !generated_string.is_empty() {
        if fixed_letters.is_empty() {
            println!("{}", generated_string);
        } else {
            let mut final_string = String::new();
            let mut gen_chars = generated_string.chars();

            for ch in fixed_letters.chars() {
                if ch.is_alphabetic() {
                    if let Some(gen_ch) = gen_chars.next() {
                        final_string.push(ch);
                    }
                } else if ch == ' ' {
                    final_string.push(' ');
                    if let Some(gen_ch) = gen_chars.next() {
                        final_string.push(gen_ch);
                    }
                } else if ch == '.' {
                    if let Some(gen_ch) = gen_chars.next() {
                        final_string.push(gen_ch);
                    }
                }
            }

            println!("{}", final_string);
        }
    }
}

/// Creates a new random string based on the input string.
///
/// # Params
/// `input_string` - The input string to base the randomized string on. `c` represents a consonant and `v` represents a vowel.
///
/// # Returns
/// - A `String` containing the randomized name string.
fn generate_name_string(input_string: &String) -> String {
    // define static variables for this function
    static CONSONANT: char = 'c';
    static VOWEL: char = 'v';
    static CONSONANTS: &[char] = &[
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];
    static VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

    // create new string
    let mut output = String::new();
    for char in input_string.chars() {
        if char == CONSONANT {
            let rand_char = CONSONANTS[random::<usize>() % CONSONANTS.len()];
            output.push(rand_char);
        } else if char == VOWEL {
            let rand_char = VOWELS[random::<usize>() % VOWELS.len()];
            output.push(rand_char);
        }
    }

    output
}
