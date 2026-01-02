pub mod prelude;
pub use prelude::{Error, Result};

pub mod pb {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    pub mod hello_world {
        tonic::include_proto!("hello_world");
    }
    pub mod route_guide {
        tonic::include_proto!("route_guide");
    }
    pub mod chat {
        use tonic::Request;
        tonic::include_proto!("chat");

        impl From<Request<SendMessageRequest>> for Message {
            fn from(request: Request<SendMessageRequest>) -> Self {
                let sender_id = request
                    .remote_addr()
                    .map(|addr| addr.to_string())
                    .unwrap_or_default();
                let inner = request.into_inner();
                Message {
                    id: uuid::Uuid::now_v7().to_string(),
                    room_id: inner.room_id,
                    sender_id,
                    content: inner.content,
                    reply_to_message_id: inner.reply_to_message_id,
                    attachments: inner.attachments,
                    edited: false,
                    deleted: false,
                }
            }
        }
    }
}
