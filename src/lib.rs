use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub mod models;
pub mod schema;
use self::models::{Post, NewPost};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("A variável de ambiente DATABASE_URL precisa estar configurada.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Erro ao se conectar a {}", database_url))
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost {title, body};

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Erro ao salvar post.")
}