import init, { Simulation } from "./pkg/particles.js";

await init();
const sim = new Simulation(800, 600, 500);
const ctx = document.querySelector("canvas").getContext("2d");

function frame() {
    sim.tick();
    const points = sim.positions();
    ctx.clearRect(0, 0, 800, 600);
    for (let i = 0; i < points.length; i += 2) {
        ctx.fillRect(points[i], points[i + 1], 2, 2);
    }
    requestAnimationFrame(frame);
}
frame();