# mine_sweeperr CHANGELOG

## 0.3.0
Many major changes:
- Added utility functions to get `height`, `width` and number of mines from a `MineSweeper` object.
- Added support for `wasm`, except for the `rand` crate which is not compatible with `wasm`.
- Added format option `{:#}` to print the board as emojis.
- Added format option `{:.0}` to print the board with row and column numbers.
- Fixed an error that swapped `height` and `width` in the constructor.
- Improved the opening algorithm to make it propagate from safe opened cells:
cells that are already open but surrounded by enough flags will open their closed neighbors.

## 0.2.0
Added hash implementation of the game. Added support for seeded random generator.

## 0.1.0
Initial release. The matrix implementation is ready to use.
