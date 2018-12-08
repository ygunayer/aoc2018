use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::num::ParseIntError;
use std::collections::HashMap;
use std::fmt::{Formatter, Debug, Result as FormatterResult};

use util;

fn as_io_error(e: ParseIntError) -> Error {
    Error::new(ErrorKind::InvalidInput, e.to_string())
}

type InputItem = Result<i32, Error>;
type Input = Iterator<Item = InputItem>;

struct Solution2Iterator<'a> {
    bucket: &'a mut HashMap<i32, i32>,
    result: Option<i32>,
    sum: i32,
    is_first: bool
}

impl<'a> Debug for Solution2Iterator<'a> {
    fn fmt(&self, f: &mut Formatter) -> FormatterResult {
        write!(f, "Solution2Iterator {{ result: {:?}, sum: {:?}, map: {:?} }}", self.result, self.sum, self.bucket)
    }
}

impl<'a> Solution2Iterator<'a> {
    pub fn init(bucket: &'a mut HashMap<i32, i32>) -> Solution2Iterator {
        Solution2Iterator {
            bucket,
            result: None,
            sum: 0,
            is_first: true
        }
    }

    pub fn add(self, n: i32) -> Solution2Iterator<'a> {
        let sum = self.sum + n;
        let prev_count = *self.bucket.get(&sum).unwrap_or(&0);
        let next_count = prev_count + 1;

        if !self.is_first {
            self.bucket.insert(sum, next_count);
        }

        if next_count > 1 {
            Solution2Iterator {
                bucket: self.bucket,
                result: Some(sum),
                sum,
                is_first: false
            }
        } else {
            Solution2Iterator {
                bucket: self.bucket,
                result: None,
                sum,
                is_first: false
            }
        }
    }
}

pub fn solution1(iter: &mut Input) -> InputItem {
    iter.fold(Ok(0), |acc, num| {
        if acc.is_err() {
            return acc;
        }

        match num {
            Ok(x) => Ok(x + acc.unwrap()),
            Err(e) => Err(e)
        }
    })
}

pub fn solution2(iter: &mut Input) -> InputItem {
    let numbers: &mut Vec<i32> = &mut Vec::new();

    util::collections::collect_into(numbers, iter)
        .and_then(|_| {
            util::collections::repeated(numbers, 1000); // huge but works

            let bucket: &mut HashMap<i32, i32> = &mut HashMap::new();

            let result = numbers.into_iter()
                .fold(Solution2Iterator::init(bucket), |acc, num| {
                    match acc.result {
                        Some(_) => acc,
                        None => acc.add(*num)
                    }
                });
            
            match result.result {
                Some(n) => Ok(n),
                None => Err(Error::new(ErrorKind::Other, "No solution found"))
            }
        })
}

pub fn solve(question: i32) {
    let file = File::open("data/day01/1.txt").expect("Input #1 for Day 01 is missing.");

    let numbers: &mut Input = &mut BufReader::new(file).lines().map(|line|
        line.and_then(|st| st.parse::<i32>().map_err(|e| as_io_error(e)))
    );

    let result = match question {
        1 => solution1(numbers),
        2 => solution2(numbers),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Unknown question"))
    };

    match result {
        Ok(r) => println!("Solution for #{:?} is {:?}", question, r),
        Err(e) => println!("Failed to solve question #{:?} due to {:?}", question, e)
    }
}
