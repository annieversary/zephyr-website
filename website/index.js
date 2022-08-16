import init, { generate, scrape_and_generate } from "./wasm/zephyr_website.js";

const htmlInput = document.getElementById('html-input');
const cssOutput = document.getElementById('css-output');

function regen() {
  const css = scrape_and_generate(htmlInput.value);
  cssOutput.innerHTML = css;
}

const runWasm = async () => {
  // Instantiate our wasm module
  await init("./wasm/zephyr_website_bg.wasm");

  htmlInput.onchange = regen;
  regen();
};
runWasm();
