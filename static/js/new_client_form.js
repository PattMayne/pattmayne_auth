$(document).foundation()
import * as utils from './utils.js'
import * as globals from './globals.js'


/**
 * Functions for the login input page
 **/

let msgs = []


const submit_data = async () => {
    msgs = []

    console.log("SUBMITTING DATA")

    msgs.push("you pressed up a button buddy")
    show_msg_box()

    /*  KEEPING THE FETCH STUFF IN COMMENTS FOR LATER ADAPTATION

    // now send it to the login route
    const route = "/auth/login"

    await utils.fetch_json_post(route, creds)
        .then(response => {
            if(!response.ok) {
                response.json().then(data => {
                    let msg = (!!data.code) ? (data.code.toString() + " ") : ""
                    msg += (!!data.error) ? data.error : " Error occurred"
                    err_msgs.push(msg)
                    show_err_box()
                })

                throw new Error("User not found or server error.")
            }
            return response.json()
        }).then(user => {
            console.log("User data: ", user)
            if(!!user.user_id){
                window.location.href = "/dashboard";
            }
            
        }).catch(error => {
            console.log('Error: ', error)
        })
            */
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


// Make functions available to the HTML elements (via window)
window.submit_data = submit_data
