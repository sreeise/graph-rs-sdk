pub struct BackingStore;

/*
use std::marker::PhantomData;
use actix::{Actor, Context, Handler};
use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::Message;

pub struct BackingStore {
    response: Responses
}

impl BackingStore {
    pub fn new(t: Responses) -> BackingStore {
        BackingStore {
            response: t
        }
    }
}

impl Actor for BackingStore {
    type Context = Context<Self>;
}

impl<M: Message<Result = Responses>> Handler<M> for BackingStore {
    type Result = Responses;

    fn handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::Result {
        self.response.clone()
    }
}

#[derive(Clone)]
pub enum Responses {
    AccessToken(String),
    RefreshToken(String)
}

impl<A, M> MessageResponse<A, M> for Responses
    where
        A: Actor,
        M: Message<Result = Responses>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

 */
