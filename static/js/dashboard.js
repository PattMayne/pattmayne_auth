$(document).foundation()
import * as utils from './utils.js'
import { logout } from './globals.js'


const save_password = () => {
    console.log("SAVING PASSWORD (not really but soon I'll write that part)")
}

const save_names = () => {
    console.log("SAVING NAMES (not really but soon I'll write that part)")
}

let msgs = []

// SHOW/HIDE ERROR BOX

const hide_msg_box = () =>
    document.getElementById("msg_box").style.display = "none"

const show_msg_box = () => {
    const msg_box = document.getElementById("msg_box")
    msg_box.innerHTML = "";

    for (let msg of msgs) {
        const msg_p = "<p>" + err_msg + "</p>"
        msg_box.innerHTML += msg_p
    }

    msg_box.style.display = ""
}


// Add event listeners

document.addEventListener('DOMContentLoaded', () => hide_msg_box())
document.getElementById('first_name').addEventListener(
    'keydown', (e) => (e.key === 'Enter') && save_names())

document.getElementById('last_name').addEventListener(
    'keydown', (e) => (e.key === 'Enter') && save_names())

document.getElementById('new_password').addEventListener(
    'keydown', (e) => (e.key === 'Enter') && save_password())

document.getElementById('new_password_confirm').addEventListener(
    'keydown', (e) => (e.key === 'Enter') && save_password())


window.save_names = save_names
window.save_password = save_password