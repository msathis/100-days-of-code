# 100 Days Of Code - Log

### Day 0: May 16, 2019 - Hanoi Tower
**Problem:** [Geeksforgeeks](https://www.geeksforgeeks.org/c-program-for-tower-of-hanoi/)\
**Code:** [Hanoi Tower](src/day1/mod.rs)

Important to remember the order of moves. There are two moves required to push one disk from A to C. 
First move the disk from A to B using C as axillary. Then move it from B to C using A as axillary. 


### Day 1: May 17, 2019 - Matrix Multiplication
**Problem:** [Geeksforgeeks](https://www.geeksforgeeks.org/matrix-chain-multiplication-dp-8/)\
**Code:** [Matrix Multiplication](src/day2/mod.rs)

We have to try all the possible combinations of putting parentheses 
and pick the one which is yielding minimal multiplications. Start with recursive one 
then build the dynamic programming approach from it.