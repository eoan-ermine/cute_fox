use cute_fox::{
    requests::api_manager::{ApiManager, API_TIMEOUT_MS, API_VERSION},
    stages::users::UserInteraction,
};
use rusqlite::Connection;

const START: i32 = 0;
const STOP: i32 = 652_860_000;
const FIELDS: &str = "verified, sex, bdate, city, country, home_town, has_photo, photo_max_orig, domain, has_mobile, contacts, site, education, universities, schools, status, last_seen, followers_count, occupation, nickname, relatives, relation, personal, connections, activities, interests, music, movies, tv, books, games, about, quotes, timezone, screen_name, maiden_name, career, military";

#[tokio::main]
async fn main() {
    let mut args = std::env::args();

    let _ = args.next().unwrap();
    let db_path = args.next().expect("Please, specify argument: DB_PATH");
    let access_token = args.next().expect("Please, specify argument: ACCESS_TOKEN");

    let api = ApiManager::new(access_token, API_VERSION);

    let mut connection = Connection::open(&db_path).expect("Failed to open database");

    for i in START..=(STOP - START) / 1000 {
        let ids = ((i * 1000)..((i + 1) * 1000)).collect::<Vec<i32>>();
        let users = api.get_users(&ids, FIELDS).await;

        if let Ok(users) = users {
            let tx = connection.transaction().unwrap();
            for user in users {
                user.store(&tx, "objects").unwrap();
            }
            tx.commit().unwrap();
            println!("Saved users from {} to {}", i * 100, (i + 1) * 100);
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(API_TIMEOUT_MS)).await;
    }
}
