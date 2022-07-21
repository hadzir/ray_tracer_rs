import * as Comlink from "comlink";
const size = 128;

function createCanvas() {
  const canvas = document.createElement("canvas");
  canvas.setAttribute("width", size);
  canvas.setAttribute("height", size);
  canvas.style.width = size / 2 + "px";
  canvas.style.height = size / 2 + "px";
  document.body.appendChild(canvas);
  return canvas;
}

async function spawnRenderer(color) {
  const start = 0;
  const end = size;
  const worker = new Worker("./worker.js");
  const renderer = Comlink.wrap(worker);

  const canvas = createCanvas();
  const context = canvas.getContext("2d");

  await renderer.init(size, { start: start, end: end }, color);

  let result;
  while ((result = await renderer.renderNext()) !== false) {
    context.putImageData(result.data, 0, result.y);
  }
  worker.terminate()
}

const sphere_count = 8*20;
(async () => {
  for (let i = 0; i < sphere_count; i++) {
    spawnRenderer({ r: Math.random(), g: Math.random(), b: Math.random() });
  }
})();
