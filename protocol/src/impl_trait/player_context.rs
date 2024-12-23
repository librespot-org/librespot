use crate::player::Context;
use protobuf::Message;
use std::hash::{Hash, Hasher};

impl Hash for Context {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Ok(ctx) = self.write_to_bytes() {
            ctx.hash(state)
        }
    }
}

impl Eq for Context {}
