function switch_mode() {
    let element = document.getElementById("linear");

    switch (element.textContent.trim()) {
        case "Linear equation":
            document.getElementById("textbox").placeholder = "x^2 + 4x + 3 = 0";
    }                                                         
}