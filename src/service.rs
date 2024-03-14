use crate::utils::*;
use postgres::Error as PostgresError;
use postgres::{Client, NoTls};

const DB_URL: &str = env!("DATABASE_URL");

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    cpf: String,
    email: String,
}
pub fn handle_post_request(request: &str) -> (String, String) {
    match (
        get_user_request_body(&request),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(user), Ok(mut client)) => {
            match handle_requisition(&user) {
                Ok(_) => {
                    client
                        .execute(
                            "INSERT INTO users (name, email, cpf) VALUES ($1, $2 ,$3)",
                            &[&user.name, &user.email, &user.cpf],
                        )
                        .unwrap();
                    println!("User created successfully");
                    (OK_RESPONSE.to_string(), "User created".to_string())
                }
                Err(err) => {
                    println!("Error handling user requisition: {}", err);
                    (BAD_REQUEST.to_string(), err)
                }
            }
        }
        _ => {
            println!("Error handling POST request");
            (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
        }
    }
}
fn handle_requisition(body: &User) -> Result<(String, String), String> {
    let user = body;
    if cpf_validation(&user.cpf) && email_validation(&user.email) {
        Ok((OK_RESPONSE.to_string(), "User Validated".to_string()))
    } else {
        Err("Invalid CPF or email".to_string())
    }
}

pub fn handle_get_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                        cpf: row.get(3),
                    };

                    println!("User retrieved successfully");
                    (
                        OK_RESPONSE.to_string(),
                        serde_json::to_string(&user).unwrap(),
                    )
                }
                _ => {
                    println!("User not found");
                    (NOT_FOUND.to_string(), "User not found".to_string())
                }
            }
        }

        _ => {
            println!("Error handling GET request");
            (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
        }
    }
}

pub fn handle_get_all_request(request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();
            for row in client.query("SELECT * FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                    cpf: row.get(3),
                });
            }
            println!("Retrieved all users successfully");
            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&users).unwrap(),
            )
        }
        _ => {
            println!("Error handling GET all request");
            (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
        }
    }
}

pub fn handle_put_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        get_user_request_body(&request),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&user.name, &user.email, &id],
                )
                .unwrap();

            println!("User updated successfully");
            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => {
            println!("Error handling PUT request");
            (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
        }
    }
}

pub fn handle_delete_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        Client::connect(DB_URL, NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client
                .execute("DELETE FROM users  WHERE id = $1", &[&id])
                .unwrap();
            if rows_affected == 0 {
                println!("User not found");
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }

            println!("User deleted successfully");
            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => {
            println!("Error handling DELETE request");
            (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string())
        }
    }
}
fn get_id(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
pub fn set_database() -> Result<(), PostgresError> {
    println!("Connecting to database...");
    let mut client = Client::connect(DB_URL, NoTls)?;

    println!("Creating users table if not exists...");
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL,
            cpf VARCHAR NOT NULL
        )",
    )?;
    println!("Database setup completed successfully");
    Ok(())
}

