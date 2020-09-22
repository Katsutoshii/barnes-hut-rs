# barnes-hut-rs

Implementation of the Barnes-Hut and direct algorithms in Rust.

## Running the project

Assuming you have `cargo` installed along with the rest of the `rustup` toolchain, run the following to run the simulation locally.

```bash
cargo run
```

Note that the local OpenGL frontend is very slow compared to the [web frontend](https://github.com/Katsutoshii/barnes-hut-frontend).

## The algorithm

The direct algorithm for solving the N-body problem involves the quadratic operation of computing the force between all pairs of bodies in the simulation.
The Barnes-Hut algorithm notices that while the exact solution is expensive to compute, we can receive a decent approximation by "grouping" faraway bodies together.

This grouping is achieved by a quadtree (works for 2D, an octree is required for 3D) that maintains the center of mass for each node.
When computing the force on each body, the tree is traversed from the root, only taking into account child nodes that are within a threshold distance.

For full details on the Barnes-Hut algorithm, see the [wikipedia article](https://en.wikipedia.org/wiki/Barnes%E2%80%93Hut_simulation).

## Efficient quadtree implementation

This quadtree implementation has the following advantages compared to other existing quadtrees:

1) Maintains center of mass for each node (required for Barnes-Hut algorithm).
2) Does not explicity store a bounding box per node (this is inferred during iteration)
3) Iterative insertion of new bodies (avoids overhead of recursing, which would limit the tree size by the call stack limit).

See [src/quadtree/tree.rs](./src/quadtree/tree.rs) for the implementation.
