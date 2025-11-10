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

    // if any checks failed, show the error and return
    if (!all_fields_legit) {
        console.log(err_msgs.length);
        show_err_box()
        return
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
    }).then(response => {
        if (!response.ok) {
            response.json().then(data => {

                // If inputs were unacceptable, backend informs us, we show the message.
                !data.username_valid && err_msgs.push(utils.username_reqs_msg)
                !data.email_valid && err_msgs.push(utils.email_reqs_msg)
                !data.password_valid && err_msgs.push(utils.password_reqs_msg)

                show_err_box()
            })

            throw new Error("Inputs invalid or server error.")
        }
        return response.json()
    }).then(user => {
        // THIS WILL BE AUTH DATA NOT USER (change "user" to "auth_data")
        console.log("User data: ", user)
        // do something with the user
    }).catch(error => {
        console.log('Error: ', error)
    })

    // CATCH the errors and display
    // 

    //console.log('status:', response.status)
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


document.addEventListener('DOMContentLoaded', () => hide_err_box())
document.getElementById('username').addEventListener('keydown', (e) => (e.key === 'Enter') && submit_register())
document.getElementById('email').addEventListener('keydown', (e) => (e.key === 'Enter') && submit_register())
document.getElementById('password').addEventListener('keydown', (e) => (e.key === 'Enter') && submit_register())

window.submit_register = submit_register
