import init, { RingBuffer } from './pkg/wasb_ringbuffer.js'

console.log("hellp");
async function main() {
  await init('./pkg/wasb_ringbuffer_bg.wasm');
  // initSync();
  // await init('./pkg/wasb_ringbuffer_bg.wasm');
  const rb = new RingBuffer(5);
  rb.push(1);
  rb.push(2);
  rb.push(3);
  rb.push(4);

  for (let x = 0; x < 10; x++) {
    let n = rb.next();
    if (n !== undefined)  {
      console.log(n);
    }
  }
  console.log("goofbye");
}
await main()
