import {Universe, Cell} from "../pkg";
import { memory } from "../pkg/game_of_life_bg";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const canvas = document.getElementById("game-of-life-canvas")
const universe = Universe.new();
const width = universe.width();
const height = universe.height();
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext('2d');
const counter = document.getElementById("counter");
let count = 0, startTime = Date.now(), nowTime, elapsedTime = 500;


function drawGrid() {
    ctx.beginPath();
    ctx.strokeSyle = GRID_COLOR;

    // Vertical lines
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

function drawCells(params) {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let index = 0; index < width * height; index++) {
        let row = index / height;
        let col = index % height;
        ctx.fillStyle = cells[index] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
        ctx.fillRect(
            col * (CELL_SIZE + 1) + 1,
            row * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
        )
    }

    ctx.stroke();
}

/**
 * requestAnimationFrame is a native function that will call
 * a callback function on every frame. We write renderLoop as 
 * the callback to render the game.
 */
function renderLoop(params) {
    // canvas.textContent = universe.render();

    nowTime = Date.now();
    drawGrid();
    drawCells();
    counter.textContent = count++;
    if (nowTime - startTime > elapsedTime) {
        startTime = nowTime;
        universe.tick();
    }
    requestAnimationFrame(renderLoop);
}

// to initiate the loop, make the first call
drawGrid();
drawCells();
requestAnimationFrame(renderLoop);