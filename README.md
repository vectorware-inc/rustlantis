# A fork of rustlantis, for fuzzing GPUs

This is a fork of the original [`rustlantis`](https://github.com/cbeuw/rustlantis), modified to work within the `no_std` environment of a GPU. Instead of comparing different backends / optimization levels, it runs the code on a CPU and on a CUDA-capable GPU. 

The size of the generated samples has also been adjusted, to avoid overwhelming GPU drivers(this increases fuzzing speed, at the cost of a lower fuzz quality).

# Usage

Install the `nvptx64-nvidia-cuda` target, and a CUDA runtime. 
Go into the `fuzzer` directory, and run:
```
cargo run --bin fuzzer --release -- --start 0 --count 1000
```
The starting seed and the fuzz campaign size can be adjusted. The found samples will be dumped at `target/fuzz`. 