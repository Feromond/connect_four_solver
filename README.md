## Connect Four Solver

Cross-platform desktop app to play and analyze Connect Four. Comes with a fast AI powered by minimax and alpha–beta pruning, a clean GUI, and helpful hints about forced wins.

### Quick look

<img width="600" height="1444" alt="image" src="https://github.com/user-attachments/assets/c8275580-e3a7-4fcb-9781-c27d1b68251e" />
<img width="600" height="1462" alt="image" src="https://github.com/user-attachments/assets/401a2f02-9ab3-4786-9b25-8af7e6b03643" />



### Features

- **Play vs. AI or watch AI move**: Choose who goes first on a simple setup screen.
- **Strong search**: Minimax with alpha–beta pruning and a small transposition table speeds up analysis.
- **Smarter move ordering**: Searches center columns first to prune more effectively and play more human-like moves.
- **Immediate-win checks**: Detects mate-in-1 for the side to move and avoids root-level blunders that allow an immediate reply win.
- **Heuristic evaluation**: Scores lines of four based on counts (2/3 in a row with empties) and emphasizes center control.
- **Responsive board**: Board scales to available window size; polished colors and simple visuals.
- **Forced-win indicator**: When applicable, shows “AI can force a win in N turns.”

### How it works (high level)

- **Game model**: A `Board` tracks a 7×6 grid, current player, legal moves, wins/draws, and applies moves.
- **Search**: The `Solver` runs minimax with alpha–beta pruning. It caches `(position, depth)` results in a `HashMap` to avoid recomputation.
- **Ordering**: Candidate columns are ordered center-out to improve pruning and play strength.
- **Tactics**: Before full search, it checks for immediate winning moves; at the root it filters out moves that allow the opponent an instant win.
- **Evaluation**: For non-terminal nodes, a heuristic sums all 4-cell windows, rewarding threats (2/3 in a row with empties) and center occupancy; terminal wins/losses get large scores.
- **Depth**: Default search depth is set in the UI layer (currently 9 plies); adjust in `src/app.rs` if you want a faster or stronger AI.

## Build and run

- **Requirements**: Latest stable Rust toolchain.
- **Build**:

```bash
cargo build
```

- **Run**:

```bash
cargo run
```

## Controls

- **Setup**: Pick who moves first (Human or AI).
- **Play**: Click a column to drop a piece.
- **New Game** returns to setup; **Reset Board** clears the current board.

## Notes

- On Windows, if `icon.ico` exists, it will be embedded; otherwise a transparent placeholder is used.
- On macOS, the `.app` icon is provided via bundling metadata; the runtime uses a default icon.

## Tech

- **Rust**, **egui/eframe** for the native GUI
- **env_logger + log** for simple logging
