# lyd (wip)

`lyd` (the Norwegian/Danish word for `audio`) is a rust library that offers an audio graph, nodes, and message system for dynamic audio/music control including node adding, removing and modifying.

`lyd` aims to offer and balance:

1. minimal but intuitive apis (read more below)
2. consideration for optimizations: wasm, embedded devices, etc.

## usage

```rust
use lyd::*;

fn main() {
    let mut ctx = context().frames(4).channels(2).sr(44100).build(
        &[
            &[sin_osc!(440.0)],
        ]
    );
    println!("{:?}", ctx.next_block());
}
```

more in the `examples` folder.

## contribution

poc so far, many bugs, will break the apis.

but feel free to write your suggestions at issues