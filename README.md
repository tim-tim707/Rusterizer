# Rustcaster
> A raycaster implementation build in rust web assembly.
> Inspired by One Lone Coder 3D Engine serie https://github.com/OneLoneCoder/videos
> Inspired by rust wasm tutorial: https://rustwasm.github.io/book/game-of-life/introduction.html

## Introduction
In order to display 3D models, computers use either Raytracing
(light simulation) or Raycasting algorithm. The latter is much faster and uses
triangles as well as linear algebra to render images.

#### Pipeline overview:
Triangles are subjects to the following actions:
- Stored in a list according to the 3D models in the scene to be
  rendered
- Transformed (rotation, translation, scaling) via matrix multiplication
- Backface Culling (removed if their normal is not in the right direction)
- Changed to camera view coordinate space
- Clipped from the near clipping plane
- Projected to screen space
- Normalized to coordinate system [-1,-1] -> [+1,+1]
- Computed to pixel space [0,0] -> [screen width, screen height]
- Sorted via depth buffer (In order to draw far triangles first)
- Clipped against all four screen borders
- Drawn

## Project Usage

```
$ npm init wasm-app
$ wasm-build pack [--debug|--release]
$ cd www && npm run start
```

## License

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
