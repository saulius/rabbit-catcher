# rabbit-catcher

Solution to [Trustpilot anagram challenge](http://followthewhiterabbit.trustpilot.com/) in Rust. Uses Rust nightly toolchain.

## Usage:

On Mid 2015 Macbook Pro with 2,2 GHz Intel Core i7:

```
$ rustc --version
rustc 1.21.0-nightly (aac223f4f 2017-07-30)

$ cargo run --release

Found anagram: "..." - "23170acc097c24edb98fc5488ab033fe" (in 6 seconds)
Found anagram: "..." - "e4820b45d2277f3844eac66c903e84be" (in 15 seconds)
```

## Possible optimizations:

* [ ] Use `Vec<u8>` instead of `String` as word representation on `Word`
* [ ] Do not generate combinations of 3 words if 2 words aren't a subset of target phrase - early termination, custom combinator implementation
* [ ] Parallelize: one thread generates combinations, other thread checks permutations
* [ ] Parallelize: multiple threads can generate combinations, each should start at different position
