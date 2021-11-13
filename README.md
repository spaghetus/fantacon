# Fantascan

Fantascan is a fantasy console inspired by a video i watched about the atari quadrascan once.

Every script in the `routines` directory will run every frame. The following constants are available:

* `DRAW` - A table containing named groups of draw instructions.
* `DRAW_DIRTY` - Set this to true if you have changed any draw instructions.
* `WORK` - Write any data you need to store here. This might be used for save states if I ever continue this.
* `SAVE` - Does nothing.
* `SAVE_DIRTY` - Does nothing.
* `UP`, `DOWN`, `LEFT`, `RIGHT`, `Z`, `X` - Reads user input.

I don't particularly like Lua, so if I ever do anything with this I'm going to port it to a more normal language.