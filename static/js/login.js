$(document).foundation()

const submit_login = async () => {
    const pass_element = document.getElementById("password")
    const username_element = document.getElementById("username")

    // TODO: check data format

    const creds = {
        password: pass_element.value.trim(),
        username: username_element.value.trim()
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