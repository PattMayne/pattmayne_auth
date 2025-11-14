export const logout = async () => {

    // send it to the login route
    const route = "/auth/logout"

    await fetch(route, {
        method: 'POST',
        credentials: 'include'
    }).then(response => {
        if(!response.ok) {
            response.json().then(data => {
                err_msgs.push(!!data.error ? data.error : "Error")
                show_err_box()
            })

            throw new Error("Unable to logout.")
        }
        return response.json()
    }).then(logout_data => {
        console.log("Logout data: ", logout_data)
        if(!!logout_data.logout){
            window.location.href = "/";
        }
        
    }).catch(error => {
        console.log('Error: ', error)
    })
}


document.addEventListener('DOMContentLoaded', () => {
  const button = document.getElementById('logout_nav_button')
  // Checking for the button first in case use is logged in (and button doesn't exist)
  if (!!button) button.addEventListener('click', logout)
})


window.logout = logout