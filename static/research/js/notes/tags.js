filterSelection("all-btn");

function filterSelection(sel) {
    var elements;
    elements = document.getElementsByClassName("filterLi");
    buttons = document.getElementsByClassName("tag-btn");
    // Show all
    if(sel == "all-btn") {
        // Make all visible
        for(var i = 0; i < elements.length; i++) {
            elements[i].removeAttribute("hidden");
        }
        // Make all buttons outlined except for all
        for(var i = 0; i < buttons.length; i++) {
            buttons[i].classList.remove("btn-dark");
            buttons[i].classList.add("btn-outline-dark");
        }
        document.getElementById(sel).classList.remove("btn-outline-dark");
        document.getElementById(sel).classList.add("btn-dark");
    }
    // Select only one if "tag_all" is active
    else if(document.getElementById("all-btn").classList.contains("btn-dark")) {
        // Make only "sel" visible
        for(var i = 0; i < elements.length; i++) {
            elements[i].setAttribute("hidden", "");
            if(elements[i].classList.contains(sel)) {
                elements[i].removeAttribute(("hidden"));
            }
        }
        // Make only "sel" button outlined
        document.getElementById("all-btn").classList.remove("btn-dark");
        document.getElementById("all-btn").classList.add("btn-outline-dark");
        document.getElementById(sel).classList.remove("btn-outline-dark");
        document.getElementById(sel).classList.add("btn-dark");
    }
    // Select more
    else {
        // If "sel" is already selected, remove filter
        if(document.getElementById(sel).classList.contains("btn-dark")) {
            // Un-highlight button
            document.getElementById(sel).classList.remove("btn-dark");
            document.getElementById(sel).classList.add("btn-outline-dark");
            // Show everything
            for(var i = 0; i < elements.length; i++) {
                elements[i].removeAttribute("hidden");
            }
            // Hide any element with an inactive button
            for(var i = 0; i < buttons.length; i++) {
                if(buttons[i].classList.contains("btn-dark") == true) {
                    var id = buttons[i].id;
                    for(var j = 0; j < elements.length; j++) {
                        if(elements[j].classList.contains(id) == false) {
                            elements[j].setAttribute("hidden", "");
                        }
                    }
                }
            }
            // If no buttons are selected, highlight "all-btn"
            var btn_outlined = false;
            for(var i = 0; i < buttons.length; i++) {
                if(buttons[i].classList.contains("btn-dark")) {
                    btn_outlined = true;
                    break;
                }
            }
            if(btn_outlined === false) {
                document.getElementById("all-btn").classList.remove("btn-outline-dark");
                document.getElementById("all-btn").classList.add("btn-dark");
            }
        }
        // If "sel" was not selected, add filter
        else {
            // Make "sel" visible
            for(var i = 0; i < elements.length; i++) {
                if(elements[i].classList.contains(sel) == false) {
                    elements[i].setAttribute("hidden", "");
                }
            }
            // Make "sel" buttons outlined
            document.getElementById(sel).classList.remove("btn-outline-dark");
            document.getElementById(sel).classList.add("btn-dark");
        }
    }
}
