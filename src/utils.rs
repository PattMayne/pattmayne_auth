use regex::Regex;

// INPUT VALIDATIONS

struct StringRange {
    min: usize,
    max: usize,
}

fn string_length_valid(range_obj: &StringRange, string: &String) -> bool {
    let string_length: usize = string.len();
    string_length >= range_obj.min && string_length <= range_obj.max
}

pub fn validate_username(username: &String) -> bool {
    let range: StringRange = StringRange{ min:6, max: 16 };
    let reg: Regex = Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
    reg.is_match(&username) && string_length_valid(&range, &username)
}

pub fn validate_password(password: &String) -> bool {
    let range: StringRange = StringRange{ min:6, max: 16 };
    let reg: Regex = Regex::new(r"[A-Za-z0-9!@#$%^&*()_\-+=\[\]{}:;<>.,?~`|]+$").unwrap();
    reg.is_match(&password) && string_length_valid(&range, &password)
}

pub fn validate_email(email: &String) -> bool {
    let reg: Regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    reg.is_match(&email)
}
