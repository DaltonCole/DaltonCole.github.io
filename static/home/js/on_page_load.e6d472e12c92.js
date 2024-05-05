// Functions to call on page load
add_rounded_class_to_pre();

// Add "rounded" class to all "pre" tags
function add_rounded_class_to_pre() {
    var pre = document.getElementsByTagName('pre');
    for(var i = 0; i < pre.length; i++) {
        pre[i].classList.add("rounded");
    }
}
