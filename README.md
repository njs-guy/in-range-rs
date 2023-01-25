# In range

A Rust library to return true if a number is in range.

## Usage

First, install the library with

```bash
cargo add in_range
```

Then, in code:

```rs
use in_range::in_range;

// Args: value, min, max
let result = in_range(10, 1, 15);

print!("{}", result);
// Prints true;
```

You can also use `not_in_range` to return *false* when value is in range.

```rs
use in_range::not_in_range;

// Args: value, min, max
let result = not_in_range(25, 1, 15);

print!("{}", result);
// Prints true;
```

## License

In range uses the Rust standard MIT/Apache-2.0 dual license for best
compatibility. See
[LICENSE-APACHE.txt](LICENSE-APACHE.txt) and [LICENSE-MIT.txt](LICENSE-MIT.txt)
for the full licenses.
