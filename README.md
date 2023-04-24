# lyd

`lyd` (the Norwegian/Danish word for `audio`) is a rust library that offers an audio graph, nodes, and message system for dynamic audio/music control including node adding, removing and modifying.

- minimal but intuitive apis (read more below)
- consideration for optimizations: wasm, embedded devices, etc.
- try to combine these two

read more about the philosophy of lyd in [this blog post(wip)]().

## usage

```rust
use lyd::{context, node::*};

fn main() {
    let mut ctx = context().frames(128).channels(2).sr(44100);
    ctx.add_sig("output", vec![sin_osc().freq("~fm"), mul(0.1)]);
    ctx.add_sig("~fm", vec![sin_osc().freq(black_box(200.)), mul(300.), add(600.)]);
    ctx.next_block();
    println!("output buffers: {:?}", ctx.buffers.get("output").unwrap());
}
```

more in the `examples` folder.

## contribution

if you can contribute a node, that would be great!

follow the procedure here.

besides that, bug report and feature suggestions are welcomed.