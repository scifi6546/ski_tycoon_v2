import * as rust from "rust";
let last_x = null;
let last_y = null;
let last_time = new Date();
let wheel_last_time = new Date();
let events = []
function canvas_click() {
    console.log("clicked??")
    document.getElementById("canvas").requestPointerLock();
}
function mouse_move(event) {

    let now = new Date();
    let mouse_event = new Map();
    if (last_x === null) {
        last_x = event.clientX;


    }
    if (last_y === null) {
        last_y = event.clientY;
    }
    let delta_x = Number(event.clientX - last_x);
    mouse_event.set("name", "mouse_move");
    mouse_event.set("delta_x", event.clientX - last_x);
    mouse_event.set("delta_y", event.clientY - last_y);
    mouse_event.set("delta_time_ms", Number(now - last_time))
    mouse_event.set("buttons", event.buttons);
    events.push(mouse_event)
    last_x = event.clientX;
    last_y = event.clientY;
    last_time = now;
}
function onwheel(event) {
    let now = new Date();
    let wheel_event = new Map();
    wheel_event.set("name", "wheel");
    wheel_event.set("delta_y", event.deltaY);
    let delta_time = Number(now - wheel_last_time);
    wheel_event.set("delta_time_ms", delta_time);
    events.push(wheel_event);
    last_time = now;
}
function press_putton(event) {
    console.log(event);
    let button_event = new Map();
    button_event.set("name", "keypress");
    button_event.set("key", event.key)
    events.push(button_event)
}
document.getElementById("canvas").onclick = canvas_click;
document.getElementById("canvas").onmousemove = mouse_move
document.getElementById("canvas").onwheel = onwheel;
document.onkeypress = press_putton;
let game = rust.init_game();
function render() {
    let event_state = new Map();
    if (last_x === null) {
        event_state.set("position_x", 0.0);
    } else {
        event_state.set("position_x", last_x);
    }
    if (last_y === null) {
        event_state.set("position_y", 0.0);
    } else {
        event_state.set("position_y", last_y);
    }
    game.render_frame(events);
    events = []
    requestAnimationFrame(render)
}
requestAnimationFrame(render)
