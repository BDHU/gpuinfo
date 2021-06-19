# gpuinfo

A small command-line tool used to query and monitor GPU status.

![gpuinfo-screenshot](https://raw.githubusercontent.com/BDHU/gpuinfo/main/gpu-info.png)

NOTE: We only support NVIDIA GPU currently, AMD GPU is not yet supported. All contributions are welcome! This is an ongoing project and there might be changes in the future.

## Usage

```bash
$ gpu-info
```

Options:

* `-w`, `--watch`: Prints GPU information to terminal every second
* `-i`, `--interval <interval>`: Prints GPU information to terminal according to given interval (seconds)

## Installation

```bash
cargo install gpuinfo
```

## License

[MIT License](https://github.com/BDHU/gpuinfo/blob/main/LICENSE)