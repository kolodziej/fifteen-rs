#![feature(variant_count)]
#![feature(generic_const_exprs)]

use std::collections::{HashMap, VecDeque};
use std::iter::Iterator;
use std::clone::Clone;
use std::future::pending;
use std::marker::Copy;
use std::hash::Hash;
use std::mem::variant_count;


extern crate rand;

use rand::{thread_rng, Rng};
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;

mod puzzle15;

pub trait State {
    type Change;

    fn modify(&self, change: Self::Change) -> Self;
    fn is_solved(&self) -> bool;
}

pub trait Solver<S: State> {
    fn solve(t: S) -> dyn Iterator<Item=S::Change>;
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SimpleState {
    state: u64
}

impl SimpleState {
    fn get_specific(fields: [u64; 16]) -> Self {
        let mut state: u64 = 0;
        for (index, n) in fields.iter().enumerate() {
            state ^= n << (4 * (16 - index));
        }

        SimpleState{state}
    }

    fn get_sorted() -> Self {
        let mut state: u64 = 0;
        for i in 1..=15 {
            state ^= i << (4 * (16 - i));
        }

        SimpleState{state}
    }

    fn get_random() -> Self {
        let mut fields: Vec<u64> = (0..=15).collect();

        let mut rng = thread_rng();
        fields.as_mut_slice().shuffle(&mut rng);


        Self::get_specific(fields.as_slice().try_into().expect("incorrect size of slice"))
    }

    fn get_zero(&self) -> u64 {
        (0..=15).into_iter().find(|i| self.state & (0b1111 << (4 * i)) == 0).unwrap()
    }

    fn swap_fields(&self, some: u64, zero: u64) -> u64 {
        let mut next = self.state.clone();

        let some_mask: u64 = 0b1111 << (4 * some);
        let some_val = (next & some_mask) >> (4 * some);

        next ^= some_val << (4 * zero);

        let zero_mask: u64 = u64::MAX & (0b0000 << (4 * some));
        next &= zero_mask;

        next
    }
}

impl State for SimpleState {
    type Change = puzzle15::PossibleMoves;
    fn modify(&self, change: puzzle15::PossibleMoves) -> Self {
        let mut next = self.state.clone();

        let zero = self.get_zero();


        SimpleState{state: next}
    }

    fn is_solved(&self) -> bool {
        self.state == Self::get_sorted().state
    }
}

pub struct Previous {
    mv: puzzle15::PossibleMoves,
    num_of_moves: usize,
}

pub struct BFSSolver {
}

pub struct BFSSolution<S: State> {
    solution_graph: HashMap<S, Option<Previous>>,
    current_state: S,
}

impl<S: State> BFSSolution<S> {
    fn new(p_final_state: S, p_solution: HashMap<S, Option<Previous>>) -> Self {
        BFSSolution {
            solution_graph: p_solution,
            current_state: p_final_state,
        }
    }
}

impl<S: State + Eq + Hash> Iterator for BFSSolution<S> {
    type Item = PossibleMoves;
    fn next(&mut self) -> Option<PossibleMoves> {
        if let Some(opt_previous) = self.solution_graph.get(&self.current_state) {
            if let Some(previous) = opt_previous {
                Some(previous.mv)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Solver<SimpleState> for BFSSolver {
    fn solve(init: SimpleState) -> impl Iterator<Item=PossibleMoves> {
        let mut q = VecDeque::<SimpleState>::new();

        q.push_back(init);

        while !q.is_empty() {
            if let Some(top) = q.pop_front() {
                if top.is_solved() {

                }
            }
        }
    }
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
