# The Lost Vessel

The Lost Vessel is a top down video game in pixel art style taking place in an unknown spaceship on earth's orbit.

## Game engine

This project is using `Bevy` as his game engine.

## Systems

### Tilemap

Right now, the tilemap is specified as a static string in a source file but I'm planning on
making it an asset that can be created with a level editor.

### Dialogs

Dialogs can be written as `.dialog.ron` files and are loaded when the game starts
