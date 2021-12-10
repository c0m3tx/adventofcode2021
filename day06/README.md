# Day 6

I started using an iterative approach, building a new vector from the previous one.

It worked for the first part, but not for the second one (it's too inefficient), so I implemented a new version which uses a fixed-dimension array to store the number of fishes.

Therefore, out of curiosity, I benchmarked the two solutions (file `src/bench.rs`).

Thanks to criterion.rs for the benchmarking tool, it's fantastic!

## Benchmark results

I used a small starting input

```rust
vec![3, 4, 3, 1, 2]
```

for the two benchmarks, with 200 steps.

Here are the results:

```
iterative version       time:   [1.7347 s 1.7485 s 1.7629 s]
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild

optimized version       time:   [73.866 ns 74.811 ns 76.329 ns]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
```
