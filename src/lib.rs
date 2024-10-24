pub mod correlation;
pub mod fft;
pub mod filters;

use std::path::PathBuf;

use correlation::find_optimal_shift;
use fft::compute_fft;
use filters::lowpass_filter;
use hound;

pub struct AudioCompareParams {
    pub max_length_difference: f32,
    pub min_correlation: f32,
    pub lowpass_cutoff: usize,
    pub shift_tolerance_seconds: usize,
    pub frequency: usize,
}

impl Default for AudioCompareParams {
    fn default() -> Self {
        Self {
            max_length_difference: 2.0,
            min_correlation: 0.7,
            lowpass_cutoff: 3000,
            shift_tolerance_seconds: 2,
            frequency: 16000,
        }
    }
}

pub fn compare_audio_files(file1: PathBuf, file2: PathBuf, params: AudioCompareParams) -> bool {
    let audio1 = read_wav(file1);
    let audio2 = read_wav(file2);

    let duration1 = audio1.len() as f32 / params.frequency as f32;
    let duration2 = audio2.len() as f32 / params.frequency as f32;

    let length_difference = (duration1 - duration2).abs();
    if length_difference > params.max_length_difference {
        // println!("Audio length difference is too large: {} seconds", length_difference);
        return false;
    }

    let audio1_filtered = lowpass_filter(&audio1, params.lowpass_cutoff, params.frequency);
    let audio2_filtered = lowpass_filter(&audio2, params.lowpass_cutoff, params.frequency);

    let optimal_shift = find_optimal_shift(
        &audio1_filtered,
        &audio2_filtered,
        params.shift_tolerance_seconds,
        params.frequency,
    );

    // println!("Optimal shift (in samples): {}", optimal_shift);

    let (audio1_shifted, audio2_shifted) =
        apply_shift(&audio1_filtered, &audio2_filtered, optimal_shift);

    let fft1 = compute_fft(&audio1_shifted);
    let fft2 = compute_fft(&audio2_shifted);

    let correlation = correlation_coefficient(&fft1, &fft2);

    // println!("Correlation: {}", correlation);

    correlation >= params.min_correlation
}

fn read_wav(filename: PathBuf) -> Vec<f32> {
    let mut reader = hound::WavReader::open(filename).expect("Failed to open WAV file");
    reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect()
}

fn apply_shift(audio1: &[f32], audio2: &[f32], shift: isize) -> (Vec<f32>, Vec<f32>) {
    if shift > 0 {
        (
            audio1[shift as usize..].to_vec(),
            audio2[..audio1.len() - shift as usize].to_vec(),
        )
    } else {
        (
            audio1[..audio2.len() + shift as usize].to_vec(),
            audio2[(-shift) as usize..].to_vec(),
        )
    }
}

fn correlation_coefficient(fft1: &[f32], fft2: &[f32]) -> f32 {
    let mean1 = fft1.iter().sum::<f32>() / fft1.len() as f32;
    let mean2 = fft2.iter().sum::<f32>() / fft2.len() as f32;

    let num = fft1
        .iter()
        .zip(fft2.iter())
        .map(|(a, b)| (a - mean1) * (b - mean2))
        .sum::<f32>();
    let den1 = fft1.iter().map(|a| (a - mean1).powi(2)).sum::<f32>().sqrt();
    let den2 = fft2.iter().map(|b| (b - mean2).powi(2)).sum::<f32>().sqrt();

    num / (den1 * den2)
}
