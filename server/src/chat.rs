use common::pb::chat::{
    AddReactionRequest, ChatEvent, CreateRoomRequest, CreateRoomResponse, DeleteMessageRequest,
    EditMessageRequest, Empty, InviteUserRequest, ListMessagesRequest, ListMessagesResponse,
    Message, RemoveReactionRequest, RemoveUserRequest, SendMessageRequest, SendMessageResponse,
    chat_event::Event, chat_server::Chat,
};
use http::request;
use std::{collections::VecDeque, pin::Pin};
use tokio::sync::{
    Mutex,
    broadcast::{self, Receiver, Sender},
};
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::{TonicResponse, middleware::add_request_log};

#[derive(Debug)]
pub struct ChatService {
    limit: usize,
    history: Mutex<VecDeque<Message>>,
    sender: Sender<ChatEvent>,
    receiver: Receiver<ChatEvent>,
}

impl ChatService {
    pub fn new(limit: usize) -> Self {
        let (sender, receiver) = broadcast::channel(limit);
        Self {
            limit,
            history: Mutex::new(VecDeque::new()),
            sender,
            receiver,
        }
    }

    async fn push_event(&self, event: ChatEvent) {
        let Some(event) = event.event else {
            return;
        };
        let mut history = self.history.lock().await;
        while history.len() >= self.limit {
            history.pop_front();
        }
        match event {
            Event::NewMessage(message) => history.push_back(message),
            Event::UpdatedMessage(_message) => todo!(),
            Event::DeletedMessageId(_deleted_id) => todo!(),
            Event::Typing(_typing_event) => todo!(),
            Event::ReadReceipt(_read_receipt) => todo!(),
        }
    }
}

#[tonic::async_trait]
impl Chat for ChatService {
    async fn create_room(
        &self,
        request: Request<CreateRoomRequest>,
    ) -> TonicResponse<CreateRoomResponse> {
        add_request_log(&request);
        todo!()
    }

    async fn invite_user(&self, request: Request<InviteUserRequest>) -> TonicResponse<Empty> {
        add_request_log(&request);
        todo!()
    }

    async fn remove_user(&self, request: Request<RemoveUserRequest>) -> TonicResponse<Empty> {
        add_request_log(&request);
        todo!()
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> TonicResponse<SendMessageResponse> {
        add_request_log(&request);
        let message: Message = request.into();
        println!("Received message: {:?}", message);
        let message_id = message.id.clone();
        let event = ChatEvent {
            event: Some(Event::NewMessage(message)),
        };
        self.push_event(event).await;
        Ok(Response::new(SendMessageResponse { message_id }))
    }

    async fn edit_message(&self, request: Request<EditMessageRequest>) -> TonicResponse<Empty> {
        add_request_log(&request);
        todo!()
    }

    async fn delete_message(&self, request: Request<DeleteMessageRequest>) -> TonicResponse<Empty> {
        add_request_log(&request);
        todo!()
    }

    async fn list_messages(
        &self,
        request: Request<ListMessagesRequest>,
    ) -> TonicResponse<ListMessagesResponse> {
        add_request_log(&request);
        let request = request.into_inner();
        let history = self.history.lock().await;
        let slices = history.as_slices();
        let mut messages = Vec::with_capacity(slices.0.len() + slices.1.len());
        // TODO: Filter by room_id
        messages.extend_from_slice(slices.0);
        messages.extend_from_slice(slices.1);
        messages.truncate(usize::try_from(request.limit).unwrap_or(self.limit));
        Ok(Response::new(ListMessagesResponse { messages }))
    }

    async fn add_reaction(&self, request: Request<AddReactionRequest>) -> TonicResponse<Empty> {
        add_request_log(&request);
        todo!()
    }

    async fn remove_reaction(
        &self,
        request: Request<RemoveReactionRequest>,
    ) -> TonicResponse<Empty> {
        add_request_log(&request);
        todo!()
    }

    type StreamEventsStream =
        Pin<Box<dyn Stream<Item = Result<ChatEvent, Status>> + Send + 'static>>;

    async fn stream_events(
        &self,
        request: Request<Streaming<Message>>,
    ) -> TonicResponse<Self::StreamEventsStream> {
        add_request_log(&request);
        todo!()
    }
}
