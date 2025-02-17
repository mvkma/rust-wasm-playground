// Use ES module import syntax to import functionality from the module
// that we have compiled.
//
// Note that the `default` import is an initialization function which
// will "boot" the module and make it ready to use. Currently browsers
// don't support natively imported WebAssembly as an ES module, but
// eventually the manual initialization won't be required!
import init, { Grid } from './pkg/rwff.js';

// Some elements we will need
const ms_display = document.getElementById("ms-per-frame");
const fps_display = document.getElementById("fps");
const func_select = document.getElementById("function-select");
const param_form = document.getElementById("parameters");
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

// Parameters
canvas.height = 800;
canvas.width = 800;
const width = canvas.width;
const height = canvas.height;

const flow_params = {
nparticles: 200,
nsamples: 400,
lifetime: 1800,
func: 0,
max_lifetimes: 100,
};

function get_flow_params() {
return {
    nparticles: param_form.elements["nparticles"].valueAsNumber,
    nsamples: param_form.elements["nsamples"].valueAsNumber,
    lifetime: param_form.elements["lifetime"].valueAsNumber,
    func: func_select.selectedIndex,
    max_lifetimes: param_form.elements["max_lifetimes"].valueAsNumber,
};
};

async function run() {
// First up we need to actually load the wasm file, so we use the
// default export to inform it where the wasm file is located on the
// server, and then we wait on the returned promise to wait for the
// wasm to be loaded.
//
// It may look like this: `await init('./pkg/without_a_bundler_bg.wasm');`,
// but there is also a handy default inside `init` function, which uses
// `import.meta` to locate the wasm file relatively to js file.
//
// Note that instead of a string you can also pass in any of the
// following things:
//
// * `WebAssembly.Module`
//
// * `ArrayBuffer`
//
// * `Response`
//
// * `Promise` which returns any of the above, e.g. `fetch("./path/to/wasm")`
//
// This gives you complete control over how the module is loaded
// and compiled.
//
// Also note that the promise, when resolved, yields the wasm module's
// exports which is the same as importing the `*_bg` module in other
// modes
const wasm_exports = await init();
const memory = wasm_exports.memory;
let grid = undefined;

const set_flow_params = () => {
    let params = get_flow_params();
    console.log(params);

    grid.set_grid_lifetime(params["lifetime"]);

    grid.set_flow_params(
    params["nparticles"],
    params["nsamples"],
    params["func"],
    params["max_lifetimes"],
    );
}

func_select.addEventListener("input", (ev) => {
    set_flow_params();
});

param_form.addEventListener("submit", (ev) => {
    set_flow_params();
});

// Grid.new(width, height, nparticles, nsamples, lifetime, func, max_lifetimes)
grid = Grid.new(
    width,
    height,
    flow_params["nparticles"],
    flow_params["nsamples"],
    flow_params["lifetime"],
    0,
    flow_params["max_lifetimes"]
);

console.log(grid);

// Populate dropdown
let i = 0;
for (const f of grid.fields()) {
    const op = document.createElement("option");
    op.value = i.toString();
    op.innerText = f.name();
    func_select.appendChild(op);
    i++;
}

let diffs = Array.from({ length: 60 }, () => 0);
let start = performance.now();
let end = start;
let frame = 0;

let paused = true;

canvas.addEventListener("click", (el, ev) => {
    paused = !paused;
    requestAnimationFrame(renderLoop);
})

const renderLoop = () => {
    grid.tick();

    const pixelsPtr = grid.pixels();
    const pixels = new Uint8ClampedArray(memory.buffer, pixelsPtr, width * height * 4);
    const image = new ImageData(pixels, width, height);

    ctx.putImageData(image, 0, 0);

    end = performance.now();
    diffs[frame] = end - start;
    frame = (frame + 1) % 60;
    start = end;

    if (frame % 60 === 0) {
    let ms = diffs.reduce((a, b) => a + b, 0) / 60;
    ms_display.textContent = ms.toFixed(2);
    fps_display.textContent = (1 / ms * 1000).toFixed(2);
    }

    if (!paused) {
    requestAnimationFrame(renderLoop);
    }
}

requestAnimationFrame(renderLoop);
}

run();