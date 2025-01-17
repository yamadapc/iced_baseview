use iced_native::futures::{
    channel::mpsc,
    task::{Context, Poll},
    Sink,
};
use std::pin::Pin;

/// An event loop proxy that implements `Sink`.
#[derive(Debug)]
pub struct Proxy<Message: 'static> {
    sender: mpsc::UnboundedSender<Message>,
}

impl<Message: 'static> Clone for Proxy<Message> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<Message: 'static> Proxy<Message> {
    /// Creates a new [`Proxy`] from an `mpsc::Sender`.
    pub fn new(sender: mpsc::UnboundedSender<Message>) -> Self {
        Self { sender }
    }
}

impl<Message: 'static> Sink<Message> for Proxy<Message> {
    type Error = mpsc::SendError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(mut self: Pin<&mut Self>, message: Message) -> Result<(), Self::Error> {
        let _ = self.sender.start_send(message)?;

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
