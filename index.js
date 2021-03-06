const canvas = document.getElementById("lightbox-canvas")
const ctx = canvas.getContext('2d');

import init, { open_image, rev, putImageData } from "./wasm/qa.js"

async function run() {
    await init();
}

// Try to load from file selection
function readImage() {
    // break on no files
    if (!this.files || !this.files[0]) return;

    const reader = new FileReader();
    reader.addEventListener("load", (event) => {
        const img = new Image();
        img.addEventListener("load", () => {
            ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
            ctx.drawImage(img, 0, 0);
            let newimg = open_image(canvas, ctx);
            rev(newimg);
            putImageData(canvas, ctx, newimg);
        });
        img.src = event.target.result;
    });
    reader.readAsDataURL(this.files[0]);
}

const fileSelector = document.getElementById('file-selector')
fileSelector.addEventListener('change', readImage);

run();
