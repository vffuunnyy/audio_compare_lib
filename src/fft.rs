use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

pub fn compute_fft(signal: &[f32]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(signal.len());

    let mut buffer: Vec<Complex<f32>> =
        signal.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();
    fft.process(&mut buffer);

    buffer.iter().map(|c| c.norm()).collect()
}
