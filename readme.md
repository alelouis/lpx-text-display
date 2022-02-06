# lpx-text-display

Small utility binary to display text on the 8x8 LED surface of an Novation Launchpad X.

<p align="center">
  <img src="demo.gif" alt="animated" />
</p>


Font used is public domain 8x8 [font](https://github.com/dhepper/font8x8)

## How to use
1. Clone the repo.
```bash
gh repo clone alelouis/lpx-text-display
```
2. Run the main binary with your text as first argument. Use "" for special characters.
```
cargo run your_text
```

## Customize
Main things you could want to change are **colors** and **scrolling speed**.

### Color  
Modify background and foreground on respectively line `56` and `57`.
```rust
let color = match matrix[frame+x][y] {
    true => vec![0u8, 0u8, 0u8],
    false => vec![127u8, 0u8, 0u8],
};
```
### Scrolling speed  
Modify scrolling speed at line `50` by changing the sleeping time in milliseconds.
```rust
let wait = time::Duration::from_millis(100);
```