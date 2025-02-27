This file contains the GDExtension code used in the game.

## Note about unit tests

Because of how the Godot integration works, most code cannot be tested through unit tests, testing needs to be done at the Godot call site. Any code that must traverse the FFI boundary, cannot be tested in unit tests.