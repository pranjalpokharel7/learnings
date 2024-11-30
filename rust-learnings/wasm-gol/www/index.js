import { Universe, Cell } from "wasm-gol";
import { memory } from "wasm-gol/wasm_gol_bg";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";


// construct universe
const UNIVERSE_WIDTH = 60;
const UNIVERSE_HEIGHT = 60;
const universe = Universe.new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT);

// canvas
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * UNIVERSE_HEIGHT + 1;
canvas.width = (CELL_SIZE + 1) * UNIVERSE_WIDTH + 1;

const ctx = canvas.getContext('2d');

// offsets of 1 px for each grid border
const translateGridPoint = (point) => point * (CELL_SIZE + 1) + 1;

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // vertical lines
    for (let i = 0; i <= UNIVERSE_WIDTH; i++) {
        let x = translateGridPoint(i);
        let y = translateGridPoint(UNIVERSE_HEIGHT);

        ctx.moveTo(x, 0);
        ctx.lineTo(x, y);
    }

    // horizontal lines
    for (let j = 0; j <= UNIVERSE_HEIGHT; j++) {
        let x = translateGridPoint(UNIVERSE_WIDTH);
        let y = translateGridPoint(j);

        ctx.moveTo(0, y);
        ctx.lineTo(x, y);
    }

    ctx.stroke();
}

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, UNIVERSE_WIDTH * UNIVERSE_HEIGHT);

    ctx.beginPath();

    for (let row = 0; row < UNIVERSE_HEIGHT; row++) {
        for (let col = 0; col < UNIVERSE_WIDTH; col++) {
            const idx = row * UNIVERSE_WIDTH + col;

            ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
            ctx.fillRect(translateGridPoint(col), translateGridPoint(row), CELL_SIZE, CELL_SIZE);
        }
    }

    ctx.stroke();
}

let start;
let elapsed;
const FPS = 50;

const renderLoop = (timestamp) => {
    if (start === undefined) { start = timestamp };
    let end = timestamp;
    let delta = end - start;
    elapsed = elapsed === undefined ? delta : elapsed + delta;
    start = end;

    while (elapsed >= FPS) {
        universe.tick();

        drawGrid();
        drawCells();

        elapsed -= FPS;
    }

    requestAnimationFrame(renderLoop);
}

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);