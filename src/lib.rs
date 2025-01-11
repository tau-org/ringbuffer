use wasm_bindgen::prelude::*;

// WebAudio AudioWorkletProcessor fixed block size
const OFFSET: usize = 128;

#[wasm_bindgen]
pub struct RingBuffer {
  buffer: Vec<f32>,
  capacity: usize,
  bitmask: usize,
  r_ptr: usize,
  w_ptr: usize,
  full: bool,
  overwrite: bool
}

#[wasm_bindgen]
impl RingBuffer {
  #[wasm_bindgen(constructor)]
  /// Creates a new `RingBuffer`.
  /// Underlying structure is a Vec<f32> with fixed size of some power-of-2
  /// ```ignore
  /// const rb = new RingBuffer(100) // creates a buffer of size 128
  /// for (let i = 0; i < 200; i++) {
  ///   rb.push(i);
  ///   let n = rb.next();
  ///   if (n)  { 
  ///     console.log("Some(" + n + ")"); 
  ///   } else { 
  ///     console.log("None"); 
  ///   } 
  /// }
  /// ```
  pub fn new(capacity: usize) -> Self {
    let mut x = 1;
    // force power-of-2-size buffer
    while x < capacity { x <<= 1; }
    Self {
      buffer: vec![0.0; x],
      capacity: x,
      bitmask: x - 1,
      r_ptr: 0,
      w_ptr: 0,
      full: false,
      overwrite: false
    }
  }

  #[inline]
  /// wrapping the read ptr
  fn wrap_read(&mut self, inc: usize) {
    self.r_ptr = (self.r_ptr + inc) & self.bitmask;
  }
  
  #[inline]
  /// wrapping the write ptr
  fn wrap_write(&mut self, inc: usize) {
    self.w_ptr = (self.w_ptr + inc) & self.bitmask;
  }

  /// gets size - debug purpose 
  pub fn size(&self) -> usize { self.capacity }
  /// gets read ptr position - debug purpose 
  pub fn read_pos(&self) -> usize {self.r_ptr}
  /// gets write ptr position - debug purpose 
  pub fn write_pos(&self) -> usize {self.w_ptr}

  /// set the options to overwrite data, which allows the read index to 
  /// increment to make space for writing new data.
  pub fn set_overwrite(&mut self, overwrite: bool) {self.overwrite = overwrite}

  /// Pushes a Vec<f32> onto the `RingBuffer` returns false if the buffer is 
  /// ''full´´ i.e the write index catches up to the read index.
  /// If `overwrite` is `true`, it will not set `self.full`, but make room for 
  /// new samples by incrementing self.r_ptr
  pub fn push_block(&mut self, block: Vec<f32>) -> bool {
    for n in block {
      if self.full { return false }
      self.buffer[self.w_ptr % self.capacity] = n;
      self.wrap_write(1);
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
    self.wrap_write(1);
    if self.w_ptr == self.r_ptr { 
      if self.overwrite {
        // make room for more by 'freeing' positions
        self.wrap_read(1);
      } else {
        self.full = true 
      }
    }
    true
  }

  /// Retrieves the next value to be read from the buffer.
  /// Returns `undefined` if `Option::None` which happens when the read index
  /// catches up to write index
  /// Increments the `read index` by `1` each function call, unless it catches up 
  /// to `write index`
  #[allow(clippy::should_implement_trait)]
  pub fn next(&mut self) -> Option<f32> {
    if self.r_ptr == self.w_ptr { return None }
    let out = self.buffer[self.r_ptr];
    self.wrap_read(1);
    Some(out)
  }

  /// Retrieves the next vector of values between the `read index` in the 
  /// buffer and an offset. The offset is hardcoded to `128`, because of the
  /// block size of the `AudioWorkletProcessor`
  pub fn next_block(&mut self) -> Option<Vec<f32>> {
    let end = self.r_ptr + OFFSET;
    if end < self.capacity {
      // if retrieved block crosses write pointer, return None
      if (self.r_ptr..end).contains(&self.w_ptr) { return None; }
      let slice = Some(self.buffer[self.r_ptr..(self.r_ptr+128)].to_vec());
      // does not need wrapping since we have checked bounds 
      self.r_ptr = end;
      slice
    } else {
      let x = self.r_ptr..self.capacity;
      let y = 0..(end%self.capacity);
      // if retrieved block crosses write pointer, return None
      if x.contains(&self.w_ptr) || y.contains(&self.w_ptr) { return None; } 
      // concatinate to mend the discontinuity
      let slice = Some([&self.buffer[x], &self.buffer[y]].concat());
      self.wrap_read(OFFSET);
      slice
    }
  }
  
  /// Gets value at a certain position in the buffer
  /// Does not modify the read index.
  pub fn get(&self, index: usize) -> Option<f32> {
    if index >= self.capacity { return None }
    Some(self.buffer[index])
  }
}

