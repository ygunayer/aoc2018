use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::num::ParseIntError;
use std::collections::{HashMap, HashSet};

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

fn count_letters(s: String) -> HashMap<char, i32> {
    let map: &mut HashMap<char, i32> = &mut HashMap::new();

    for c in s.chars().into_iter() {
        *map.entry(c).or_insert(0) += 1;
    }

    map.clone()
}

pub fn find_key_letters(s: String) ->  KeyLetters {
    let counts = &mut count_letters(s);
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

pub fn solve(question: i32) {
    let file = File::open("data/day02/1.txt").expect("Input #1 for Day 01 is missing.");

    let ids: &mut Input = &mut BufReader::new(file).lines();

    let result = match question {
        1 => solution1(ids),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Unknown question"))
    };

    match result {
        Ok(r) => println!("Solution for #{:?} is {:?}", question, r),
        Err(e) => println!("Failed to solve question #{:?} due to {:?}", question, e)
    }
}
