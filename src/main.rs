use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let mut client =
        Client::connect("postgres://administrator:pass@localhost/test", NoTls).unwrap();

    _ = client.batch_execute(
        "
            CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )
    ",
    );

    Ok(())
}
