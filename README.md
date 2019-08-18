

# Hyperloglog


HyperLogLog is an algorithm approximating the number of distincts elements in a set.
More precisely, it's a probabilistic data-structure.

Original Paper: http://algo.inria.fr/flajolet/Publications/FlFuGaMe07.pdf


This is an basic implementation in Rust. It performs badly on small datasets.


# Algorithm

Inserting a element involves 4 steps: (N=16)

- computes its hash
- take the first N bits from this hash, this define a register address
- find the leftmost 1 bit in the remaining bits
- compare its offset to the register value, if more, update it with the offset value

Estimation is done by computing the sum of 2 of the negative power of each register value,
then multiplicating this result with some constants. 
(This is a very straightforward explanation, if you want more details, I encourage you to read the paper).




# More

- HLL+: https://research.neustar.biz/2013/01/24/hyperloglog-googles-take-on-engineering-hll/
