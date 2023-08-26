# conll
conll is a Rust Crate for efficiently parsing
[Treebanks](https://en.wikipedia.org/wiki/Treebank) in the
[CoNLL(-U)](https://universaldependencies.org/format.html) format.

# Usage
You can use the `parse` program bundled with the crate, or you can use the
library programmatically with the following usage:
```rust
let lines: Vec<String>;

let treebank = conll::conllu::parser::parse(lines).unwrap();
```

# Performance
The ConLL-U parser is quite fast. Here is the output of executing the binary
using `time`, on a **14MB** file.
```
$ time ./target/release/parse nl_alpino-ud-dev.conllu -s

real    0m0.074s
user    0m0.054s
sys     0m0.019s
```

For comparison, here it is on a **195MB** file.
```
time ./target/release/parse de_hdt-ud-train.conllu -s

real    0m5.006s
user    0m3.866s
sys     0m1.116s
```
