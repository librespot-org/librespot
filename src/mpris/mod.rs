//! [MPRIS] D-Bus interface, allowing desktop environments and other tools to display what is
//! playing and to control the player.
//!
//! [MPRIS]: https://specifications.freedesktop.org/mpris-spec/latest/

use std::{process, sync::Arc};

use librespot_connect::Spirc;
use log::{debug, warn};
use tokio::sync::mpsc;
use zbus::{connection, fdo::RequestNameFlags};

use librespot::playback::player::Player;

mod metadata;
mod player;
mod root;
mod task;
mod types;

use player::MprisPlayerService;
use root::MprisService;
use task::MprisTask;

const OBJECT_PATH: &str = "/org/mpris/MediaPlayer2";
const BUS_NAME: &str = "org.mpris.MediaPlayer2.librespot";

enum MprisCommand {
    SetSpirc(Spirc),
    Quit,
}

pub struct MprisEventHandler {
    cmd_tx: mpsc::UnboundedSender<MprisCommand>,
    join_handle: tokio::task::JoinHandle<()>,
}

impl MprisEventHandler {
    /// Connects to the D-Bus session bus and starts serving the MPRIS interfaces.
    ///
    /// `name` is used as the MPRIS identity, and `initial_volume` as the volume until the first
    /// volume change, as Spirc only reports it once active. If `org.mpris.MediaPlayer2.librespot` is already
    /// taken (e.g. by another librespot instance), the bus name is suffixed with the process id,
    /// as recommended by the specification.
    pub async fn spawn(
        player: Arc<Player>,
        name: &str,
        initial_volume: u16,
        desktop_entry: Option<&str>,
    ) -> zbus::Result<MprisEventHandler> {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        let mpris_service = MprisService {
            identity: name.to_string(),
            desktop_entry: desktop_entry.map(str::to_string),
        };

        let connection = connection::Builder::session()?
            .serve_at(OBJECT_PATH, mpris_service)?
            .serve_at(OBJECT_PATH, MprisPlayerService::new(initial_volume))?
            .build()
            .await?;

        // Without `DoNotQueue`, the request would be queued instead of failing if the name is
        // already owned.
        let flags = RequestNameFlags::DoNotQueue.into();
        let bus_name = match connection.request_name_with_flags(BUS_NAME, flags).await {
            Ok(_) => BUS_NAME.to_string(),
            Err(zbus::Error::NameTaken) => {
                let bus_name = format!("{BUS_NAME}.instance{}", process::id());
                warn!("D-Bus name {BUS_NAME} taken, trying with pid specific name: {bus_name}");
                connection
                    .request_name_with_flags(bus_name.as_str(), flags)
                    .await?;
                bus_name
            }
            Err(e) => return Err(e),
        };
        debug!("MPRIS interface registered on the D-Bus session bus as {bus_name}");

        let mpris_task = MprisTask::new(player, connection, cmd_rx);
        let join_handle = tokio::spawn(mpris_task.run());

        Ok(MprisEventHandler {
            cmd_tx,
            join_handle,
        })
    }

    pub fn set_spirc(&self, spirc: Spirc) {
        let _ = self.cmd_tx.send(MprisCommand::SetSpirc(spirc));
    }

    pub async fn quit_and_join(self) {
        let _ = self.cmd_tx.send(MprisCommand::Quit);
        let _ = self.join_handle.await;
    }
}
