<h3 align="center">
  <br />
  <img src="https://user-images.githubusercontent.com/1794291/193512543-73eeea2a-86d7-4a23-a86e-5efd21202c03.png" alt="logo" width="800" />
  <br />
  <br />
  <br />
</h3>
# Rust for Node.js Developers

> Examples of [Rust](https://rustlang.org/) examples compared to [Node.js](https://nodejs.org/) for learning

[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/jason-shen/rust-for-nodejs-developers/master/LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](#contributing)

This guide full of examples is intended for people learning Rust that are coming from Node.js, although the vice versa can work too. This is not meant to be a complete guide and it is assumed that you've gone through the [Getting Started of Rust](https://www.rust-lang.org/learn/get-started) tutorial. This guide is meant to be barely good enough to help you at a high level understand how to do *X* in *Y* and doing further learning on your own is of course required.

## Contents

- [Examples](#examples)
  - [comments](#comments)
  - [printing](#printing)
  - [logging](#logging)
  - [variables](#variables)


## Examples

All sample code is available in [examples/](examples/)

### comments
---

#### Node.js

```node
// this is a line comment

/*
 this is a block comment
*/
```

#### Rust

```rust
fn main() {
	// this is a line comment

	/*
	   this is a block comment
	*/
}
```

**[⬆ back to top](#contents)**

### printing
---

#### Node.js

```node
console.log('print to stdout')
console.log('format %s %d', 'example', 1)
console.error('print to stderr')
```

Output

```bash
print to stdout
format example 1
print to stderr
```

#### Rust

```rust

fn main() {
	println!("print to stdout")
	println!("format %s %v\n", "example", 1)
	eprintln!f("print to stderr")
}
```

Output

```bash
print to stdout
format example 1
print to stderr
```

**[⬆ back to top](#contents)**

### logging
---

#### Node.js

```node
console.log((new Date()).toISOString(), 'hello world')
```

Output

```bash
2021-04-11T20:55:07.451Z hello world
```

#### Rust
you will need couple of crates from crates.io for this example
```rust 
log = "0.4.0"
env_logger = "0.9.0"
```
```rust

use log::debug;
use log::error;
use log::info;
use log::warn;
fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    debug!("hello world!");
    error!("{}", "hello world!");
    info!("{:?}", "hello world!");
    warn!("{:#?}", "hello world!");
}
```

Output

```bash
[2022-10-03T08:06:04Z DEBUG logging] hello world!
[2022-10-03T08:06:04Z ERROR logging] hello world!
[2022-10-03T08:06:04Z INFO  logging] "hello world!"
[2022-10-03T08:06:04Z WARN  logging] "hello world!"
```

**[⬆ back to top](#contents)**

### variables
---

#### Node.js

```node
// function scoped
var foo = 'foo'

// block scoped
let bar = 'bar'

// constant
const qux = 'qux'
```

#### Rust

(variables are block scoped in Rust)

```rust
fn main() {
	// explicit
	let foo:&str = "foo";

	// type inferred
	let bar = "foo";

	// constant
	const QUX:&str = "Qux";

    println!("{} {} {}", foo, bar, QUX);
}
```

**[⬆ back to top](#contents)**
