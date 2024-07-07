use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(is_published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Erro ao carregar os posts.");

    println!("Mostrando {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("--------------\n");
        println!("{}", post.body);
    }
}