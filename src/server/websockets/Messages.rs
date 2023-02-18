use actix::prelude::*;

use crate::messages::base::NetworkGameIdentifier::NetworkGameIdentifier;
use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::outgoing::GamesListResponse::GamesListResponse;
use crate::messages::outgoing::GameStateResponse::GameStateResponse;

// IN- AND OUTGOING
#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
// pub struct JsonMessage(pub String);
pub struct JsonMessage(pub String, pub Option<Recipient<JsonMessage>>);

// INTERNAL
#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct InternalPlayerRegistrationMessage(pub NetworkPlayerIdentifier, pub Recipient<JsonMessage>, pub GamesListResponse);

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct InternalGameListUpdateMessage(pub GamesListResponse);

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct InternalGameUpdateMessage(pub GameStateResponse, pub Vec<NetworkPlayerIdentifier>);