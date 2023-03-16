use core::time::Duration;
use rodio::{OutputStream, source::Source};
#[derive(Clone)]
pub struct Oscillator {
    sample_rate: f32,
    wave_table: Vec<f32>,
    frequency: f32,
    index: f32,
    index_increment: f32,
}


impl Oscillator {
    pub fn new(sample_rate: f32, wave_table: Vec<f32>, frequency: f32) -> Oscillator {
        let mut osc = Oscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            frequency: 440.0,
            index: 0.0,
            index_increment: 0.0,
        };
        osc.index_increment = osc.frequency * osc.wave_table.len() as f32 / osc.sample_rate as f32;
        osc
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    pub fn make_beep(&mut self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let new_osc = self.clone();
        let _result = stream_handle.play_raw(new_osc.convert_samples());
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();
        
        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index] 
               + next_index_weight * self.wave_table[next_index];
    }
}

impl Source for Oscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate as u32;
    }   

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

impl Iterator for Oscillator {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}