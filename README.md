# benchmark
```rust
#[derive(Debug, DekuRead)]
pub struct Test {
    a: u8,
    b: u8,
}
```
```
deku read new           time:   [10.862 ns 10.994 ns 11.150 ns]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

deku                    time:   [23.186 ns 23.337 ns 23.510 ns]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

manual                  time:   [4.5398 ns 4.5656 ns 4.6002 ns]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
```
