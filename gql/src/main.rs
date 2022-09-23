use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use starwars::{QueryRoot, StarWars};
use std::{fs::File, io::prelude::Write};
fn main() -> std::io::Result<()> {
    println!("Generate Schema");
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    let sdl = schema.sdl();
    let mut file = File::create("gql/schema.graphql")?;
    file.write_all(sdl.as_bytes())?;
    Ok(())
}
