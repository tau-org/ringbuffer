use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RingBuffer {
  buffer: Vec<f32>,
  capacity: usize,
  r_ptr: usize,
  w_ptr: usize,
  full: bool,
  overwrite: bool
}

#[wasm_bindgen]
impl RingBuffer {
  #[wasm_bindgen(constructor)]
  pub fn new(capacity: usize) -> Self {
    Self {
      buffer: vec![0.0; capacity],
      capacity,
      r_ptr: 0,
      w_ptr: 0,
      full: false,
      overwrite: false
    }
  }

  /// Pushes a Vec<f32> onto the `RingBuffer` returns false if the buffer is 
  /// ''full´´ i.e the write index catches up to the read index.
  /// If `overwrite` is `true`, it will not set `self.full`, but make room for 
  /// new samples by incrementing self.r_ptr
  pub fn push_block(&mut self, block: Vec<f32>) -> bool {
    for n in block {
      if self.full { return false }
      self.buffer[self.w_ptr % self.capacity] = n;
      self.w_ptr+=1;
      if self.w_ptr == self.r_ptr { 
        if self.overwrite {
          self.r_ptr = (self.r_ptr + 1) % self.capacity;
        } else {
          self.full = true 
        }
      }
    }
    true
  }

  /// Pushes a single value onto the `RingBuffer` returns false if the buffer is
  /// ''full´´ i.e the write index catches up to the read index
  /// If `overwrite` is `true`, it will not set `self.full`, but make room for 
  /// new samples by incrementing self.r_ptr
  pub fn push(&mut self, item: f32) -> bool {
    if self.full { return false }
    self.buffer[self.w_ptr] = item;
    self.w_ptr = (self.w_ptr + 1) % self.capacity;
    if self.w_ptr == self.r_ptr { 
      if self.overwrite {
        // make room for more by 'freeing' positions
        self.r_ptr = (self.r_ptr + 1) % self.capacity;
      } else {
        self.full = true 
      }
    }
    true
  }

  /// Retrieves the next value to be read from the buffer.
  /// Returns `undefined` if `Option::None` which happens when the read index
  /// catches up to write index
  pub fn next(&mut self) -> Option<f32> {
    if self.r_ptr == self.w_ptr { return None }
    let out = self.buffer[self.r_ptr];
    self.r_ptr = (self.r_ptr + 1) % self.capacity;
    Some(out)
  }

  /// Gets value at a certain position in the buffer
  pub fn get(&self, index: usize) -> Option<f32> {
    if index >= self.capacity { return None }
    Some(self.buffer[index])
  }
}

