import init, { add } from './wasm_link_color.js';

await init('./wasm_link_color_bg.wasm');

window.computeResult = async () => {
    const num1 = parseInt(document.getElementById("number1").value);
    const num2 = parseInt(document.getElementById("number2").value);
    const result = add(num1, num2);
    console.log(`The result of add(${num1}, ${num2}) is: ` + result);
    document.getElementById('result').innerHTML = result;
}
