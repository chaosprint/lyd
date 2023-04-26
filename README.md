# lyd (wip)

`lyd` (the Norwegian/Danish word for `audio`) is a rust library that offers an audio graph, nodes, and message system for dynamic audio/music control including node adding, removing and modifying.

`lyd` aims to offer and balance:

1. minimal but intuitive apis (read more below)
2. consideration for optimizations: wasm, embedded devices, etc.

## usage

```rust
use lyd::*;

fn main() {
    let mut ctx = context().channels(2).frames(1024).sr(48000).build(&[
        ("~mod", &[sin_osc().freq(10.0).amp(300.), add(500.1)]),
        ("out", &[sin_osc().freq("~mod"), add(0.1)]),
    ]);
    println!("{:?}", ctx.next_block());
}
```

more in the `examples` folder.

## contribution

feel free to write your suggestions on github

> poc phase with many bugs and the apis can significantly change