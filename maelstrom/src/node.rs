use crate::{error::Error, message};

#[derive(Debug, Clone, Default)]
pub struct Node;

impl Node {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        msg_type: message::Type,
        _callable: &dyn FnOnce(message::Msg) -> Result<(), Error>,
    ) -> anyhow::Result<()> {
        match msg_type {
            message::Type::Echo => todo!(),
        }
    }
}
