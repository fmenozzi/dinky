My attempt at learning Rust by porting [this project](https://github.com/fmenozzi/2d-graphics-engine) from C++. Goals for now are just to re-create all images from the original project.

I've also experimented with using triangles as primitives, rather than trying to re-write that monster function for filling generic convex polygons. This would allow non-opaque stroking and other niceties, since non-convex polygons could be triangulated without double-drawing. Results are promising so far.

Run `make` to generate PNG images in the `results/png` directory (requires ImageMagick).
