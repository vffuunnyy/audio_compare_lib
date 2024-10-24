# Audio Compare Library

This Rust library provides functionality to compare two audio files based on various parameters such as length difference, correlation, and frequency.

# Features
* **Audio Comparison**: Compare two audio files based on length difference, correlation, and frequency.
* **Low-Pass Filtering**: Apply a low-pass filter to the audio data.
* **FFT Computation**: Compute the Fast Fourier Transform (FFT) of the audio data.
* **Optimal Shift Calculation**: Find the optimal shift between two audio signals to maximize correlation.

# Installation

```toml
[dependencies]
audio_compare_lib = { git = "https://github.com/vffuunnyy/audio_compare_lib.git", version = "0.1.0" }
```

# Usage

#### Example
```rust
use std::path::PathBuf;
use audio_compare_lib::{AudioCompareParams, compare_audio_files};

fn main() {
    let file1 = PathBuf::from("path/to/first/audio.wav");
    let file2 = PathBuf::from("path/to/second/audio.wav");

    let params = AudioCompareParams::default(); // Use default parameters or customize as needed

    let result = compare_audio_files(file1, file2, params);

    if result {
        println!("The audio files are similar.");
    } else {
        println!("The audio files are not similar.");
    }
}
```
#### API

**[AudioCompareParams]()**
> This struct holds the parameters for audio comparison.
> ```rust
> pub struct AudioCompareParams {
>    pub max_length_difference: f32,
>    pub min_correlation: f32,
>    pub lowpass_cutoff: usize,
>    pub shift_tolerance_seconds: usize,
>    pub frequency: usize,
>}

**[compare_audio_files]()**
> This function compares two audio files based on the provided parameters.
> ```rust
> pub fn compare_audio_files(file1: PathBuf, file2: PathBuf, params: AudioCompareParams) -> bool;

# License
This project is licensed under the MIT License.

# Contributing
Contributions are welcome! Please open an issue or submit a pull request.

# Acknowledgements
This library uses the following crates:

* [hound](https://docs.rs/hound/) for reading WAV files.
* [rustfft](https://docs.rs/rustfft/) for FFT computation.