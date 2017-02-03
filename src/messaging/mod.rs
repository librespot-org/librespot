pub struct UpdateMessage;

#[derive(Debug)]
pub struct MercuryResponse {
    pub uri: String,
    pub payload: Vec<Vec<u8>>,
}

pub enum SpircMessage {
    MercuryMsg(MercuryResponse),
    UpdateMsg(UpdateMessage)
}

implement_sender!(name => MercuryResponseSender, 
                  wrap => MercuryResponse, 
                  with => SpircMessage, 
                  variant => MercuryMsg);

implement_sender!(name => UpdateMessageSender, 
                  wrap => UpdateMessage, 
                  with => SpircMessage, 
                  variant => UpdateMsg);