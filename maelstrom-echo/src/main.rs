use maelstrom::message::Type;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let node = maelstrom::node::Node::new();

    node.handle(Type::Echo, &callable).await
}

fn callable(msg: maelstrom::Msg) -> Result<(), maelstrom::Error> {
    // let body: Result<_, serde_json::Error> = serde_json::from_slice(_msg.)
    Ok(())
}
