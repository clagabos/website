let illustration_wrapper = document.querySelector(".illustration-wrapper");
let main_element = document.querySelector("main");
window.addEventListener("resize", illustration_wrapper_resize);
illustration_wrapper_resize();
function illustration_wrapper_resize() {
    illustration_wrapper.style.height = main_element.offsetHeight + "px";
    illustration_wrapper.style.width = main_element.offsetWidth + "px";
}


if (document.getElementById("time_in_uk")) {
    let elem = document.getElementById("time_in_uk");
    
    let update_time = () => { elem.innerText = new Date().toLocaleTimeString("en-GB", { timeZone: "Europe/London" }); };
    update_time();
    setInterval(update_time, 100);

    elem.parentElement.style.display = "block";
}