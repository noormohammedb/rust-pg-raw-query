/*
this code wrote with refering below blog
https://tms-dev-blog.com/postgresql-database-with-rust-how-to/
 */
use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let mut client =
        Client::connect("postgres://administrator:pass@localhost/test", NoTls).unwrap();

    _ = client.batch_execute(
        "
            CREATE TABLE IF NOT EXISTS app_user (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )
    ",
    );

    // let res = client
    //     .execute(
    //         "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
    //         &[&"user1", &"mypass", &"user@test.com"],
    //     )
    //     .unwrap();

    // client.execute(
    //     "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
    //     &[&"user2", &"mypass2", &"use2@gmail.com"],
    // )?;
    // client.execute(
    //     "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
    //     &[&"user3", &"anotherpass", &"mister3@test.com"],
    // )?;

    // dbg!(res);

    let client_query_res = client
        .query("SELECT id, username, password, email FROM app_user", &[])
        .unwrap();

    for row in client_query_res {
        // let id: i32 = row.get(0).unwrap().get(0);
        // let username: &str = row.get(0).unwrap().get(1);
        // let password: &str = row.get(0).unwrap().get(2);
        // let email: &str = row.get(0).unwrap().get(3);

        let id: i32 = row.get(0);
        let username: &str = row.get(1);
        let password: &str = row.get(2);
        let email: &str = row.get(3);

        dbg!(id, username, password, email);
        println!();
    }

    client.execute(
        "UPDATE app_user SET username=$1 WHERE id=$2",
        &[&"jack2", &3.to_owned()],
    )?;
    Ok(())
}
