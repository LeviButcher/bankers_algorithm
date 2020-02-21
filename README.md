# bankers_algorithm
![Rust](https://github.com/LeviButcher/bankers_algorithm/workflows/Rust/badge.svg)

## Assignment
Be sure to commit often as you both work through the assignment together.  You will present your assignment in class.

I've provided three banker input files (BankerData) to represent the allocation of resources to processes for the Banker's algorithm. 

Write an application in Rust that reads an input file with the following format:

    Each line is a set of integers separated by a space.  There will be an odd number of lines in the file.  
    The number of resources is equivalent to the number of columns in each line.  The number of processes is equivalent to the line numbers -1 (the first line is the allocation array) divided by 2.
    line 1 -> Available resources array
    First half of the lines are resources for each process already allocated to each process
    Second half of the lines are the maximum resources needed for the process to run

Implement the Banker's algorithm to determine a safe state order in which to run the processes, and output that safe state order.  If a safe state order cannot be had, output a message saying no safe state exists.  Do not copy someone else's solution from the Internet, use the videos provided and develop your own solution.
