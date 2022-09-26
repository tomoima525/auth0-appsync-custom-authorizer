use async_graphql::{Context, Object, Result, SimpleObject};

#[derive(SimpleObject, Debug)]
struct Event {
    id: &'static str,
    name: &'static str,
}

pub struct Query;

#[Object]
impl Query {
    async fn event_get(&self, _ctx: &Context<'_>) -> Result<Event> {
        println!("====== called");
        let event = Event {
            id: "testid".as_ref(),
            name: "event name".as_ref(),
        };
        Ok(event)
    }
}
