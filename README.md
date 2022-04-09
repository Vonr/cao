# CaO
[![Crates.io](https://img.shields.io/crates/v/calcium-oxide)](https://crates.io/crates/calcium-oxide)

Calc-ium Oxide, a simple stack based command line calculator written in rust

## Examples
```sh
# Calculate golden ratio
$ cao 1 5 sqrt + 2 /
1.618033988749895

# Calculate the greatest common divisor of 372 and 64
$ cao 372 64 gcd
4

# Calculate the first 20 fibonnaci numbers
$ cao 1 21 seq fib map
1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 6765

# Calculate the ratio of the number of numbers in (1..1000000] that
# are prime to the number that are composite to 5 decimal places
$ cao 1 1000000 seq len 0 store prime map + fold dup 0 take swap - / 5 dp
0.08518
```

## Documentation

Refer to [DOCS.md](https://github.com/Vonr/cao/blob/master/DOCS.md)
