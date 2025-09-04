# Connect Four Solver (GUI)

Simple cross-platform GUI to play and analyze Connect Four using a minimax + alpha-beta pruning solver.

## Build and Run

- Requirements: Rust (stable). On macOS, no extra setup needed.
- Build:

```bash
cargo build
```

- Run:

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Notes

- On Windows, the build script embeds `icon.ico` if available.
- On other platforms, a transparent placeholder icon is used if `icon.ico` is missing.

## Controls

- Choose who goes first on the setup screen.
- Click a column during your turn to drop a piece.
- New Game resets to setup; Reset Board clears the board mid-game.
