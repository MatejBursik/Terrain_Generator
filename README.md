# Terrain Generation using Perlin Noise

Writen in Rust with OpenGL to visualise the Perlin Noise in the form of 3D Terrain. The idea comes from [The Coding Train](https://www.youtube.com/watch?v=IKB1hWWedMk) combined with my interest to make a video games.

There are 2 modes (Top down view, First person view)

## Controls
| Key | Description |
|---|---|
| W, S, A, D | Forward, Backward, Left, Right (Movement keys) |
| F1 | You switch to first person view in which you can walk around the terrain with movement keys. |
| F2 | You switch to top down view in which you can see the whole generated terrain. You can also use movement keys to move the generated terrain around. |

## Basic 3D Game
Trying to make a basic 3D Game using ray casting with a similar approach to [Wolfenstein 3D](https://en.wikipedia.org/wiki/Wolfenstein_3D)

## Issues
- coloring walls has some issues, overall approach to displaying perspective needs to be rethinked
- need to provide a fragment shader for the lines representing the walls, not all drivers provide a default
- slow movements and rotation, probably caused by incorrect use of elapsed_time, temporary solution (speed: 5000.0)
