pub mod create_key;
pub mod invite;
pub mod validation;
pub mod get_message;
pub mod send_message;
pub mod update_profile;
pub mod proofs;
pub mod agent_utils;
pub mod update_connection;
pub mod update_message;
pub mod message_type;

use std::u8;
use settings;
use utils::libindy::crypto;
use utils::error;
use self::create_key::{CreateKeyBuilder, CreateKey, CreateKeyResponse};
use self::update_connection::{DeleteConnectionBuilder, UpdateConnection, UpdateConnectionResponse};
use self::update_profile::{UpdateProfileDataBuilder, UpdateConfigs, UpdateConfigsResponse};
use self::invite::{
    SendInviteBuilder, ConnectionRequest, SendInviteMessageDetails, SendInviteMessageDetailsResponse, ConnectionRequestResponse,
    AcceptInviteBuilder, ConnectionRequestAnswer, AcceptInviteMessageDetails, ConnectionRequestAnswerResponse
};
use self::get_message::{GetMessagesBuilder, GetMessages, GetMessagesResponse, MessagesByConnections};
use self::send_message::SendMessageBuilder;
use self::update_message::{UpdateMessageStatusByConnections, UpdateMessageStatusByConnectionsResponse};
use self::proofs::proof_request::ProofRequestMessage;
use self::agent_utils::{Connect, ConnectResponse, SignUp, SignUpResponse, CreateAgent, CreateAgentResponse, UpdateConnectionMethod};
use self::message_type::*;

