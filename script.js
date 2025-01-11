import init, { RingBuffer } from './pkg/wasb_ringbuffer.js'

console.log("hellp");
async function main() {
  await init('./pkg/wasb_ringbuffer_bg.wasm');
  const rb = new RingBuffer(100);
  for (let i = 0; i < 200; i++) {
    rb.push(i);
    let n = rb.next();
    if (n)  { console.log("Some(" + n + ")");
    } else { console.log("None");
    }
  }
  console.log( rb.read_pos() + '\n' + rb.write_pos() + '\n' + "goofbye");
}
await main()
