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
