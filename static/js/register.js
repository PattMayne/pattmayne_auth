$(document).foundation()


const submit_register = async () => {
    const pass_element = document.getElementById("password")
    const username_element = document.getElementById("username")

    // TODO: check data format

    const creds = {
        password: pass_element.value.trim(),
        username: username_element.value.trim()
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
