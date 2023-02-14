use actix::prelude::*;

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::outgoing::GamesListResponse::GamesListResponse;

// IN- AND OUTGOING
#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
// pub struct JsonMessage(pub String);
pub struct JsonMessage(pub String, pub Option<Recipient<JsonMessage>>);

// INTERNAL
#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct InternalPlayerRegistrationMessage(pub NetworkPlayerIdentifier, pub Recipient<JsonMessage>);

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct InternalGameCreationMessage(pub GamesListResponse);