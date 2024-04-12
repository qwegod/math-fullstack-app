function input() {
    let element = document.getElementById("bot_text");
    let input = document.getElementById("textbox").value;

    element.textContent = element.textContent + input;
    document.getElementById("textbox").value = "";
}