use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let mut client = Client::connect("postgresql://administrator:123@localhost/test", NoTls);
    Ok(())
}
