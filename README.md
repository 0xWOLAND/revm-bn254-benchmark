# Benchmarks

| Operation   | Cycle Count |
| ----------- | ----------- |
| decode-g1   | 10,194      |
| decode-g2   | 27,935,766  |
| sum-g1      | 30,393      |
| mul-g1      | 7,232,320   |
| sum-g2      | 103,638     |
| mul-g2      | 30,495,032  |
| pairing     | 78,451,766  |
| inv         | 388,868     |
| miller-loop | 33,205,151  |
| final-exp   | 45,271,739  |

# Ops

## `G1` BN Curve

- `G1` Add
- `G1` Mul

## `G2` Twisted BN Curve

- `G2` Add
- `G2` Mul

## `Gt` Pairing

- `Gt` Pow
- `Gt` Inv
- `Gt` Exp

## Batch Pairing Functions

- `miller_loop_batch`
- `pairing`

# Features

- Random elliptic curve points

# Credits

- Credit to Michael Zaikin for the [initial benchmark](https://github.com/m-kus/sp1-bn254-benchmark)
