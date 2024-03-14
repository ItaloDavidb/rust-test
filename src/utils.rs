use regex::Regex;

pub fn email_validation(email: &str) -> bool {
    let re = Regex::new(r"^[\w-]+(\.[\w-]+)*@([\w-]+\.)+[a-zA-Z]{2,7}$").unwrap();
    re.is_match(email)
}
pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
pub const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
pub const BAD_REQUEST:&str = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";
pub const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

pub fn cpf_validation(cpf: &str) -> bool {
    let cpf = cpf.chars().filter(|&c| c.is_digit(10)).collect::<String>();
    if cpf.len() != 11 {
        return false;
    }
    if cpf.chars().all(|c| c == cpf.chars().nth(0).unwrap()) {
        return false;
    }
    let mut soma = 0;
    for (i, c) in cpf.chars().take(9).enumerate() {
        soma += c.to_digit(10).unwrap() * (10 - i as u32);
    }
    let resto = soma % 11;
    let digito_verif_1 = if resto < 2 { 0 } else { 11 - resto };

    if digito_verif_1 != cpf.chars().nth(9).unwrap().to_digit(10).unwrap() {
        return false;
    }

    let mut soma = 0;
    for (i, c) in cpf.chars().take(10).enumerate() {
        soma += c.to_digit(10).unwrap() * (11 - i as u32);
    }
    let resto = soma % 11;
    let digito_verif_2 = if resto < 2 { 0 } else { 11 - resto };

    if digito_verif_2 != cpf.chars().nth(10).unwrap().to_digit(10).unwrap() {
        return false;
    }

    true
}
