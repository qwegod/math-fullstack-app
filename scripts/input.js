import { calculate } from './bot.js';

window.send_input = function() {
    let a = document.getElementById("a");
    let b = document.getElementById("b");
    let c = document.getElementById("c");
    let element_bot = document.getElementById("bot_text");
    let element_user = document.getElementById("user_text");
    element_user.textContent = "Question: ";
    element_bot.textContent = "Result: ";
    calculate(a.value, b.value, c.value);
}
