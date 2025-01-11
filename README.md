# Terrain Generation using Perlin Noise

Writen in Rust with OpenGL to visualise the Perlin Noise in the form of 3D Terrain. The idea comes from [The Coding Train](https://www.youtube.com/watch?v=IKB1hWWedMk) combined with my interest in making video games.

There are 2 modes (Landscape view, First person view):
- In landscape view, you can see a segment of the generated terrain. You can also use movement keys to change which segment of the generated terrain is being displayed.
- In first person view, you can walk around the segment of the terrain that you saw in the landscape view.

## Controls
| Key | Description |
|---|---|
| W, A, S, D | Forward, Left, Backward, Right (Movement keys) |
| F1 | Switch between the 2 modes |

## Perlin noise generator
Custom perlin noise generator based on the description from this video ( [How Does Perlin Noise Work? by Doggo's Science 2](https://www.youtube.com/watch?v=9B89kwHvTN4) ).

## Basic 3D Game
** **Needs to be remade to function with terrain generation** **

Trying to make a basic 3D Game using ray casting with a similar approach to [Wolfenstein 3D](https://en.wikipedia.org/wiki/Wolfenstein_3D).

## Issues
- coloring walls has some issues, overall approach to displaying perspective needs to be rethinked
- need to provide a fragment shader for the lines representing the walls, not all drivers provide a default
- slow movements and rotation, probably caused by incorrect use of elapsed_time, temporary solution (speed: 5000.0)
