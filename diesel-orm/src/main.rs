use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel_orm::schema::posts::dsl::*;
use diesel_orm::models::Post;
use diesel_orm::create_post as create_new_post;

/// Establishes a connection to the SQLite database specified in the
pub fn establish_connection() -> SqliteConnection {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Establish and return the database connection
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Function to demonstrate fetching and displaying posts
pub fn display_posts(conn: &mut SqliteConnection) {
    // Query to fetch published posts
    let results = posts
        //.filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts");

    // Display the fetched number of posts
    println!("Displaying {} posts", results.len());

    // Iterate and print each post's title and body
    for post in results {
        println!("-----------");
        println!("ID: {}", post.id);
        println!("Title: {}", post.title);
        println!("Body: {}", post.body);
        println!("-----------");
    }
}

// Function to create a new post
pub fn add_post(connection: &mut SqliteConnection, stitle: &str, sbody: &str) -> Post {

    let post = create_new_post(connection, stitle, sbody);

    println!("\nSaved draft {stitle} with id {}", post.id);

    return post;
}

fn main() {
    // Establish the database connection
    let mut _connection = establish_connection();

    println!("Database connection established.");  

    // Create a new post
    let stitle = "Sample Post Title";
    let sbody = "This is the body of the sample post.";
    let _post = add_post(&mut _connection, stitle, sbody);

    // Display posts
    display_posts(&mut _connection);

}
