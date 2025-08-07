# blas-rs
The packages provides a BLAS binding.

## Configuration
The following implementations are available:

- `intel-mkl`, which is the one in [Intel MKL]

- `openblas`, which is the one in [OpenBLAS]

## Supported Platforms

| Blas       | Windows      | Linux      | macOS      |
| :------    | :------------: | :----------: | :----------: |
| `intel-mkl`  |     ✅       |            |            |
| `openblas`   |     ✅       |            |            |
| `netlib`     |              |            |            |
| `accelerate` |              |            |            |



## License
This Rust bindings library is licensed under the MIT or Apache-2.0 license, at your option.

Note: This project only provides bindings to the following libraries:
- Intel MKL (Intel® oneAPI Math Kernel Library) — proprietary license: https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html
- OpenBLAS — BSD 3-Clause license: https://github.com/xianyi/OpenBLAS/blob/develop/LICENSE
- Netlib BLAS — public domain
- Apple Accelerate — proprietary license: https://developer.apple.com/documentation/accelerate

You must comply with the licenses of these libraries when using this crate.

[intel mkl]: https://software.intel.com/en-us/mkl
[openblas]: https://github.com/OpenMathLib/OpenBLAS