use serde::{de, Deserialize, Deserializer, ser, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum A2AMessageV1 {
    /// routing
    Forward(Forward),

    /// onbording
    Connect(Connect),
    ConnectResponse(ConnectResponse),
    SignUp(SignUp),
    SignUpResponse(SignUpResponse),
    CreateAgent(CreateAgent),
    CreateAgentResponse(CreateAgentResponse),

    /// PW Connection
    CreateKey(CreateKey),
    CreateKeyResponse(CreateKeyResponse),

    CreateMessage(CreateMessage),
    MessageDetail(MessageDetail),
    MessageCreated(MessageCreated),
    MessageSent(MessageSent),

    GetMessages(GetMessages),
    GetMessagesResponse(GetMessagesResponse),
    GetMessagesByConnections(GetMessages),
    GetMessagesByConnectionsResponse(MessagesByConnections),

    UpdateConnection(UpdateConnection),
    UpdateConnectionResponse(UpdateConnectionResponse),
    UpdateMessageStatusByConnections(UpdateMessageStatusByConnections),
    UpdateMessageStatusByConnectionsResponse(UpdateMessageStatusByConnectionsResponse),

    /// Configs
    UpdateConfigs(UpdateConfigs),
    UpdateConfigsResponse(UpdateConfigsResponse),
    UpdateConnectionMethod(UpdateConnectionMethod),
}

impl<'de> Deserialize<'de> for A2AMessageV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        let message_type: MessageTypeV1 = serde_json::from_value(value["@type"].clone()).map_err(de::Error::custom)?;

        match message_type.name.as_str() {
            "FWD" => {
                Forward::deserialize(value)
                    .map(|msg| A2AMessageV1::Forward(msg))
                    .map_err(de::Error::custom)
            }
            "CONNECT" => {
                Connect::deserialize(value)
                    .map(|msg| A2AMessageV1::Connect(msg))
                    .map_err(de::Error::custom)
            }
            "CONNECTED" => {
                ConnectResponse::deserialize(value)
                    .map(|msg| A2AMessageV1::ConnectResponse(msg))
                    .map_err(de::Error::custom)
            }
            "SIGNUP" => {
                SignUp::deserialize(value)
                    .map(|msg| A2AMessageV1::SignUp(msg))
                    .map_err(de::Error::custom)
            }
            "SIGNED_UP" => {
                SignUpResponse::deserialize(value)
                    .map(|msg| A2AMessageV1::SignUpResponse(msg))
                    .map_err(de::Error::custom)
            }
            "CREATE_AGENT" => {
                CreateAgent::deserialize(value)
                    .map(|msg| A2AMessageV1::CreateAgent(msg))
                    .map_err(de::Error::custom)
            }
            "AGENT_CREATED" => {
                CreateAgentResponse::deserialize(value)
                    .map(|msg| A2AMessageV1::CreateAgentResponse(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_COM_METHOD" => {
                UpdateConnectionMethod::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateConnectionMethod(msg))
                    .map_err(de::Error::custom)
            }
            "CREATE_KEY" => {
                CreateKey::deserialize(value)
                    .map(|msg| A2AMessageV1::CreateKey(msg))
                    .map_err(de::Error::custom)
            }
            "KEY_CREATED" => {
                CreateKeyResponse::deserialize(value)
                    .map(|msg| A2AMessageV1::CreateKeyResponse(msg))
                    .map_err(de::Error::custom)
            }
            "GET_MSGS" => {
                GetMessages::deserialize(value)
                    .map(|msg| A2AMessageV1::GetMessages(msg))
                    .map_err(de::Error::custom)
            }
            "MSGS" => {
                GetMessagesResponse::deserialize(value)
                    .map(|msg| A2AMessageV1::GetMessagesResponse(msg))
                    .map_err(de::Error::custom)
            }
            "GET_MSGS_BY_CONNS" => {
                GetMessages::deserialize(value)
                    .map(|msg| A2AMessageV1::GetMessagesByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "MSGS_BY_CONNS" => {
                MessagesByConnections::deserialize(value)
                    .map(|msg| A2AMessageV1::GetMessagesByConnectionsResponse(msg))
                    .map_err(de::Error::custom)
            }
            "CREATE_MSG" => {
                CreateMessage::deserialize(value)
                    .map(|msg| A2AMessageV1::CreateMessage(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_DETAIL" => {
                MessageDetail::deserialize(value)
                    .map(|msg| A2AMessageV1::MessageDetail(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_CREATED" => {
                MessageCreated::deserialize(value)
                    .map(|msg| A2AMessageV1::MessageCreated(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_SENT" | "MSGS_SENT" => {
                MessageSent::deserialize(value)
                    .map(|msg| A2AMessageV1::MessageSent(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_CONN_STATUS" => {
                UpdateConnection::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateConnection(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_STATUS_UPDATED" => {
                UpdateConnectionResponse::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateConnectionResponse(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_MSG_STATUS_BY_CONNS" => {
                UpdateMessageStatusByConnections::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateMessageStatusByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_STATUS_UPDATED_BY_CONNS" => {
                UpdateMessageStatusByConnectionsResponse::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateMessageStatusByConnectionsResponse(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_CONFIGS" => {
                UpdateConfigs::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateConfigs(msg))
                    .map_err(de::Error::custom)
            }
            "CONFIGS_UPDATED" => {
                UpdateConfigsResponse::deserialize(value)
                    .map(|msg| A2AMessageV1::UpdateConfigsResponse(msg))
                    .map_err(de::Error::custom)
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum A2AMessageV2 {
    /// routing
    Forward(Forward),

    /// onbording
    Connect(Connect),
    ConnectResponse(ConnectResponse),
    SignUp(SignUp),
    SignUpResponse(SignUpResponse),
    CreateAgent(CreateAgent),
    CreateAgentResponse(CreateAgentResponse),

    /// PW Connection
    CreateKey(CreateKey),
    CreateKeyResponse(CreateKeyResponse),
    ConnectionRequest(ConnectionRequest),
    ConnectionRequestResponse(ConnectionRequestResponse),

    SendRemoteMessage(SendRemoteMessage),
    SendRemoteMessageResponse(SendRemoteMessageResponse),

    GetMessages(GetMessages),
    GetMessagesResponse(GetMessagesResponse),
    GetMessagesByConnections(GetMessages),
    GetMessagesByConnectionsResponse(MessagesByConnections),

    ConnectionRequestAnswer(ConnectionRequestAnswer),
    ConnectionRequestAnswerResponse(ConnectionRequestAnswerResponse),

    UpdateConnection(UpdateConnection),
    UpdateConnectionResponse(UpdateConnectionResponse),
    UpdateMessageStatusByConnections(UpdateMessageStatusByConnections),
    UpdateMessageStatusByConnectionsResponse(UpdateMessageStatusByConnectionsResponse),

    /// config
    UpdateConfigs(UpdateConfigs),
    UpdateConfigsResponse(UpdateConfigsResponse),
    UpdateConnectionMethod(UpdateConnectionMethod),
}

impl<'de> Deserialize<'de> for A2AMessageV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        let message_type: MessageTypeV2 = serde_json::from_value(value["@type"].clone()).map_err(de::Error::custom)?;

        match message_type.type_.as_str() {
            "FWD" => {
                Forward::deserialize(value)
                    .map(|msg| A2AMessageV2::Forward(msg))
                    .map_err(de::Error::custom)
            }
            "CONNECT" => {
                Connect::deserialize(value)
                    .map(|msg| A2AMessageV2::Connect(msg))
                    .map_err(de::Error::custom)
            }
            "CONNECTED" => {
                ConnectResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::ConnectResponse(msg))
                    .map_err(de::Error::custom)
            }
            "SIGNUP" => {
                SignUp::deserialize(value)
                    .map(|msg| A2AMessageV2::SignUp(msg))
                    .map_err(de::Error::custom)
            }
            "SIGNED_UP" => {
                SignUpResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::SignUpResponse(msg))
                    .map_err(de::Error::custom)
            }
            "CREATE_AGENT" => {
                CreateAgent::deserialize(value)
                    .map(|msg| A2AMessageV2::CreateAgent(msg))
                    .map_err(de::Error::custom)
            }
            "AGENT_CREATED" => {
                CreateAgentResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::CreateAgentResponse(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_COM_METHOD" => {
                UpdateConnectionMethod::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateConnectionMethod(msg))
                    .map_err(de::Error::custom)
            }
            "CREATE_KEY" => {
                CreateKey::deserialize(value)
                    .map(|msg| A2AMessageV2::CreateKey(msg))
                    .map_err(de::Error::custom)
            }
            "KEY_CREATED" => {
                CreateKeyResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::CreateKeyResponse(msg))
                    .map_err(de::Error::custom)
            }
            "GET_MSGS" => {
                GetMessages::deserialize(value)
                    .map(|msg| A2AMessageV2::GetMessages(msg))
                    .map_err(de::Error::custom)
            }
            "MSGS" => {
                GetMessagesResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::GetMessagesResponse(msg))
                    .map_err(de::Error::custom)
            }
            "GET_MSGS_BY_CONNS" => {
                GetMessages::deserialize(value)
                    .map(|msg| A2AMessageV2::GetMessagesByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "MSGS_BY_CONNS" => {
                MessagesByConnections::deserialize(value)
                    .map(|msg| A2AMessageV2::GetMessagesByConnectionsResponse(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_REQUEST" => {
                ConnectionRequest::deserialize(value)
                    .map(|msg| A2AMessageV2::ConnectionRequest(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_REQUEST_RESP" => {
                ConnectionRequestResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::ConnectionRequestResponse(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_REQUEST_ANSWER" => {
                ConnectionRequestAnswer::deserialize(value)
                    .map(|msg| A2AMessageV2::ConnectionRequestAnswer(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_REQUEST_ANSWER_RESP" => {
                ConnectionRequestAnswerResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::ConnectionRequestAnswerResponse(msg))
                    .map_err(de::Error::custom)
            }
            "SEND_REMOTE_MSG" => {
                SendRemoteMessage::deserialize(value)
                    .map(|msg| A2AMessageV2::SendRemoteMessage(msg))
                    .map_err(de::Error::custom)
            }
            "REMOTE_MSG_SENT" => {
                SendRemoteMessageResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::SendRemoteMessageResponse(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_CONN_STATUS" => {
                UpdateConnection::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateConnection(msg))
                    .map_err(de::Error::custom)
            }
            "CONN_STATUS_UPDATED" => {
                UpdateConnectionResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateConnectionResponse(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_MSG_STATUS_BY_CONNS" => {
                UpdateMessageStatusByConnections::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateMessageStatusByConnections(msg))
                    .map_err(de::Error::custom)
            }
            "MSG_STATUS_UPDATED_BY_CONNS" => {
                UpdateMessageStatusByConnectionsResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateMessageStatusByConnectionsResponse(msg))
                    .map_err(de::Error::custom)
            }
            "UPDATE_CONFIGS" => {
                UpdateConfigs::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateConfigs(msg))
                    .map_err(de::Error::custom)
            }
            "CONFIGS_UPDATED" => {
                UpdateConfigsResponse::deserialize(value)
                    .map(|msg| A2AMessageV2::UpdateConfigsResponse(msg))
                    .map_err(de::Error::custom)
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

#[derive(Debug)]
pub enum A2AMessage {
    Version1(A2AMessageV1),
    Version2(A2AMessageV2),
}

impl Serialize for A2AMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            A2AMessage::Version1(msg) => msg.serialize(serializer).map_err(ser::Error::custom),
            A2AMessage::Version2(msg) => msg.serialize(serializer).map_err(ser::Error::custom)
        }
    }
}

impl<'de> Deserialize<'de> for A2AMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        let message_type: MessageTypes = serde_json::from_value(value["@type"].clone()).map_err(de::Error::custom)?;

        match message_type {
            MessageTypes::MessageTypeV1(_) =>
                A2AMessageV1::deserialize(value)
                    .map(|msg| A2AMessage::Version1(msg))
                    .map_err(de::Error::custom),
            MessageTypes::MessageTypeV2(_) =>
                A2AMessageV2::deserialize(value)
                    .map(|msg| A2AMessage::Version2(msg))
                    .map_err(de::Error::custom),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Forward {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "@fwd")]
    fwd: String,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
}

impl Forward {
    fn new(fwd: String, msg: Vec<u8>) -> Forward {
        Forward {
            msg_type: MessageTypes::build(A2AMessageKinds::Forward),
            fwd,
            msg,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CreateMessage {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV1,
    mtype: RemoteMessageType,
    #[serde(rename = "sendMsg")]
    send_msg: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<String>,
    #[serde(rename = "replyToMsgId")]
    reply_to_msg_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralMessageDetail {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV1,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
    title: Option<String>,
    detail: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageCreated {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV1,
    pub uid: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageSent {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default)]
    pub uids: Vec<String>,
}

#[serde(untagged)]
#[derive(Debug, Deserialize, Serialize)]
pub enum MessageDetail {
    ConnectionRequestAnswer(AcceptInviteMessageDetails),
    ConnectionRequest(SendInviteMessageDetails),
    ConnectionRequestResp(SendInviteMessageDetailsResponse),
    General(GeneralMessageDetail),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendRemoteMessage {
    #[serde(rename = "@type")]
    pub msg_type: MessageTypeV2,
    pub mtype: RemoteMessageType,
    #[serde(rename = "replyToMsgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_msg_id: Option<String>,
    #[serde(rename = "sendMsg")]
    pub send_msg: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "@msg")]
    msg: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendRemoteMessageResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    pub uid: String,
    pub sent: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RemoteMessageType {
    Other(String),
    ConnReq,
    ConnReqAnswer,
    CredOffer,
    CredReq,
    Cred,
    ProofReq,
    Proof,
}

impl Serialize for RemoteMessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            RemoteMessageType::ConnReq => "connReq",
            RemoteMessageType::ConnReqAnswer => "connReqAnswer",
            RemoteMessageType::CredOffer => "credOffer",
            RemoteMessageType::CredReq => "credReq",
            RemoteMessageType::Cred => "cred",
            RemoteMessageType::ProofReq => "proofReq",
            RemoteMessageType::Proof => "proof",
            RemoteMessageType::Other(_type) => _type,
        };
        Value::String(value.to_string()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RemoteMessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        match value.as_str() {
            Some("connReq") => Ok(RemoteMessageType::ConnReq),
            Some("connReqAnswer") => Ok(RemoteMessageType::ConnReqAnswer),
            Some("credOffer") => Ok(RemoteMessageType::CredOffer),
            Some("credReq") => Ok(RemoteMessageType::CredReq),
            Some("cred") => Ok(RemoteMessageType::Cred),
            Some("proofReq") => Ok(RemoteMessageType::ProofReq),
            Some("proof") => Ok(RemoteMessageType::Proof),
            Some(_type) => Ok(RemoteMessageType::Other(_type.to_string())),
            _ => Err(de::Error::custom("Unexpected message type."))
        }
    }
}


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum PayloadTypes {
    PayloadTypeV1(PayloadTypeV1),
    PayloadTypeV2(PayloadTypeV2),
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct PayloadTypeV1 {
    name: String,
    ver: String,
    fmt: String,
}

type PayloadTypeV2 = MessageTypeV2;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PayloadKinds {
    CredOffer,
    CredReq,
    Cred,
    Proof,
    ProofRequest,
    Other(String)
}

impl PayloadKinds {
    fn family(&self) -> MessageFamilies {
        match self {
            PayloadKinds::CredOffer => MessageFamilies::CredentialExchange,
            PayloadKinds::CredReq => MessageFamilies::CredentialExchange,
            PayloadKinds::Cred => MessageFamilies::CredentialExchange,
            PayloadKinds::Proof => MessageFamilies::CredentialExchange,
            PayloadKinds::ProofRequest => MessageFamilies::CredentialExchange,
            PayloadKinds::Other(family) => MessageFamilies::Unknown(family.to_string()),
        }
    }

    fn name<'a>(&'a self) -> &'a str {
        match settings::get_protocol_type() {
            settings::ProtocolTypes::V1 => {
                match self {
                    PayloadKinds::CredOffer => "CRED_OFFER",
                    PayloadKinds::CredReq => "CRED_REQ",
                    PayloadKinds::Cred => "CRED",
                    PayloadKinds::ProofRequest => "PROOF_REQUEST",
                    PayloadKinds::Proof => "PROOF",
                    PayloadKinds::Other(kind) => kind,
                }
            }
            settings::ProtocolTypes::V2 => {
                match self {
                    PayloadKinds::CredOffer => "credential-offer",
                    PayloadKinds::CredReq => "credential-request",
                    PayloadKinds::Cred => "credential",
                    PayloadKinds::ProofRequest => "presentation-request",
                    PayloadKinds::Proof => "presentation",
                    PayloadKinds::Other(kind) => kind,
                }
            }
        }
    }
}

impl PayloadTypes {
    pub fn build_v1(kind: PayloadKinds, fmt: &str) -> PayloadTypes {
        PayloadTypes::PayloadTypeV1(PayloadTypeV1 {
            name: kind.name().to_string(),
            ver: MESSAGE_VERSION_V1.to_string(),
            fmt: fmt.to_string(),
        })
    }

    pub fn build_v2(kind: PayloadKinds) -> PayloadTypes {
        PayloadTypes::PayloadTypeV2(PayloadTypeV2 {
            did: DID.to_string(),
            family: kind.family(),
            version: kind.family().version().to_string(),
            type_: kind.name().to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageStatusCode {
    Created,
    Sent,
    Pending,
    Accepted,
    Rejected,
    Reviewed,
}

impl Serialize for MessageStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            MessageStatusCode::Created => "MS-101",
            MessageStatusCode::Sent => "MS-102",
            MessageStatusCode::Pending => "MS-103",
            MessageStatusCode::Accepted => "MS-104",
            MessageStatusCode::Rejected => "MS-105",
            MessageStatusCode::Reviewed => "MS-106",
        };
        Value::String(value.to_string()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MessageStatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;
        match value.as_str() {
            Some("MS-101") => Ok(MessageStatusCode::Created),
            Some("MS-102") => Ok(MessageStatusCode::Sent),
            Some("MS-103") => Ok(MessageStatusCode::Pending),
            Some("MS-104") => Ok(MessageStatusCode::Accepted),
            Some("MS-105") => Ok(MessageStatusCode::Rejected),
            Some("MS-106") => Ok(MessageStatusCode::Reviewed),
            _ => Err(de::Error::custom("Unexpected message type."))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum A2AMessageKinds {
    Forward,
    Connect,
    Connected,
    SignUp,
    SignedUp,
    CreateAgent,
    AgentCreated,
    CreateKey,
    KeyCreated,
    CreateMessage,
    MessageDetail,
    MessageCreated,
    MessageSent,
    GetMessages,
    GetMessagesByConnections,
    Messages,
    UpdateMessageStatusByConnections,
    MessageStatusUpdatedByConnections,
    UpdateConnectionStatus,
    UpdateConfigs,
    ConfigsUpdated,
    UpdateConMethod,
    ConnectionRequest,
    ConnectionRequestAnswer,
    SendRemoteMessage,
    SendRemoteMessageResponse,
}

impl A2AMessageKinds {
    pub fn family(&self) -> MessageFamilies {
        match self {
            A2AMessageKinds::Forward => MessageFamilies::Routing,
            A2AMessageKinds::Connect => MessageFamilies::Onboarding,
            A2AMessageKinds::Connected => MessageFamilies::Onboarding,
            A2AMessageKinds::CreateAgent => MessageFamilies::Onboarding,
            A2AMessageKinds::AgentCreated => MessageFamilies::Onboarding,
            A2AMessageKinds::SignUp => MessageFamilies::Onboarding,
            A2AMessageKinds::SignedUp => MessageFamilies::Onboarding,
            A2AMessageKinds::CreateKey => MessageFamilies::Pairwise,
            A2AMessageKinds::KeyCreated => MessageFamilies::Pairwise,
            A2AMessageKinds::CreateMessage => MessageFamilies::Pairwise,
            A2AMessageKinds::MessageDetail => MessageFamilies::Pairwise,
            A2AMessageKinds::MessageCreated => MessageFamilies::Pairwise,
            A2AMessageKinds::MessageSent => MessageFamilies::Pairwise,
            A2AMessageKinds::GetMessages => MessageFamilies::Pairwise,
            A2AMessageKinds::GetMessagesByConnections => MessageFamilies::Pairwise,
            A2AMessageKinds::Messages => MessageFamilies::Pairwise,
            A2AMessageKinds::UpdateConnectionStatus => MessageFamilies::Pairwise,
            A2AMessageKinds::ConnectionRequest => MessageFamilies::Pairwise,
            A2AMessageKinds::ConnectionRequestAnswer => MessageFamilies::Pairwise,
            A2AMessageKinds::UpdateMessageStatusByConnections => MessageFamilies::Pairwise,
            A2AMessageKinds::MessageStatusUpdatedByConnections => MessageFamilies::Pairwise,
            A2AMessageKinds::UpdateConfigs => MessageFamilies::Configs,
            A2AMessageKinds::ConfigsUpdated => MessageFamilies::Configs,
            A2AMessageKinds::UpdateConMethod => MessageFamilies::Configs,
            A2AMessageKinds::SendRemoteMessage => MessageFamilies::Routing,
            A2AMessageKinds::SendRemoteMessageResponse => MessageFamilies::Routing,
        }
    }

    pub fn name(&self) -> String {
        match self {
            A2AMessageKinds::Forward => "FWD".to_string(),
            A2AMessageKinds::Connect => "CONNECT".to_string(),
            A2AMessageKinds::Connected => "CONNECTED".to_string(),
            A2AMessageKinds::CreateAgent => "CREATE_AGENT".to_string(),
            A2AMessageKinds::AgentCreated => "AGENT_CREATED".to_string(),
            A2AMessageKinds::SignUp => "SIGNUP".to_string(),
            A2AMessageKinds::SignedUp => "SIGNED_UP".to_string(),
            A2AMessageKinds::CreateKey => "CREATE_KEY".to_string(),
            A2AMessageKinds::KeyCreated => "KEY_CREATED".to_string(),
            A2AMessageKinds::CreateMessage => "CREATE_MSG".to_string(),
            A2AMessageKinds::MessageDetail => "MSG_DETAIL".to_string(),
            A2AMessageKinds::MessageCreated => "MSG_CREATED".to_string(),
            A2AMessageKinds::MessageSent => "MSGS_SENT".to_string(),
            A2AMessageKinds::GetMessages => "GET_MSGS".to_string(),
            A2AMessageKinds::GetMessagesByConnections => "GET_MSGS_BY_CONNS".to_string(),
            A2AMessageKinds::UpdateMessageStatusByConnections => "UPDATE_MSG_STATUS_BY_CONNS".to_string(),
            A2AMessageKinds::MessageStatusUpdatedByConnections => "MSG_STATUS_UPDATED_BY_CONNS".to_string(),
            A2AMessageKinds::Messages => "MSGS".to_string(),
            A2AMessageKinds::UpdateConnectionStatus => "UPDATE_CONN_STATUS".to_string(),
            A2AMessageKinds::ConnectionRequest => "CONN_REQUEST".to_string(),
            A2AMessageKinds::ConnectionRequestAnswer => "CONN_REQUEST_ANSWER".to_string(),
            A2AMessageKinds::UpdateConfigs => "UPDATE_CONFIGS".to_string(),
            A2AMessageKinds::ConfigsUpdated => "CONFIGS_UPDATED".to_string(),
            A2AMessageKinds::UpdateConMethod => "UPDATE_CONNECTION_METHOD".to_string(),
            A2AMessageKinds::SendRemoteMessage => "SEND_REMOTE_MSG".to_string(),
            A2AMessageKinds::SendRemoteMessageResponse => "REMOTE_MSG_SENT".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Thread {
    pub thid: String,
    pub pthid: Option<String>,
    pub sender_order: u32,
    pub received_orders: Option<HashMap<String, u32>>,
}

pub fn prepare_message_for_agency(message: &A2AMessage, agency_did: &str) -> Result<Vec<u8>, u32> {
    match settings::get_protocol_type() {
        settings::ProtocolTypes::V1 => bundle_for_agency_v1(message, &agency_did),
        settings::ProtocolTypes::V2 => pack_for_agency_v2(message, agency_did)
    }
}

fn bundle_for_agency_v1(message: &A2AMessage, agency_did: &str) -> Result<Vec<u8>, u32> {
    let agent_vk = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY)?;
    let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;

    let message = rmp_serde::to_vec_named(&message).or(Err(error::UNKNOWN_ERROR.code_num))?;
    let message = Bundled::create(message).encode()?;

    let message = crypto::prep_msg(&my_vk, &agent_vk, &message[..])?;

    prepare_forward_message(message, agency_did)
}

fn pack_for_agency_v2(message: &A2AMessage, agency_did: &str) -> Result<Vec<u8>, u32> {
    let agent_vk = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY)?;
    let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;

    let message = serde_json::to_string(&message).or(Err(error::SERIALIZATION_ERROR.code_num))?;
    let receiver_keys = ::serde_json::to_string(&vec![&agent_vk]).or(Err(error::SERIALIZATION_ERROR.code_num))?;

    let message = crypto::pack_message(Some(&my_vk), &receiver_keys, message.as_bytes())?;

    prepare_forward_message(message, agency_did)
}

fn parse_response_from_agency(response: &Vec<u8>) -> Result<Vec<A2AMessage>, u32> {
    match settings::get_protocol_type() {
        settings::ProtocolTypes::V1 => parse_response_from_agency_v1(response),
        settings::ProtocolTypes::V2 => parse_response_from_agency_v2(response)
    }
}

fn parse_response_from_agency_v1(response: &Vec<u8>) -> Result<Vec<A2AMessage>, u32> {
    let verkey = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY)?;
    let (_, data) = crypto::parse_msg(&verkey, &response)?;
    let bundle: Bundled<Vec<u8>> = bundle_from_u8(data)?;
    bundle.bundled
        .iter()
        .map(|msg| rmp_serde::from_slice(msg)
            .map_err(|err| error::INVALID_JSON.code_num))
        .collect::<Result<Vec<A2AMessage>, u32>>()
}

fn parse_response_from_agency_v2(response: &Vec<u8>) -> Result<Vec<A2AMessage>, u32> {
    let unpacked_msg = crypto::unpack_message(&response[..])?;

    let message: Value = ::serde_json::from_slice(unpacked_msg.as_slice())
        .or(Err(error::INVALID_JSON.code_num))?;

    let message = message["message"].as_str().ok_or(error::INVALID_JSON.code_num)?;

    let message: A2AMessage = serde_json::from_str(message)
        .map_err(|ec| { error::INVALID_JSON.code_num })?;
    Ok(vec![message])
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Bundled<T> {
    bundled: Vec<T>,
}

impl<T> Bundled<T> {
    pub fn create(bundled: T) -> Bundled<T> {
        let mut vec = Vec::new();
        vec.push(bundled);
        Bundled {
            bundled: vec,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, u32> where T: serde::Serialize {
        rmp_serde::to_vec_named(self)
            .map_err(|err| {
                error!("Could not convert bundle to messagepack: {}", err);
                error::INVALID_MSGPACK.code_num
            })
    }
}

pub fn try_i8_bundle(data: Vec<u8>) -> Result<Bundled<Vec<u8>>, u32> {
    let bundle: Bundled<Vec<i8>> =
        rmp_serde::from_slice(&data[..])
            .map_err(|err| {
                warn!("could not deserialize bundle with i8, will try u8");
                error::INVALID_MSGPACK.code_num
            })?;

    let mut new_bundle: Bundled<Vec<u8>> = Bundled { bundled: Vec::new() };
    for i in bundle.bundled {
        let mut buf: Vec<u8> = Vec::new();
        for j in i { buf.push(j as u8); }
        new_bundle.bundled.push(buf);
    }
    Ok(new_bundle)
}

pub fn to_u8(bytes: &Vec<i8>) -> Vec<u8> {
    bytes.iter().map(|i| *i as u8).collect()
}

pub fn to_i8(bytes: &Vec<u8>) -> Vec<i8> {
    bytes.iter().map(|i| *i as i8).collect()
}

pub fn bundle_from_u8(data: Vec<u8>) -> Result<Bundled<Vec<u8>>, u32> {
    try_i8_bundle(data.clone())
        .or_else(|_| rmp_serde::from_slice::<Bundled<Vec<u8>>>(&data[..]))
        .map_err(|err| {
            error!("could not deserialize bundle with i8 or u8: {}", err);
            error::INVALID_MSGPACK.code_num
        })
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Payload {
    #[serde(rename = "@type")]
    pub type_: PayloadTypes,
    #[serde(rename = "@msg")]
    pub msg: String,
}

impl Payload {
    // TODO: Refactor Error
    // this will become a CommonError, because multiple types (Connection/Issuer Credential) use this function
    // Possibly this function moves out of this file.
    // On second thought, this should stick as a ConnectionError.
    pub fn encrypted(my_vk: &str, their_vk: &str, data: &str, msg_type: PayloadKinds) -> Result<Vec<u8>, u32> {
        match settings::ProtocolTypes::from(settings::get_protocol_type()) {
            settings::ProtocolTypes::V1 => {
                let payload = ::messages::Payload {
                    type_: PayloadTypes::build_v1(msg_type, "json"),
                    msg: data.to_string(),
                };

                let bytes = rmp_serde::to_vec_named(&payload)
                    .map_err(|err| {
                        error!("could not encode create_keys msg: {}", err);
                        error::INVALID_MSGPACK.code_num
                    })?;

                trace!("Sending payload: {:?}", bytes);
                ::utils::libindy::crypto::prep_msg(&my_vk, &their_vk, &bytes)
            }
            settings::ProtocolTypes::V2 => {
                let payload = ::messages::Payload {
                    type_: PayloadTypes::build_v2(msg_type),
                    msg: data.to_string(),
                };

                let message = serde_json::to_string(&payload)
                    .map_err(|err| {
                        error!("could not serialize create_keys msg: {}", err);
                        error::INVALID_MSGPACK.code_num
                    })?;

                let receiver_keys = serde_json::to_string(&vec![&their_vk]).or(Err(error::SERIALIZATION_ERROR.code_num))?;

                trace!("Sending payload: {:?}", message.as_bytes());
                ::utils::libindy::crypto::pack_message(Some(my_vk), &receiver_keys, message.as_bytes())
            }
        }
    }

    pub fn decrypted(my_vk: &str, payload: &Vec<i8>) -> Result<String, u32> {
        match settings::ProtocolTypes::from(settings::get_protocol_type()) {
            settings::ProtocolTypes::V1 => {
                let (_, data) = crypto::parse_msg(&my_vk, &to_u8(payload))?;

                let my_payload: Payload = rmp_serde::from_slice(&data[..])
                    .map_err(|err| {
                        error!("could not deserialize bundle with i8 or u8: {}", err);
                        error::INVALID_MSGPACK.code_num
                    })?;
                Ok(my_payload.msg)
            }
            settings::ProtocolTypes::V2 => {
                let unpacked_msg = crypto::unpack_message(&to_u8(payload))?;

                let message: Value = ::serde_json::from_slice(unpacked_msg.as_slice())
                    .or(Err(error::INVALID_JSON.code_num))?;

                let message = message["message"].as_str().ok_or(error::INVALID_JSON.code_num)?.to_string();

                let my_payload: Payload = serde_json::from_str(&message)
                    .map_err(|err| {
                        error!("could not deserialize bundle with i8 or u8: {}", err);
                        error::INVALID_MSGPACK.code_num
                    })?;
                Ok(my_payload.msg)
            }
        }
    }
}

fn prepare_forward_message(message: Vec<u8>, did: &str) -> Result<Vec<u8>, u32> {
    let agency_vk = settings::get_config_value(settings::CONFIG_AGENCY_VERKEY)?;

    let message = Forward::new(did.to_string(), message);

    match settings::get_protocol_type() {
        settings::ProtocolTypes::V1 => prepare_forward_message_for_agency_v1(&message, &agency_vk),
        settings::ProtocolTypes::V2 => prepare_forward_message_for_agency_v2(&message, &agency_vk)
    }
}

fn prepare_forward_message_for_agency_v1(message: &Forward, agency_vk: &str) -> Result<Vec<u8>, u32> {
    let message = rmp_serde::to_vec_named(message).or(Err(error::UNKNOWN_ERROR.code_num))?;
    let message = Bundled::create(message).encode()?;
    crypto::prep_anonymous_msg(agency_vk, &message[..])
}

fn prepare_forward_message_for_agency_v2(message: &Forward, agency_vk: &str) -> Result<Vec<u8>, u32> {
    let message = serde_json::to_string(message).or(Err(error::SERIALIZATION_ERROR.code_num))?;
    let receiver_keys = serde_json::to_string(&vec![agency_vk]).or(Err(error::SERIALIZATION_ERROR.code_num))?;
    crypto::pack_message(None, &receiver_keys, message.as_bytes())
}

pub fn prepare_message_for_agent(messages: Vec<A2AMessage>, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<Vec<u8>, u32> {
    match settings::get_protocol_type() {
        settings::ProtocolTypes::V1 => prepare_message_for_agent_v1(messages, pw_vk, agent_did, agent_vk),
        settings::ProtocolTypes::V2 => prepare_message_for_agent_v2(messages, pw_vk, agent_did, agent_vk)
    }
}

fn prepare_message_for_agent_v1(messages: Vec<A2AMessage>, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<Vec<u8>, u32> {
    let message = messages
        .iter()
        .map(|msg| rmp_serde::to_vec_named(msg))
        .collect::<Result<Vec<_>, _>>()
        .map(|msgs| Bundled { bundled: msgs })
        .and_then(|bundle| rmp_serde::to_vec_named(&bundle))
        .or(Err(error::SERIALIZATION_ERROR.code_num))?;

    let message = crypto::prep_msg(&pw_vk, agent_vk, &message[..])?;

    /* forward to did */
    let message = Forward::new(agent_did.to_owned(), message);

    let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

    bundle_for_agency_v1(&A2AMessage::Version1(A2AMessageV1::Forward(message)), &to_did)
}

fn prepare_message_for_agent_v2(messages: Vec<A2AMessage>, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<Vec<u8>, u32> {
    let message = messages.get(0).ok_or(error::SERIALIZATION_ERROR.code_num)?;
    let message = serde_json::to_string(message).or(Err(error::SERIALIZATION_ERROR.code_num))?;
    let receiver_keys = serde_json::to_string(&vec![&agent_vk]).or(Err(error::SERIALIZATION_ERROR.code_num))?;

    let message = crypto::pack_message(Some(pw_vk), &receiver_keys, message.as_bytes())?;

    /* forward to did */
    let message = Forward::new(agent_did.to_owned(), message);

    let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

    pack_for_agency_v2(&A2AMessage::Version2(A2AMessageV2::Forward(message)), &to_did)
}

pub trait GeneralMessage {
    type Msg;

    //todo: deserialize_message

    fn to(&mut self, to_did: &str) -> Result<&mut Self, u32> {
        validation::validate_did(to_did)?;
        self.set_to_did(to_did.to_string());
        Ok(self)
    }

    fn to_vk(&mut self, to_vk: &str) -> Result<&mut Self, u32> {
        validation::validate_verkey(to_vk)?;
        self.set_to_vk(to_vk.to_string());
        Ok(self)
    }

    fn agent_did(&mut self, did: &str) -> Result<&mut Self, u32> {
        validation::validate_did(did)?;
        self.set_agent_did(did.to_string());
        Ok(self)
    }

    fn agent_vk(&mut self, to_vk: &str) -> Result<&mut Self, u32> {
        validation::validate_verkey(to_vk)?;
        self.set_agent_vk(to_vk.to_string());
        Ok(self)
    }

    fn set_to_vk(&mut self, to_vk: String);
    fn set_to_did(&mut self, to_did: String);
    fn set_agent_did(&mut self, did: String);
    fn set_agent_vk(&mut self, vk: String);

    fn prepare_request(&mut self) -> Result<Vec<u8>, u32>;
}

pub fn create_keys() -> CreateKeyBuilder { CreateKeyBuilder::create() }

pub fn send_invite() -> SendInviteBuilder { SendInviteBuilder::create() }

pub fn delete_connection() -> DeleteConnectionBuilder { DeleteConnectionBuilder::create() }

pub fn accept_invite() -> AcceptInviteBuilder { AcceptInviteBuilder::create() }

pub fn update_data() -> UpdateProfileDataBuilder { UpdateProfileDataBuilder::create() }

pub fn get_messages() -> GetMessagesBuilder { GetMessagesBuilder::create() }

pub fn send_message() -> SendMessageBuilder { SendMessageBuilder::create() }

pub fn proof_request() -> ProofRequestMessage { ProofRequestMessage::create() }

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_to_u8() {
        let vec: Vec<i8> = vec![-127, -89, 98, 117, 110, 100, 108, 101, 100, -111, -36, 5, -74];

        let buf = to_u8(&vec);
        println!("new bundle: {:?}", buf);
    }

    #[test]
    fn test_to_i8() {
        let vec: Vec<u8> = vec![129, 167, 98, 117, 110, 100, 108, 101, 100, 145, 220, 19, 13];
        let buf = to_i8(&vec);
        println!("new bundle: {:?}", buf);
    }
}
