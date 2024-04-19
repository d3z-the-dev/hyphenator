# hyphenator

**hyphenator** is a simple **Rust** library for segmenting words into syllables, using a simple vowel-consonant combination approach to split words into syllables.

## Usage

To use the hyphenator, add this to your `Cargo.toml`:
```toml
[dependencies]
hyphenator = "0.1.0"
```

## Example

```rust
use hyphenator::split;

fn main() {
    let word = "Hyphenator";
    let results = split(word);
    
    println!("{:?}", results[0]);
}
```