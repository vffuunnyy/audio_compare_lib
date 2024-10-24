use std::f32::consts::PI;

pub fn lowpass_filter(signal: &[f32], cutoff: usize, sample_rate: usize) -> Vec<f32> {
    let rc = 1.0 / (2.0 * PI * cutoff as f32);
    let dt = 1.0 / sample_rate as f32;
    let alpha = dt / (rc + dt);

    let mut filtered_signal = vec![0.0; signal.len()];
    filtered_signal[0] = signal[0];

    for i in 1..signal.len() {
        filtered_signal[i] = filtered_signal[i - 1] + alpha * (signal[i] - filtered_signal[i - 1]);
    }

    filtered_signal
}
