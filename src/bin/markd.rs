use sqlx::postgres::PgPoolOptions;
use std::env;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args: Vec<String> = env::args().collect();
    let _inserter;

    match File::open(&args[2]) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            _inserter = content;
        } // as ref in the
        Err(_error) => {
            panic!("could not insert into postgres")
        }
    }
    let _pool = PgPoolOptions::new()
        .max_connections(3)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await
        .expect("couldn't create pool");

    Ok(())
}
