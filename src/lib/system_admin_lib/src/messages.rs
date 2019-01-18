use bincode::{serialize, deserialize};
/// system_admin is multi-protocol and converts other protocols to it's internal representation
/// not all message types will be used by all protocols, however this should broadly cover most
/// protocols, Action should be considered analogous to ACTION in the IRC protocol
#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum MessageType {
    Ping,
    Pong,
    ConnectionOpen { address : String },
    ConnectionClose { address : String },
    Join { group : String },
    Acknowledge,
    NotAcknowledge,
    CommandNotFound,
    PermissionDenied,
    UndefinedError,
    Message { message : String },
    Action { message : String },
}

/// PermissionLevel is broadly modeled off of IRC's permission level system with the names changed
/// to be more generic with an additional internal permission level for modules that should not be
/// invoked by users
#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum PermissionLevel {
    Internal,
    Owner,
    Admin,
    SubAdmin,
    User,
    World
}

/// Supported protocol types by system_admin, a module can select which protocols it supports
/// allowing fine grained control, or Any to allow for responding to any message. All received messages
/// will be tagged with the protocol they were received by according to the connection's settings
/// PlainText is a special plain text protocol primarily for development purposes
#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum ProtocolType {
    PlainText,
    SOAP,
    REST,
    IRC,
    Telegram,
    Any,
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct Message {
    pub message_type : MessageType,
    pub permission_level : PermissionLevel,
    pub protocol : ProtocolType,
    pub origin : String
}

impl Message {
    pub fn new(message_type : MessageType, permission_level: PermissionLevel,
               protocol: ProtocolType, origin : String) -> Message {
        Message { message_type, permission_level, protocol, origin}
    }
}