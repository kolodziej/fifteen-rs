# Fifteen Puzzle Game solver in Rust

## How board state will be implemented?

Each of 16 fields keeps number from 1 to 15 (16th field is empty - marked with zero). 4 bits are enough to save each
field state. It is possible to keep whole board in a 64-bit integer. However on each move we need to verify if it is
possible. What will be the impact on CPU time used to solve puzzles when empty field will be saved in separate field?
It is going to be a part of this analysis.

## C++ and Rust implementation

Both Rust and C++ implement all required data structures (queues, stack, hashmaps and hashsets) in standard library.
I'll use these implementation. In next step I'll replace some C++ sets with 

## Tasks

 - [X] Create logic of moving tiles on board
 - [ ] Implement simple BFS solver that counts moves
 - [ ] Implement solver framework that restores moves order
 - [ ] Reimplement BFS and DFS solver using framework
 - [ ] Implement A* based algorithm
