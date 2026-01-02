use common::{
    pb::chat::{ListMessagesRequest, SendMessageRequest, chat_client::ChatClient},
    prelude::Result,
};
use tonic::transport::Channel;
use tracing::info;

pub async fn send_message(client: Channel, message: String) -> Result<()> {
    let mut client = ChatClient::new(client);
    let message_request = SendMessageRequest {
        content: message,
        room_id: String::new(),
        reply_to_message_id: String::new(),
        attachments: vec![],
    };
    let response = client.send_message(message_request).await?;
    let message_id = response.into_inner().message_id;
    info!("Message sent [{message_id}]");
    list_messages(client, String::new()).await?;
    Ok(())
}

pub async fn list_messages(mut client: ChatClient<Channel>, room_id: String) -> Result<()> {
    let request = ListMessagesRequest {
        room_id,
        limit: 100,
    };
    let response = client.list_messages(request).await?;
    let messages = response.into_inner().messages;
    for message in messages {
        info!("Message: {:?}", message);
    }
    Ok(())
}
