# barnes-hut-rs

Implementation of the Barnes Hut Algorithm in Rust.

## Running the local simulation

To build the MP4 from the output files:

```sh
cargo run
ffmpeg -y -r 30 -s 500x500 -i data/frames/img%04d.png -vcodec libx264 -crf 4 -pix_fmt yuv420p data/videos/sim.mp4
./data/videos/sim.mp4
```
