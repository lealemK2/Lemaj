use crate::database::db::Db;
use crate::models::account::Account;
use deadpool_postgres::Client;
use deadpool_postgres::PoolError;

pub async fn create_account(db: &Db, account: Account) -> Result<Account, PoolError> {
    let client: Client = db.get().await?;
    let stmt = client
        .prepare("INSERT INTO accounts(firstname, lastname) VALUES($1, $2) RETURNING *")
        .await?;
    let rows = client
        .query(&stmt, &[&account.firstname, &account.lastname])
        .await?;
    let row = rows.first().unwrap();
    let created_account = Account {
        id: Some(row.get(0)),
        firstname: row.get(1),
        lastname: row.get(2),
    };
    Ok(created_account)
}

pub async fn get_account(db: &Db, id: i32) -> Result<Account, PoolError> {
    let client: Client = db.get().await?;
    let stmt = client
        .prepare("SELECT id, firstname, lastname from accounts WHERE id = $1")
        .await?;
    let rows = client.query(&stmt, &[&id]).await?;
    let row = rows.first().unwrap();
    let created_account = Account {
        id: Some(row.get(0)),
        firstname: row.get(1),
        lastname: row.get(2),
    };
    Ok(created_account)
}

pub async fn get_accounts(
    db: &Db,
    page_number: i64,
    mut page_size: i64,
) -> Result<Vec<Account>, PoolError> {
    let max_page_size: &str = dotenv!("MAX_PAGE_SIZE");
    let max_page_size: i64 = max_page_size.parse().unwrap_or(0);
    if max_page_size != 0 && page_size > max_page_size {
        page_size = max_page_size
    }
    let offset = (page_number - 1) * page_size;
    let page_size = page_size;
    let client: Client = db.get().await?;
    let stmt = client
        .prepare("SELECT id, firstname, lastname from accounts ORDER BY id OFFSET $1 LIMIT $2")
        .await?;
    let rows = client.query(&stmt, &[&offset, &page_size]).await?;
    Ok(rows
        .into_iter()
        .map(|row| Account {
            id: Some(row.get(0)),
            firstname: row.get(1),
            lastname: row.get(2),
        })
        .collect())
}

pub async fn update_account(db: &Db, account: Account, id: i32) -> Result<(), PoolError> {
    let client: Client = db.get().await?;
    let stmt = client
        .prepare("UPDATE accounts SET firstname = $1, lastname = $2 WHERE id = $3")
        .await?;
    client.query(&stmt, &[&account.firstname, &account.lastname, &id]).await?;
    Ok(())
}

pub async fn delete_account(db: &Db, id: i32) -> Result<(), PoolError> {
    let client: Client = db.get().await?;
    let stmt = client
        .prepare("DELETE FROM accounts WHERE id = $1")
        .await?;
    client.query(&stmt, &[&id]).await?;
    Ok(())
}
