use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

// connection simple
// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

/// connection pool
pub fn get_establish_connection(database_url: String) -> Pool<ConnectionManager<PgConnection>> {
    let manager:ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
