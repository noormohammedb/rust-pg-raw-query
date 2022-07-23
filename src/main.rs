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

    let res = client
        .execute(
            "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
            &[&"user1", &"mypass", &"user@test.com"],
        )
        .unwrap();

    client.execute(
        "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
        &[&"user2", &"mypass2", &"use2@gmail.com"],
    )?;
    client.execute(
        "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
        &[&"user3", &"anotherpass", &"mister3@test.com"],
    )?;

    dbg!(res);

    Ok(())
}
