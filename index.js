import { memory } from "/qa/qa_bg";
import { Lightbox } from "/qa";

// File stuff
const fileSelector = document.getElementById('file-selector')
fileSelector.addEventListener('change', (event) => {
    const fileList = event.target.files;
    console.log(fileList);

    var fileData = new Blob([fileList[0]]);
    var reader = new FileReader();
    reader.readAsArrayBuffer(fileData);
    reader.onload = function() {
        var arrayBuffer = reader.result
        var bytes = new Uint8Array(arrayBuffer);
        console.log(bytes);

        // Drawing stuff
        const lightbox = Lightbox.new(bytes);
        const width = lightbox.width()
        const height = lightbox.height()
        console.log("Lightbox loaded");

        const canvas = document.getElementById("lightbox-canvas")
        canvas.height = height;
        canvas.width = width;

        const ctx = canvas.getContext('2d');
        ctx.beginPath();
        ctx.rect(0, 0, width, height);
        ctx.fillStyle = "black";
        ctx.fill();
        const img_raw = lightbox.img();
        console.log("JS received from wasm")
        console.log(img_raw)
        const img_arr = Uint8ClampedArray.from(img_raw);

        console.log("width")
        console.log(width)
        console.log("height")
        console.log(height)
        console.log(img_arr.length)
        console.log("Product")
        console.log(width * height * 4)


        const img_data = new ImageData(img_arr, width, height)
        ctx.putImageData(img_data, width, height);
    }

});


