# dsp-rs

A general-purpose DSP library for Rust, targeting `no_std` environments — built with RISC-V microcontrollers and the ESP32 family in mind.

CMSIS-DSP is ARM-only. If you're running RISC-V (ESP32-C3, C6, H2, P4, or any SiFive/custom core), there is no vendor-blessed optimized DSP baseline. This crate fills that gap.

## Targets

Tested and intended for:

- **ESP32-C3 / C6 / H2** — RV32IMC, no hardware FPU, soft-float path
- **ESP32-P4** — RV32 with FPU, harder real-time workloads viable
- **ESP32 classic** — Xtensa, also works
- General `no_std` Rust embedded targets

Works with [`esp-hal`](https://github.com/esp-rs/esp-hal) and the `esp-rs` toolchain out of the box.

## Features

```toml
[dependencies]
dsp-rs = { version = "0.1", default-features = false, features = ["embedded-core"] }
```

### `embedded-core` (default, `no_std`)

The tier that runs on bare metal with no allocator:

- **FFT** — complex and real, power-of-two sizes
- **FIR / IIR** — filter execution, direct-form I/II
- **Frequency-domain filtering** (`fdfilter`)
- **Windowing** — Hann, Hamming, Blackman, flat-top, etc.
- **AGC** — automatic gain control
- **PLL** — phase-locked loop
- **Convolution**
- **Resampling**
- **Quantization**
- **Signal generators** — sine, square, sawtooth, noise
- **Modulation** — AM, FM, PM
- **Delay lines**
- **Weighting filters** — A, C, Z
- **DTMF** encode/detect

### `embedded-extended` (`no_std`, heavier compute)

For targets with more headroom (ESP32-P4, SiFive U-series):

- **STFT** — short-time Fourier transform
- **Spectral analysis**
- **Statistics**
- **Signal analysis**
- **Adaptive filtering**
- **Robust signal processing**

### `full` (requires `std`)

Desktop, Linux-embedded, or hosted environments only:

- **ICA** — independent component analysis
- **PCA** — principal component analysis
- **EMD** — empirical mode decomposition
- **GED** — generalized eigendecomposition
- **Array processing**
- **Signal I/O**
- **Coding**

## Usage

```toml
# Cargo.toml

# Bare metal RISC-V (no_std, no alloc)
[dependencies]
dsp-rs = { version = "0.1", default-features = false, features = ["embedded-core"] }

# Hosted / desktop
[dependencies]
dsp-rs = { version = "0.1", features = ["full"] }
```

Basic FIR filter on ESP32-C3:

```rust
#![no_std]
#![no_main]

use dsp_rs::signal_processing::fir::FirFilter;

// 32-tap low-pass filter, coefficients from your design tool
const COEFFS: [f32; 32] = [ /* ... */ ];

let mut filter = FirFilter::new(&COEFFS);

// In your sample loop:
let output = filter.process(input_sample);
```

## no_std Status

`embedded-core` and `embedded-extended` features are fully `no_std`. No heap allocation required for core filter and transform operations. Fixed-size buffers via const generics where applicable.

`full` pulls in `std` and is not suitable for bare-metal targets.

## Why not CMSIS-DSP via FFI?

You can call CMSIS-DSP from Rust via `cmsis-dsp-sys`. On Cortex-M4/M7, you probably should. ARM's hand-tuned SIMD is hard to beat. But CMSIS-DSP is ARM-only. On RISC-V there is no equivalent. This crate exists for that gap.

## License

MIT