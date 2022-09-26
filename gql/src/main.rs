use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use events::model::Query;
use std::{fs::File, io::prelude::Write};
fn main() -> std::io::Result<()> {
    println!("Generate Schema");
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let sdl = schema.sdl();
    let mut file = File::create("gql/events_schema.graphql")?;
    file.write_all(sdl.as_bytes())?;
    Ok(())
}
