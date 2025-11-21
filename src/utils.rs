/* 
 * ===============================
 * ===============================
 * =====                     =====
 * =====  UTILS AND HELPERS  =====
 * =====                     =====
 * ===============================
 * ===============================
 */



 use regex::Regex;



/* 
 * ===============================
 * ===============================
 * =====                     =====
 * =====  INPUT VALIDATIONS  =====
 * =====                     =====
 * ===============================
 * ===============================
 */

struct StringRange {
    min: usize,
    max: usize,
}

fn username_length_range() -> StringRange {
    StringRange{ min: 6, max: 16 }
}

fn password_length_range() -> StringRange {
    StringRange{ min: 6, max: 16 }
}

fn real_name_length_range() -> StringRange {
    StringRange { min: 2, max: 50 }
}

fn string_length_valid(range_obj: &StringRange, string: &String) -> bool {
    let string_length: usize = string.len();
    string_length >= range_obj.min && string_length <= range_obj.max
}

pub fn validate_username(username: &String) -> bool {
    let reg: Regex = Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
    reg.is_match(&username) &&
        string_length_valid(
            &username_length_range(),
            &username)
}

pub fn validate_password(password: &String) -> bool {
    let reg: Regex = Regex::new(r"[A-Za-z0-9!@#$%^&*()_\-+=\[\]{}:;<>.,?~`|]+$").unwrap();
    reg.is_match(&password) &&
        string_length_valid(
            &password_length_range(),
            &password)
}

pub fn validate_email(email: &String) -> bool {
    let reg: Regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    reg.is_match(&email)
}

pub fn validate_real_name(name: &String) -> bool {
    string_length_valid(&real_name_length_range(), name)
}

// TO DO: Move this into RESOURCES file
pub fn auth_client_id() -> String { String::from("auth_site") }


/* 
 * 
 * 
 * 
 * 
 * =============================
 * =============================
 * =====                   =====
 * =====  ERROR CODE MAPS  =====
 * =====                   =====
 * =============================
 * =============================
 * 
 * 
 * 
 * 
 */

pub struct ErrorData {
    pub code: String,
    pub title: String,
    pub message: String,
}

impl ErrorData {
    pub fn new(code: u16) -> Self {
        match code {
            400 => ErrorData {
                code: code.to_string(),
                title: "Bad Request".to_string(),
                message: "The request was malformed or otherwise bad.".to_string(),
            },
            401 => ErrorData {
                code: code.to_string(),
                title: "Unauthorized".to_string(),
                message: "User is not authenticated.".to_string(),
            },
            403 => ErrorData {
                code: code.to_string(),
                title: "Forbidden".to_string(),
                message: "You do not have permission to view this page.".to_string(),
            },
            404 => ErrorData {
                code: code.to_string(),
                title: "Not Found".to_string(),
                message: "The page you are looking for was not found.".to_string(),
            },
            408 => ErrorData {
                code: code.to_string(),
                title: "Request Timeout".to_string(),
                message: "Server is shutting down connection.".to_string(),
            },
            409 => ErrorData {
                code: code.to_string(),
                title: "Conflict".to_string(),
                message: "Unacceptable duplicate input.".to_string(),
            },
            422 => ErrorData {
                code: code.to_string(),
                title: "Unprocessable Content".to_string(),
                message: "Request was well formed but content contains semantic errors.".to_string(),
            },
            429 => ErrorData {
                code: code.to_string(),
                title: "Too Many Requests".to_string(),
                message: "User has sent too many requests.".to_string(),
            },
            500 => ErrorData {
                code: code.to_string(),
                title: "Internal Server Error".to_string(),
                message: "An unexpected error occurred.".to_string(),
            },
            502 => ErrorData {
                code: code.to_string(),
                title: "Bad Gateway".to_string(),
                message: "Gateway server received an invalid response.".to_string(),
            },
            503 => ErrorData {
                code: code.to_string(),
                title: "Service Unavailable".to_string(),
                message: "Server is not ready to handle the request. Please check back later.".to_string(),
            },
            504 => ErrorData {
                code: code.to_string(),
                title: "Gateway Timeout".to_string(),
                message: "Server did not respond in time.".to_string(),
            },
            _ => ErrorData {
                code: code.to_string(),
                title: "Unknown Error".to_string(),
                message: "An unknown error has occurred.".to_string(),
            },
        }
    }
}