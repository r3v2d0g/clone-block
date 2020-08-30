# A very simple macro that clones a list of variables before calling an expression

[![img](https://img.shields.io/crates/l/clone-block.svg)](https://github.com/r3v2d0g/clone-block/blob/main/LICENSE.txt) [![img](https://img.shields.io/crates/v/clone-block.svg)](https://crates.io/crates/clone-block) [![img](https://docs.rs/clone-block/badge.svg)](https://docs.rs/clone-block)

Based on this tweet: <https://twitter.com/untitaker/status/1299812136202493953>

```rust
use clone_block::clone;
use std::thread;

let foo = "foo".to_string();

let thread = thread::spawn(
    clone!(foo; move || {
        let foobar = format!("{}bar", foo);
        foobar
    })
);

let foobar = thread.join();
let foobaz = format!("{}baz", foo);
```


## License

> This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
