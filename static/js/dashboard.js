$(document).foundation()
import * as utils from './utils.js'
import { logout } from './globals.js'


let msgs = []

const save_password = () => {
    console.log("SAVING PASSWORD (not really but soon I'll write that part)")
}

const save_names = async () => {
    console.log("SAVING NAMES (not really but soon I'll write that part)")

    msgs = []

    // get the elements where the names are stored
    const first_name_element = document.getElementById("first_name")
    const last_name_element = document.getElementById("last_name")

    // get values from input elements
    const names = {
        first_name: first_name_element.value.trim(),
        last_name: last_name_element.value.trim()
    }

    // check inputs

    let both_names_legit = utils.check_real_name(names.first_name, msgs)
    both_names_legit = utils.check_real_name(names.last_name, msgs)

    // if any checks failed, show the error and return
    if (!both_names_legit) {
        console.log(msgs.length);
        show_msg_box()
        return
    } else { hide_msg_box() }   

    // checks passed. send names to update_names route
    const route = "/auth/update_names"


    await fetch(route, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json; charset=utf-8'
        },
        body: JSON.stringify(names)
    }).then(response => {
        if (!response.ok) {
            response.json().then(data => {

                if (!!data.code) {
                    if (data.code == 422){
                        // If inputs were unacceptable, backend informs us, we show the message.
                        !data.names_valid && msgs.push(utils.username_reqs_msg)
                    } else {
                        msgs.push("Error occurred.")
                    }
                } else { msgs.push("Error.") }

                show_msg_box()
            })

            throw new Error("Inputs invalid or server errorr.")
        }
        return response.json()
    }).then(update_data => {
        if (!!update_data.success) {
            msgs.push("Names updated.")
        } else {
            msgs.push("Update failed.")
        }
        show_msg_box()
    }).catch(error => {
        console.log('Error: ', error)
    })

}

// SHOW/HIDE ERROR BOX

const hide_msg_box = () =>
    document.getElementById("msg_box").style.display = "none"

const show_msg_box = () => {
    const msg_box = document.getElementById("msg_box")
    msg_box.innerHTML = "";

    for (let msg of msgs) {
        const msg_p = "<p>" + msg + "</p>"
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