import { memory } from "qa/qa_bg";
import { Lightbox } from "qa";

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
        const ctx = canvas.getContext('2d');

        lightbox.render_to_canvas(ctx, width, height);
    }

});


