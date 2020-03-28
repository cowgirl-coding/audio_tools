// A circular buffer is a wrapper around vec that only supports writing into the
// buffer at the next index (starting over at index 0 if the write index would
// be out of bounds. This data structure is very useful in audio DSP for
// creating delay and filter effects.
struct CircularBuffer {
    buffer: Vec<f32>,
    write_index: usize,
}

impl CircularBuffer {
    pub fn new(buffer_size: usize) -> CircularBuffer {
        CircularBuffer {
            buffer: vec![0.0; buffer_size],
            write_index: 0,
        }
    }
    pub fn write(&mut self, value: f32) {
        self.buffer[self.write_index] = value;
        self.write_index += 1;
        if self.write_index == self.buffer.len() {
            self.write_index = 0;
        }
    }
    // Callers read from the circular buffer at a specified distance from the
    // write index, i.e., samples that were inserted N write operations ago,
    // where N is length_samples. Conversion from units such as seconds to
    // samples, or interpolation between multiple read values are higher-level
    // concerns, handled by callers.
    pub fn read(&self, length_samples: usize) -> f32 {
        if length_samples > self.buffer.len() {
            panic!("Requested delay length is greater than buffer size!");
        }
        // usize::min_value() == 0, so we can't subtract two of them and think
        // about whether the result is negative. We convert our usizes to i32
        // here to handle this.
        let mut read_index = self.write_index as i32 - length_samples as i32;
        if read_index < 0 {
            read_index += self.buffer.len() as i32;
        }

        self.buffer[read_index as usize]
    }
}

fn seconds_to_samples(seconds: f32, sample_rate: u32) -> f32 {
    sample_rate as f32 * seconds
}

// SimpleDelay manages its own buffer
pub struct SimpleDelay {
    buffer: CircularBuffer,
}

// Parameters are provided as inputs to the delay. In general this provides
// room to modulate or calculate parameters per tick as needed, without needing
// accessors. Callers can provide parameter management structs if needed.
impl SimpleDelay {
    pub fn new(buffer_size: usize) -> SimpleDelay {
        SimpleDelay {
            buffer: CircularBuffer::new(buffer_size),
        }
    }
    pub fn tick(&mut self, input_sample: f32, delay_samples: f32, feedback_amount: f32) -> f32 {
        let output = self.buffer.read(delay_samples as usize);
        self.buffer.write(input_sample + (output * feedback_amount));
        output
    }
}

pub struct DelayTap(pub f32, pub f32);