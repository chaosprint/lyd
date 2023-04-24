# lyd (wip)

`lyd` (the Norwegian/Danish word for `audio`) is a rust library that offers an audio graph, nodes, and message system for dynamic audio/music control including node adding, removing and modifying.

- minimal but intuitive apis (read more below)
- consideration for optimizations: wasm, embedded devices, etc.
- try to combine these two

read more about the philosophy of `lyd` in [this blog post (wip, do not click)]().

## usage

```
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

if you can contribute a node, that would be great!

follow the procedure here.

besides that, bug report and feature suggestions are welcomed.