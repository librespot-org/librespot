use super::*;
use crate::{audio_backend::SinkResult, decoder::DecoderResult, mixer::NoOpVolume};

struct TestDecoder {
    seeks: Arc<Mutex<Vec<u32>>>,
    packets: Arc<AtomicUsize>,
}

impl AudioDecoder for TestDecoder {
    fn seek(&mut self, position_ms: u32) -> Result<u32, crate::decoder::DecoderError> {
        self.seeks.lock().unwrap().push(position_ms);
        Ok(position_ms)
    }

    fn next_packet(&mut self) -> DecoderResult<Option<(AudioPacketPosition, AudioPacket)>> {
        let packet = self.packets.fetch_add(1, Ordering::Relaxed);
        // Finite fake audio makes the broken loop fail at EOF instead of hanging.
        if packet >= 2_000 {
            return Ok(None);
        }
        Ok(Some((
            AudioPacketPosition {
                position_ms: packet as u32,
                skipped: false,
            },
            AudioPacket::Samples(vec![0.0; 2]),
        )))
    }
}

struct TestSink;

impl Sink for TestSink {
    fn write(&mut self, _: AudioPacket, _: &mut Converter) -> SinkResult<()> {
        Ok(())
    }
}

fn playing_player(
    commands: mpsc::UnboundedReceiver<PlayerCommand>,
    decoder: TestDecoder,
) -> PlayerInternal {
    let track_id = SpotifyUri::Local {
        artist: String::new(),
        album_title: String::new(),
        track_title: "scheduler regression".into(),
        duration: Duration::from_secs(180),
    };
    let audio_item = AudioItem {
        track_id: track_id.clone(),
        uri: track_id.to_uri(),
        files: Default::default(),
        name: "scheduler regression".into(),
        covers: vec![],
        language: vec![],
        duration_ms: 180_000,
        is_explicit: false,
        availability: Ok(()),
        alternatives: None,
        unique_fields: UniqueFields::Local {
            artists: None,
            album: None,
            album_artists: None,
            number: None,
            disc_number: None,
            path: Default::default(),
        },
    };
    let config = PlayerConfig::default();
    PlayerInternal {
        session: Session::new(Default::default(), None),
        converter: Converter::new(config.ditherer),
        normalisation_knee_factor: 1.0 / (8.0 * config.normalisation_knee_db),
        config,
        commands,
        load_handles: Arc::new(Mutex::new(HashMap::new())),
        state: PlayerState::Playing {
            track_id,
            play_request_id: 0,
            decoder: Box::new(decoder),
            normalisation_data: NormalisationData::default(),
            audio_item,
            normalisation_factor: 1.0,
            stream_loader_controller: StreamLoaderController::from_local_file(1_000_000),
            bytes_per_second: 176_400,
            duration_ms: 180_000,
            stream_position_ms: 0,
            reported_nominal_start_time: None,
            suggested_to_preload_next_track: true,
            is_explicit: false,
        },
        preload: PlayerPreload::None,
        sink: Box::new(TestSink),
        sink_status: SinkStatus::Running,
        sink_event_callback: None,
        volume_getter: Box::new(NoOpVolume),
        event_senders: vec![],
        normalisation_peaks: [0.0; 2],
        normalisation_integrators: [0.0; 2],
        normalisation_channel: 0,
        auto_normalise_as_album: false,
        player_id: 0,
        play_request_id_generator: SeqGenerator::new(0),
        last_progress_update: Instant::now(),
        local_file_lookup: Arc::new(create_local_file_lookup(&[])),
    }
}

/// Records how many seeks the decoder had made each time the sink was flushed.
struct FlushProbe {
    seeks: Arc<Mutex<Vec<u32>>>,
    flushed_after: Arc<Mutex<Vec<usize>>>,
}

impl Sink for FlushProbe {
    fn flush(&mut self) -> SinkResult<()> {
        let seeks = self.seeks.lock().unwrap().len();
        self.flushed_after.lock().unwrap().push(seeks);
        Ok(())
    }

    fn write(&mut self, _: AudioPacket, _: &mut Converter) -> SinkResult<()> {
        Ok(())
    }
}

#[tokio::test]
async fn seeking_flushes_the_sink_before_the_decoder_moves() {
    let (_commands, rx) = mpsc::unbounded_channel();
    let seeks = Arc::new(Mutex::new(vec![]));
    let mut player = playing_player(
        rx,
        TestDecoder {
            seeks: seeks.clone(),
            packets: Default::default(),
        },
    );
    let flushed_after = Arc::new(Mutex::new(vec![]));
    player.sink = Box::new(FlushProbe {
        seeks: seeks.clone(),
        flushed_after: flushed_after.clone(),
    });
    player.handle_command_seek(30_000).unwrap();
    assert_eq!(*seeks.lock().unwrap(), [30_000]);
    assert_eq!(
        *flushed_after.lock().unwrap(),
        [0],
        "the sink is flushed once, before the decoder seeks"
    );
}
