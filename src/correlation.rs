use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

pub fn find_optimal_shift(
    audio1: &[f32],
    audio2: &[f32],
    shift_tolerance_seconds: usize,
    frequency: usize,
) -> isize {
    let max_shift_samples = shift_tolerance_seconds * frequency;
    let correlation = cross_correlation_fft(audio1, audio2);

    let best_shift = correlation
        .iter()
        .enumerate()
        .take(max_shift_samples)
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .unwrap_or(0);

    best_shift as isize - (correlation.len() / 2) as isize
}

fn cross_correlation_fft(audio1: &[f32], audio2: &[f32]) -> Vec<f32> {
    let len = audio1.len().max(audio2.len());
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(len);

    let mut signal1: Vec<Complex<f32>> =
        audio1.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();
    let mut signal2: Vec<Complex<f32>> =
        audio2.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();

    signal1.resize(len, Complex { re: 0.0, im: 0.0 });
    signal2.resize(len, Complex { re: 0.0, im: 0.0 });

    fft.process(&mut signal1);
    fft.process(&mut signal2);

    let conjugate_signal1: Vec<Complex<f32>> = signal1.iter().map(|&x| x.conj()).collect();
    let cross_corr_fft: Vec<Complex<f32>> = conjugate_signal1
        .iter()
        .zip(signal2.iter())
        .map(|(a, b)| *a * *b)
        .collect();

    let fft = planner.plan_fft_inverse(len);
    let mut cross_corr_time = cross_corr_fft.clone();
    fft.process(&mut cross_corr_time);

    cross_corr_time.iter().map(|c| c.re).collect()
}
