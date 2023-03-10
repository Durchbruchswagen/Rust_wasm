import * as wasm from "wasm-game-of-life";
import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
const CELL_SIZE = 1; // px
//wasm.greet();
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE) * height;
canvas.width = (CELL_SIZE) * width;

const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const redPtr = universe.red();
  const red = new Uint8Array(memory.buffer, redPtr, width * height);
  const greenPtr = universe.green();
  const green = new Uint8Array(memory.buffer, greenPtr, width * height);
  const bluePtr = universe.blue();
  const blue = new Uint8Array(memory.buffer, bluePtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      const redc=red[idx].toString();
      const greenc=green[idx].toString();
      const bluec=blue[idx].toString();
      const color="rgb("+redc+", "+greenc+", "+bluec+")";
      //console.log(color);
      ctx.fillStyle = color;

      ctx.fillRect(
        col * (CELL_SIZE),
        row * (CELL_SIZE),
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};
let real=0.0;
let im=0.0;
const renderLoop = () => {
  console.log(real);
  console.log(im);
  universe.tick(real,im);
  drawCells();
};

const drawButton = document.getElementById("draw");
drawButton.addEventListener("click", event => {
  real=document.getElementById('real').value;
  im=document.getElementById('im').value;
  console.log("Click");
  renderLoop();
});


//drawGrid();
//drawCells();
