use std::io;
use futures::Future;

pub trait SolutionError {}

impl SolutionError for io::Error {}

pub trait Solution {
    type Output;
    type Error: SolutionError;

    fn solve(&mut self) -> Future<Item = Self::Output, Error = Self::Error>;
}
