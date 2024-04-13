export function calculate(a, b, c) {
    let element_bot = document.getElementById("bot_text");
    let element_user = document.getElementById("user_text");


    element_bot.textContent = element_bot.textContent + square(a, b, c);
    element_user.textContent = element_user.textContent + `${a}x² + ${b}x + ${c}c = 0`;
    
    document.getElementById("a").value = "";
    document.getElementById("b").value = "";
    document.getElementById("c").value = "";
}



function square(a, b, c) {
    let d = Math.pow(parseInt(b), 2) - 4 * parseInt(a) * parseInt(c);
    let x1 = (-parseInt(b) + Math.sqrt(d)) / (2 * parseInt(a));
    let x2 = (-parseInt(b) - Math.sqrt(d)) / (2 * parseInt(a));
    return `x₁ = ${x1}, x₂ = ${x2}`;
}