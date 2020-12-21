## peguidex

Just for fun PEG test. It calculates simple expressions and gives true or false if UID matches to rules.

### Examples

```
cargo run expression.rules 4                                                                                                                                      [130]
0. (uid==3|uid==4|uid==5) --> true with UID=4
1. (uid==3|uid==4|uid==5)&uid<10 --> true with UID=4
2. ((uid>10&uid<100)|(uid>200&uid<300))&(uid==99|uid==201) --> false with UID=4

cargo run expression.rules 201
0. (uid==3|uid==4|uid==5) --> false with UID=201
1. (uid==3|uid==4|uid==5)&uid<10 --> false with UID=201
2. ((uid>10&uid<100)|(uid>200&uid<300))&(uid==99|uid==201) --> true with UID=201
```

**Arguments**:
1 - path to a file with rules
2 - UID for testing
