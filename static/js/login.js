$(document).foundation()

const submit_login = async () => {
    const pass_element = document.getElementById("password")
    const username_or_email_element = document.getElementById("username_or_email")

    // TODO: check data format

    const creds = {
        password: pass_element.value.trim(),
        username_or_email: username_or_email_element.value.trim()
    }

    // now send it to the login route
    const route = "/auth/login"

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

