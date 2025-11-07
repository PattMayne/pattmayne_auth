$(document).foundation()

/**
 * Functions for the user registration input page.
 */


const username_regex = /^[A-Za-z0-9_-]+$/
const password_regex = /^[A-Za-z0-9!@#$%^&*()_\-+=\[\]{}:;<>.,?~`|]+$/
const email_regex = /^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$/

const username_length_range = {
    min: 6,
    max: 16
}

const password_length_range = {
    min: 6,
    max: 16
}

const email_reqs_msg = "Must be a legitimate email address. Check your formatting."
const username_reqs_msg = "Username must be 6 to 16 characters. Only letters, numbers, underscore, and hyphen allowed."
const password_reqs_msg = "Password must be 6 to 16 characters with no spaces."

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
    let all_fields_legit = check_username(creds.username)
    all_fields_legit = check_password(creds.password) && all_fields_legit
    all_fields_legit = check_email(creds.email) && all_fields_legit

    // printing to console now.
    // But we must put this in a div instead
    for (let err_msg of err_msgs) {
        console.log(err_msg)
    }

    if (!all_fields_legit) {
        console.log(err_msgs.length);
        show_err_box()
        return
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

// Make sure password matches regex and length requirements
const check_password = (password) => {
    const password_is_legit = password_regex.test(password) &&
        string_in_range(password_length_range, password)

    if (!password_is_legit) { err_msgs.push(password_reqs_msg) }
    return password_is_legit
}



// Make sure email matches regex
const check_email = (email) => {
    const email_is_legit = email_regex.test(email)

    if (!email_is_legit) { err_msgs.push(email_reqs_msg) }
    return email_is_legit
}



// Make sure username matches regex and length requirements
const check_username = (username) => {
    const username_is_legit = username_regex.test(username) &&
        string_in_range(username_length_range, username)

    if (!username_is_legit) { err_msgs.push(username_reqs_msg) }
    return username_is_legit
}



// Make sure the input string is within length range
const string_in_range = (range_obj, string) =>
    string.length >= range_obj.min && string.length <= range_obj.max



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

