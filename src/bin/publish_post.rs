use self::models::Post;
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;


fn main() {
    use self::schema::posts::dsl::{posts, is_published};

    let post_id = args()
        .nth(1)
        .expect("post_id is missing.")
        .parse::<i32>()
        .expect("post_is is invalid.");

    let connection = &mut establish_connection();

    let post = diesel::update(posts.find(post_id))
        .set(is_published.eq(true))
        .returning(Post::as_returning())
        .get_result(connection)
        .unwrap();

    println!("Post publicado {}", post.title);
}