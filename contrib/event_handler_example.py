#!/usr/bin/python3
import os
import json
from datetime import datetime

player_event = os.getenv('PLAYER_EVENT')

json_dict = {
   'event_time': str(datetime.now()),
   'event': player_event,
} 

if player_event in ('session_connected', 'session_disconnected'):
    json_dict['user_name'] = os.environ['USER_NAME']
    json_dict['connection_id'] = os.environ['CONNECTION_ID']

elif player_event == 'session_client_changed':
    json_dict['client_id'] = os.environ['CLIENT_ID']
    json_dict['client_name'] = os.environ['CLIENT_NAME']
    json_dict['client_brand_name'] = os.environ['CLIENT_BRAND_NAME']
    json_dict['client_model_name'] = os.environ['CLIENT_MODEL_NAME']

elif player_event == 'shuffle_changed':
    json_dict['shuffle'] = os.environ['SHUFFLE']

elif player_event == 'repeat_changed':
    json_dict['repeat'] = os.environ['REPEAT']

elif player_event == 'auto_play_changed':
    json_dict['auto_play'] = os.environ['AUTO_PLAY']

elif player_event == 'filter_explicit_content_changed':
    json_dict['filter'] = os.environ['FILTER']

elif player_event == 'volume_changed':
    json_dict['volume'] = os.environ['VOLUME']

elif player_event in ('seeked', 'position_correction', 'playing', 'paused'):
    json_dict['track_id'] = os.environ['TRACK_ID']
    json_dict['position_ms'] = os.environ['POSITION_MS']

elif player_event in ('unavailable', 'end_of_track', 'preload_next', 'preloading', 'loading', 'stopped'): 
    json_dict['track_id'] = os.environ['TRACK_ID']

elif player_event == 'track_changed':
    common_metadata_fields = {}
    item_type = os.environ['ITEM_TYPE']
    common_metadata_fields['item_type'] = item_type
    common_metadata_fields['track_id'] = os.environ['TRACK_ID']
    common_metadata_fields['uri'] = os.environ['URI']
    common_metadata_fields['name'] = os.environ['NAME']
    common_metadata_fields['duration_ms'] = os.environ['DURATION_MS']
    common_metadata_fields['is_explicit'] = os.environ['IS_EXPLICIT']
    common_metadata_fields['language'] = os.environ['LANGUAGE'].split('\n')
    common_metadata_fields['covers'] = os.environ['COVERS'].split('\n')
    json_dict['common_metadata_fields'] = common_metadata_fields
    

    if item_type == 'Track':
        track_metadata_fields = {}
        track_metadata_fields['number'] = os.environ['NUMBER']
        track_metadata_fields['disc_number'] = os.environ['DISC_NUMBER']
        track_metadata_fields['popularity'] = os.environ['POPULARITY']
        track_metadata_fields['album'] = os.environ['ALBUM']
        track_metadata_fields['artists'] = os.environ['ARTISTS'].split('\n')
        track_metadata_fields['album_artists'] = os.environ['ALBUM_ARTISTS'].split('\n')
        json_dict['track_metadata_fields'] = track_metadata_fields

    elif item_type == 'Episode':
        episode_metadata_fields = {}
        episode_metadata_fields['show_name'] = os.environ['SHOW_NAME']
        publish_time = datetime.utcfromtimestamp(int(os.environ['PUBLISH_TIME'])).strftime('%Y-%m-%d')
        episode_metadata_fields['publish_time'] = publish_time
        episode_metadata_fields['description'] = os.environ['DESCRIPTION']
        json_dict['episode_metadata_fields'] = episode_metadata_fields

print(json.dumps(json_dict, indent = 4))
