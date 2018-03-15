use librespot::core::session::Session;
use librespot::metadata::Events;

pub fn handle_events(event: Events, session: Session) {
    match event {
        Events::SessionActive { became_active_at } => {
            info!("Session [{}]", session.session_id());
            info!("Active at: {:?}", became_active_at);
        }
        _ => {
            info!("Matched: {:?}", event);
        }
    }
}
