import * as rust from "rust";
let last_x = null;
let last_y = null;
let last_time = new Date();
let wheel_last_time = new Date();
let events = []
let SCREEN_X_SIZE = 0;
let SCREEN_Y_SIZE = 0;
function canvas_click() {
    console.log("clicked??")
    //document.getElementById("canvas").requestPointerLock();
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
    mouse_event.set("name", "mouse_move");
    mouse_event.set("delta_x", event.clientX - last_x);
    mouse_event.set("x", event.clientX);
    mouse_event.set("delta_y", event.clientY - last_y);
    mouse_event.set("y", event.clientY);

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
function on_mouse_down(event) {
    let mouse_event = new Map();
    mouse_event.set("name", "mousedown");
    mouse_event.set("buttons", event.buttons);
    mouse_event.set("x", event.clientX);
    mouse_event.set("y", event.clientY);
    console.info("mouse down");
    events.push(mouse_event);
}
function on_mouse_up(event) {
    let mouse_event = new Map();
    console.info("mouse up");
    mouse_event.set("name", "mouseup");
    mouse_event.set("buttons", event.buttons);
    mouse_event.set("x", event.clientX);
    mouse_event.set("y", event.clientY);

    events.push(mouse_event);
}
function resize(event) {
    console.log("resized")
    console.log(event);
}
document.getElementById("canvas").onclick = canvas_click;
document.getElementById("canvas").onmousemove = mouse_move
document.getElementById("canvas").onwheel = onwheel;
document.getElementById("canvas").onresize = resize;
document.onkeypress = press_putton;
document.getElementById("canvas").onmousedown = on_mouse_down;
document.getElementById("canvas").onmouseup = on_mouse_up;
SCREEN_X_SIZE = window.innerWidth;
SCREEN_Y_SIZE = window.innerHeight;
document.getElementById("canvas").width = SCREEN_X_SIZE;
document.getElementById("canvas").height = SCREEN_Y_SIZE;
console.log("loading game")
let resolution_map = new Map();
resolution_map.set("x", SCREEN_X_SIZE);
resolution_map.set("y", SCREEN_Y_SIZE);

let game = rust.init_game(resolution_map);
console.log(window);
console.log("loaded game")
function render() {

    let new_x_size = window.innerWidth;
    let new_y_size = window.innerHeight;
    if (new_x_size != SCREEN_X_SIZE || new_y_size != SCREEN_Y_SIZE) {
        console.log("screen updated");
        console.log(new_x_size);
        console.log(SCREEN_X_SIZE);
        console.log(new_y_size);
        console.log(SCREEN_Y_SIZE);
        let screen_update_map = new Map();
        screen_update_map.set("name", "screen_size_change");
        screen_update_map.set("x", new_x_size);
        screen_update_map.set("y", new_y_size);
        SCREEN_X_SIZE = new_x_size;
        SCREEN_Y_SIZE = new_y_size;
        document.getElementById("canvas").width = SCREEN_X_SIZE;
        document.getElementById("canvas").height = SCREEN_Y_SIZE;
        events.push(screen_update_map);

    }

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
