use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::num::ParseIntError;
use std::collections::{HashMap, HashSet};
use levenshtein::levenshtein;
use std::iter::FromIterator;

use util;

type InputItem = Result<String, Error>;

type Input = Iterator<Item = InputItem>;

#[derive(Clone, Debug)]
pub struct KeyLetters {
    twice: HashSet<char>,
    thrice: HashSet<char>
}

impl KeyLetters {
    pub fn from(map: &mut HashMap<char, i32>) -> KeyLetters {
        let twice: &mut HashSet<char> = &mut HashSet::new();
        let thrice: &mut HashSet<char> = &mut HashSet::new();

        for (c, i) in map.into_iter() {
            match i {
                2 => { twice.insert(*c); },
                3 => { thrice.insert(*c); },
                _ => ()
            }
        }

        KeyLetters {
            twice: twice.clone(),
            thrice: thrice.clone()
        }
    }
}

pub fn find_key_letters(s: String) -> KeyLetters {
    let counts = &mut util::string::count_letters(s);
    KeyLetters::from(counts)
}

pub fn solution1(ids: &mut Input) -> Result<i32, Error> {
    let twice: &mut i32 = &mut 0;
    let thrice: &mut i32 = &mut 0;

    ids.fold(Ok(()), |acc, id| {
        if acc.is_err() {
            return acc;
        }

        match id {
            Ok(s) => {
                let key_letters = find_key_letters(s);
                if !key_letters.twice.is_empty() {
                    *twice += 1;
                }

                if !key_letters.thrice.is_empty() {
                    *thrice += 1;
                }

                Ok(())
            },
            Err(e) => Err(e)
        }
    }).map(|_| {
        println!("2: {:?}", twice);
        println!("3: {:?}", thrice);
        (*twice) * (*thrice)
    })
}

pub fn solution2(ids: &mut Input) -> Result<i32, Error> {
    let bucket: &mut Vec<String> = &mut Vec::new();
    util::collections::collect_into(bucket, ids)
        .map(|_| {
            let cloned = bucket.clone();
            let cnt = cloned.len();
            let mut search_vec: HashMap<&String, Vec<&String>> = HashMap::new();

            bucket.iter().fold(0, |idx, a| {
                let begin = idx + 1;

                if (begin >= cnt) {
                    return idx;
                }

                let comparables: Vec<&String> = cloned.iter().skip(begin).collect();

                search_vec.insert(a, comparables);

                idx + 1
            });

            let result = search_vec.keys()
                .fold(None, |acc, a| {
                    if acc.is_some() {
                        return acc;
                    }

                    let comparables = search_vec.get(a)?;

                    comparables.iter().fold(None, |acc, b| {
                        if acc.is_some() {
                            return acc;
                        }

                        let distance = levenshtein(a, b);

                        match distance {
                            1 => Some((a.clone().clone(), b.clone().clone())),
                            _ => None
                        }
                    })
                })
                .map(|(a, b)| {
                    let a_chars: Vec<char> = a.chars().collect();
                    let b_chars: Vec<char> = b.chars().collect();

                    let mut chars: Vec<char> = Vec::new();

                    a_chars.iter().fold(0, |idx, c1| {
                        let c2 = b_chars[idx];

                        if *c1 == c2 {
                            chars.push(*c1);
                        }

                        idx + 1
                    });

                    String::from_iter(chars)
                });

            result.map(|chars| println!("Actual solution for #2 for day 2 is {:?}", chars));
            
            42
        })
}

pub fn solve(question: i32) {
    let file = File::open("data/day02/1.txt").expect("Input #1 for Day 01 is missing.");

    let ids: &mut Input = &mut BufReader::new(file).lines();

    let result = match question {
        1 => solution1(ids),
        2 => solution2(ids),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Unknown question"))
    };

    match result {
        Ok(r) => println!("Solution for #{:?} is {:?}", question, r),
        Err(e) => println!("Failed to solve question #{:?} due to {:?}", question, e)
    }
}
