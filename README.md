# Terrain Generation using Perlin Noise

Writen in Rust with OpenGL to visualise the Perlin Noise in the form of 3D Terrain.

There are 2 modes (Landscape view, First person view):
- In landscape view, you can see a segment of the generated terrain. You can also use movement keys to change which segment of the generated terrain is being displayed.
- In first person view, you can walk around the segment of the terrain that you saw in the landscape view.

## Controls
| Key | Description |
|---|---|
| W, A, S, D | Forward, Left, Backward, Right (Movement keys) |
| Q | Rotate clockwise |
| E | Rotate anti-clockwise |
| F1 | Switch between the 2 modes (not implemented yet) |

## Perlin noise generator
Custom perlin noise generator based on the description from this video ( [How Does Perlin Noise Work? by Doggo's Science 2](https://www.youtube.com/watch?v=9B89kwHvTN4) ).

## Inspiration:
- [The Coding Train](https://www.youtube.com/watch?v=IKB1hWWedMk)
- [Zarch](https://en.wikipedia.org/wiki/Zarch)
