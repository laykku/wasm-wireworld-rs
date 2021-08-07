import { World, Cell, print_smth } from "wasm-wireworld-rs";
import { memory } from "wasm-wireworld-rs/wasm_wireworld_rs_bg";

const CELL_SIZE = 10;
const GRID_COLOR = "#D8E3E7";
const EMPTY_COLOR = "#FFFFFF";
const ELECTRONHEAD_COLOR = "#FFA900";
const ELECTRONTAIL_COLOR = "#CD113B";
const CONDUCTOR_COLOR = "#52006A";

const world = World.new(64, 64);
const width = world.width();
const height = world.height();

const canvas = document.getElementById("wireworld-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let row = 0;
let col = 0;
let prev_row = -1;
let prev_col = -1;
let draw = false;


const get_index = (row, column) => {
    return row * width + column;
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i < width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for (let j = 0; j < height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1)
    }

    ctx.stroke();
}

const drawCells = () => {
    const cellsPtr = world.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = get_index(row, col);

            switch (cells[idx]) {
                case Cell.Empty:
                    ctx.fillStyle = EMPTY_COLOR;
                    break;
                case Cell.ElectronHead:
                    ctx.fillStyle = ELECTRONHEAD_COLOR;
                    break;
                case Cell.ElectronTail:
                    ctx.fillStyle = ELECTRONTAIL_COLOR;
                    break;
                case Cell.Conductor:
                    ctx.fillStyle = CONDUCTOR_COLOR;
                    break;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
}

canvas.onmousemove = (e) => {
    let cRect = canvas.getBoundingClientRect();
    let canvasX = Math.round(e.clientX - cRect.left);
    let canvasY = Math.round(e.clientY - cRect.top);
    row = Math.floor(canvasY / (CELL_SIZE + 1));
    col = Math.floor(canvasX / (CELL_SIZE + 1));

    if (draw && (row != prev_row || col != prev_col)) {
        prev_row = row;
        prev_col = col;
        world.toggle_cell(row, col);
    }
};

canvas.onmousedown = (e) => {
    draw = true;
    console.log(row, col);
    if (e.button == 0) {
        prev_row = row;
        prev_col = col;
        world.toggle_cell(row, col);
        draw = true;
    } else if (e.button == 2) {
        world.set_electronhead(row, col);
    }
};

canvas.onmouseup = (e) => {
    draw = false;
}

canvas.oncontextmenu = (e) => {
    e.preventDefault();
};

window.setInterval(() => {

    world.tick();

    drawGrid();
    drawCells();
}, 100)