import { draw } from "qa";

const canvas = document.getElementById('lightbox-canvas');
const ctx = canvas.getContext('2d');

draw(ctx, 600, 600, -0.15, 0.65);
