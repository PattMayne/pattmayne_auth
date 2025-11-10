

export const username_regex = /^[A-Za-z0-9_-]+$/
export const password_regex = /^[A-Za-z0-9!@#$%^&*()_\-+=\[\]{}:;<>.,?~`|]+$/
export const email_regex = /^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$/

export const username_length_range = {
    min: 2,
    max: 16
}

export const password_length_range = {
    min: 2,
    max: 16
}

export const email_reqs_msg = "Must be a legitimate email address. Check your formatting."
export const username_reqs_msg = "Username must be 6 to 16 characters. Only letters, numbers, underscore, and hyphen allowed."
export const password_reqs_msg = "Password must be 6 to 16 characters with no spaces."


// Make sure password matches regex and length requirements
export const check_password = (password, err_msgs) => {
    const password_is_legit = password_regex.test(password) &&
        string_in_range(password_length_range, password)

    if (!password_is_legit) { err_msgs.push(password_reqs_msg) }
    return password_is_legit
}



// Make sure email matches regex
export const check_email = (email, err_msgs) => {
    const email_is_legit = email_regex.test(email)

    if (!email_is_legit) { err_msgs.push(email_reqs_msg) }
    return email_is_legit
}



// Make sure username matches regex and length requirements
export const check_username = (username, err_msgs) => {
    const username_is_legit = username_regex.test(username) &&
        string_in_range(username_length_range, username)

    if (!username_is_legit) { err_msgs.push(username_reqs_msg) }
    return username_is_legit
}



// Make sure the input string is within length range
const string_in_range = (range_obj, string) =>
    string.length >= range_obj.min && string.length <= range_obj.max



