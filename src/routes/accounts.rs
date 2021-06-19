use crate::database::accounts;
use crate::database::db::Db;
use crate::models::account::Account;
use rocket::fairing::AdHoc;
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;
use rocket::State;

type Result<T, E = rocket::response::Debug<deadpool_postgres::PoolError>> =
    std::result::Result<T, E>;

#[post("/accounts", data = "<account>")]
async fn create(db: &State<Db>, account: Json<Account>) -> Result<Created<Json<Account>>> {
    let account = account.clone();
    let new_account = accounts::create_account(db, account).await?;
    Ok(Created::new("/").body(Json(new_account)))
}

#[get("/accounts/<id>")]
async fn get(db: &State<Db>, id: i32) -> Result<Json<Account>> {
    let account = accounts::get_account(db, id).await?;
    Ok(Json(account))
}

#[get("/accounts?<page_number>&<page_size>")]
async fn get_all(
    db: &State<Db>,
    page_number: Option<i64>,
    page_size: Option<i64>,
) -> Result<Json<Vec<Account>>> {
    let page_number = match page_number {
        Some(page_number) => page_number,
        None => 1,
    };
    let page_size = match page_size {
        Some(page_size) => page_size,
        None => {
            dotenv!("MAX_PAGE_SIZE");
            let max_page_size: &str = dotenv!("MAX_PAGE_SIZE");
            max_page_size.parse().unwrap_or(0)
        }
    };
    let accounts = accounts::get_accounts(db, page_number, page_size).await?;
    Ok(Json(accounts))
}

#[patch("/accounts/<id>", data = "<account>")]
async fn update(db: &State<Db>, account: Json<Account>, id: i32) -> Result<NoContent> {
    let account = account.clone();
    accounts::update_account(db, account, id).await?;
    Ok(NoContent)
}

#[delete("/accounts/<id>")]
async fn delete(db: &State<Db>, id: i32) -> Result<NoContent> {
    accounts::delete_account(db, id).await?;
    Ok(NoContent)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Accounts Stage", |rocket| async {
        rocket.mount("/", routes![create, get, get_all, update, delete])
    })
}
