import init, { RingBuffer } from './pkg/wasm_ringbuffer.js'

console.log("hello");
async function main() {
  await init('./pkg/wasm_ringbuffer_bg.wasm');
  const rb = new RingBuffer(500);
  for (let i = 0; i < 200; i++) {
    rb.push(i);
    let n = rb.next_block();
    if (n)  { console.log("Some(" + n + ")");
    } else { console.log("None");
    }
  }
  console.log( rb.read_pos() + '\n' + rb.write_pos() + '\n');
  console.log("goofbye");
}

await main()
