use diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("Qual será o título do post?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("\nOK! Vamos escrever o título {}. Pressione {} quando tiver terminado.", title, EOF);

    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);

    println!("\nRascunho salvo {} com o id {}", title, post.post_id);
    
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";