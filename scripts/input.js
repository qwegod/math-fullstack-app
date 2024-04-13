import { calculate } from './bot.js';

window.send_input = function() {
    let element = document.getElementById("textbox");
    calculate(element.value, element.name);
}
