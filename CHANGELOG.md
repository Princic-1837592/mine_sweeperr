# mine_sweeperr CHANGELOG

## 0.3.0
Many major changes:
- Added utility functions to get `height`, `width` and number of mines from a `MineSweeper` object.
- Added support for `wasm`, except for the `rand` crate which is not compatible with `wasm`.
- Added format option `{:#}` to print the board with numbers.
- Fixed an error that swapped `height` and `width` in the constructor.

## 0.2.0
Added hash implementation of the game. Added support for custom random generator.

## 0.1.0
Initial release. The matrix implementation is ready to use.
