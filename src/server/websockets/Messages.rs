use actix::prelude::*;

use crate::messages::base::NetworkPlayerIdentifier::NetworkPlayerIdentifier;
use crate::messages::incoming::GameCreationRequest::GameCreationRequest;
use crate::messages::incoming::PlayerRegistrationRequest::PlayerRegistrationRequest;

use crate::messages::outgoing::GamesListResponse::GamesListResponse;

// // INCOMING
// #[derive(Clone, Message)]
// #[rtype(result = "()")]
// pub struct WebsocketPlayerRegistrationRequest(pub PlayerRegistrationRequest);

// #[derive(Clone, Message)]
// #[rtype(result = "()")]
// pub struct WebsocketGameCreationRequest(pub GameCreationRequest);



// // OUTGOING
// #[derive(Clone, Message)]
// #[rtype(result = "()")]
// pub struct WebsocketPlayerRegistrationResponse(pub PlayerRegistrationRequest);

// #[derive(Clone, Message, Debug)]
// #[rtype(result = "()")]
// pub struct WebsocketGamesList(pub GamesListResponse);

// #[derive(Clone, Message, Debug)]
// #[rtype(result = "()")]
// pub struct WebsocketGameUpdate();



#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
// pub struct JsonMessage(pub String);
pub struct JsonMessage(pub String, pub Option<Recipient<JsonMessage>>);

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct InternalPlayerRegistrationMessage(pub NetworkPlayerIdentifier, pub Recipient<JsonMessage>);

#[derive(Clone, Message, Debug)]
#[rtype(result = "()")]
pub struct InternalGameCreationMessage(pub GamesListResponse);