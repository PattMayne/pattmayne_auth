$(document).foundation()
import * as utils from './utils.js'

/**
 * Functions for the user registration input page.
 */
let err_msgs = []


/**
 * User has attempted to input registration details.
 * Gather all the user input, check if it's ok, send it to the backend.
 */
const submit_register = async () => {
    // reset error messages array with each attempt
    err_msgs = [];
    // get input elements
    const pass_element = document.getElementById("password")
    const username_element = document.getElementById("username")
    const email_element = document.getElementById("email")

    // get data (values) from input elements
    const creds = {
        password: pass_element.value.trim(),
        email: email_element.value.trim(),
        username: username_element.value.trim()
    }

    // Check the inputs
    let all_fields_legit = utils.check_username(creds.username, err_msgs)
    all_fields_legit = utils.check_password(creds.password, err_msgs) && all_fields_legit
    all_fields_legit = utils.check_email(creds.email, err_msgs) && all_fields_legit

    // printing to console now.
    // But we must put this in a div instead
    for (let err_msg of err_msgs) {
        console.log(err_msg)
    }

    if (!all_fields_legit) {
        console.log(err_msgs.length);
        show_err_box()
        //return
    } else {
        hide_err_box()
    }    

    // now send it to the register route
    const route = "/auth/register"

    const response = await fetch(route, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json; charset=utf-8'
        },
        body: JSON.stringify(creds)
    })

    console.log('status:', response.status)
}

// SHOW/HIDE ERROR BOX

const hide_err_box = () =>
    document.getElementById("err_msg_box").style.display = "none"

const show_err_box = () => {
    const err_box = document.getElementById("err_msg_box")
    err_box.innerHTML = "";

    for (let err_msg of err_msgs) {
        const msg_p = "<p>" + err_msg + "</p>"
        err_box.innerHTML += msg_p
    }

    err_box.style.display = ""
}


document.addEventListener('DOMContentLoaded', () => {
    hide_err_box();
});

window.submit_register = submit_register
