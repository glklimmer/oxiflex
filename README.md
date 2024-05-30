# oxiflex

Constraint Satisfaction Problem solver for [MiniZinc](https://www.minizinc.org/).

## Usage

The main idea is to use oxiflex as a solver for MiniZinc.

```bash
minizinc --solver oxiflex problems/simple/simple.mzn
```

## Direct Usage

It is possible to use oxiflex directly with FlatZinc files instead of MiniZinc files.

```bash
cargo run problems/simple/simple.fzn
```

## Installation

In order for oxiflex to work, it needs a working MiniZinc installation.

### Linux

You need to tell MiniZinc about oxiflex using a `.msc` file.

```msc
{
  "name" : "oxiflex",
  "version": "0.0.1",
  "id": "org.oxiflex.oxiflex",
  "executable": "../../../bin/oxiflex"
}
```

Save this at `/share/minizinc/solvers` as `oxiflex.msc` within your MiniZinc Installation.
This is usually something like `/usr/local`.

Then you need to provide MiniZinc the oxiflex binary.
Make sure you have `cargo` installed. You can install it using [rustup](https://www.rust-lang.org/tools/install).

Compile oxiflex using

```bash
cargo build --release
```

Now copy the oxiflex binary to the `/bin` folder within your MiniZinc Installation.
Something like this from within oxiflex:

```bash
cp target/release/oxiflex $MINIZINC_PATH/bin
```
