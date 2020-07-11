use super::types::FaustDsp;

use std::time::Instant;


type FloatType = f32;


pub fn run_benchmark<F>(dsp_initializer: F, sample_rate: i32)
where
    F: Fn() -> Box<dyn FaustDsp<T=FloatType>>
{
    // Generation constants
    let buffer_size = 1024;
    let min_samples = sample_rate as usize * 60;

    for _ in 1 ..= 10 {

        let mut dsp = dsp_initializer();

        let num_inputs = dsp.get_num_inputs() as usize;
        let num_outputs = dsp.get_num_outputs() as usize;

        // Prepare buffers
        let mut in_buffer = vec![vec![0 as FloatType; buffer_size]; num_inputs];
        let mut out_buffer = vec![vec![0 as FloatType; buffer_size]; num_outputs];

        // Set input buffers to fixed impulse responses
        for c in 0..num_inputs {
            for j in 0..buffer_size {
                in_buffer[c][j] = if j == 0 { 1.0 } else { 0.0 };
            }
        }

        // Compute
        let mut sample_sum = 0.0f32;
        let mut num_samples_written = 0;

        let timer = Instant::now();
        while num_samples_written < min_samples {
            dsp.compute(
                buffer_size as i32,
                in_buffer.iter().map(|buffer| buffer.as_slice()).collect::<Vec<&[FloatType]>>().as_slice(),
                out_buffer.iter_mut().map(|buffer| buffer.as_mut_slice()).collect::<Vec<&mut [FloatType]>>().as_mut_slice(),
            );

            // handle outputs
            for c in 0..num_outputs {
                unsafe {
                    sample_sum += out_buffer.get_unchecked(c).get_unchecked(0);
                }
            }
            num_samples_written += buffer_size;
        }
        let elapsed = timer.elapsed().as_secs_f64();

        let audio_length = num_samples_written as f64 / sample_rate as f64;
        let throughput = (num_samples_written * std::mem::size_of::<FloatType>() * num_outputs) as f64 / elapsed;
        println!(
            "Rendered audio of length {:.3} sec in {:.3} sec [load: {:.3} %]    {:.3} MB/sec",
            audio_length,
            elapsed,
            100.0 * elapsed / audio_length,
            throughput / 1024.0 / 1024.0,
        );

        println!("Sample sum: {}", sample_sum);
    }
}