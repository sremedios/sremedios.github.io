const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "."),
    filename: "bootstrap.js",
  },
  entry: "./bootstrap_drawing.js",
  output: {
    path: path.resolve(__dirname, "."),
    filename: "bootstrap_drawing.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html']),
  ],
};
