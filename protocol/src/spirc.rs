// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Frame {
    // message fields
    version: ::std::option::Option<u32>,
    ident: ::protobuf::SingularField<::std::string::String>,
    protocol_version: ::protobuf::SingularField<::std::string::String>,
    seq_nr: ::std::option::Option<u32>,
    typ: ::std::option::Option<MessageType>,
    device_state: ::protobuf::SingularPtrField<DeviceState>,
    goodbye: ::protobuf::SingularPtrField<Goodbye>,
    state: ::protobuf::SingularPtrField<State>,
    position: ::std::option::Option<u32>,
    volume: ::std::option::Option<u32>,
    state_update_id: ::std::option::Option<i64>,
    recipient: ::protobuf::RepeatedField<::std::string::String>,
    context_player_state: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    new_name: ::protobuf::SingularField<::std::string::String>,
    metadata: ::protobuf::SingularPtrField<Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Frame {}

impl Frame {
    pub fn new() -> Frame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Frame {
        static mut instance: ::protobuf::lazy::Lazy<Frame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Frame,
        };
        unsafe {
            instance.get(Frame::new)
        }
    }

    // optional uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.version
    }

    // optional string ident = 2;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: ::std::string::String) {
        self.ident = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut ::std::string::String {
        if self.ident.is_none() {
            self.ident.set_default();
        };
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> ::std::string::String {
        self.ident.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ident(&self) -> &str {
        match self.ident.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ident
    }

    // optional string protocol_version = 3;

    pub fn clear_protocol_version(&mut self) {
        self.protocol_version.clear();
    }

    pub fn has_protocol_version(&self) -> bool {
        self.protocol_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol_version(&mut self, v: ::std::string::String) {
        self.protocol_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_protocol_version(&mut self) -> &mut ::std::string::String {
        if self.protocol_version.is_none() {
            self.protocol_version.set_default();
        };
        self.protocol_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_protocol_version(&mut self) -> ::std::string::String {
        self.protocol_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_protocol_version(&self) -> &str {
        match self.protocol_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_protocol_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.protocol_version
    }

    fn mut_protocol_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.protocol_version
    }

    // optional uint32 seq_nr = 4;

    pub fn clear_seq_nr(&mut self) {
        self.seq_nr = ::std::option::Option::None;
    }

    pub fn has_seq_nr(&self) -> bool {
        self.seq_nr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_nr(&mut self, v: u32) {
        self.seq_nr = ::std::option::Option::Some(v);
    }

    pub fn get_seq_nr(&self) -> u32 {
        self.seq_nr.unwrap_or(0)
    }

    fn get_seq_nr_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_nr
    }

    fn mut_seq_nr_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_nr
    }

    // optional .MessageType typ = 5;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: MessageType) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> MessageType {
        self.typ.unwrap_or(MessageType::kMessageTypeHello)
    }

    fn get_typ_for_reflect(&self) -> &::std::option::Option<MessageType> {
        &self.typ
    }

    fn mut_typ_for_reflect(&mut self) -> &mut ::std::option::Option<MessageType> {
        &mut self.typ
    }

    // optional .DeviceState device_state = 7;

    pub fn clear_device_state(&mut self) {
        self.device_state.clear();
    }

    pub fn has_device_state(&self) -> bool {
        self.device_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_state(&mut self, v: DeviceState) {
        self.device_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_state(&mut self) -> &mut DeviceState {
        if self.device_state.is_none() {
            self.device_state.set_default();
        };
        self.device_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_state(&mut self) -> DeviceState {
        self.device_state.take().unwrap_or_else(|| DeviceState::new())
    }

    pub fn get_device_state(&self) -> &DeviceState {
        self.device_state.as_ref().unwrap_or_else(|| DeviceState::default_instance())
    }

    fn get_device_state_for_reflect(&self) -> &::protobuf::SingularPtrField<DeviceState> {
        &self.device_state
    }

    fn mut_device_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DeviceState> {
        &mut self.device_state
    }

    // optional .Goodbye goodbye = 11;

    pub fn clear_goodbye(&mut self) {
        self.goodbye.clear();
    }

    pub fn has_goodbye(&self) -> bool {
        self.goodbye.is_some()
    }

    // Param is passed by value, moved
    pub fn set_goodbye(&mut self, v: Goodbye) {
        self.goodbye = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_goodbye(&mut self) -> &mut Goodbye {
        if self.goodbye.is_none() {
            self.goodbye.set_default();
        };
        self.goodbye.as_mut().unwrap()
    }

    // Take field
    pub fn take_goodbye(&mut self) -> Goodbye {
        self.goodbye.take().unwrap_or_else(|| Goodbye::new())
    }

    pub fn get_goodbye(&self) -> &Goodbye {
        self.goodbye.as_ref().unwrap_or_else(|| Goodbye::default_instance())
    }

    fn get_goodbye_for_reflect(&self) -> &::protobuf::SingularPtrField<Goodbye> {
        &self.goodbye
    }

    fn mut_goodbye_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Goodbye> {
        &mut self.goodbye
    }

    // optional .State state = 12;

    pub fn clear_state(&mut self) {
        self.state.clear();
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: State) {
        self.state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state(&mut self) -> &mut State {
        if self.state.is_none() {
            self.state.set_default();
        };
        self.state.as_mut().unwrap()
    }

    // Take field
    pub fn take_state(&mut self) -> State {
        self.state.take().unwrap_or_else(|| State::new())
    }

    pub fn get_state(&self) -> &State {
        self.state.as_ref().unwrap_or_else(|| State::default_instance())
    }

    fn get_state_for_reflect(&self) -> &::protobuf::SingularPtrField<State> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<State> {
        &mut self.state
    }

    // optional uint32 position = 13;

    pub fn clear_position(&mut self) {
        self.position = ::std::option::Option::None;
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: u32) {
        self.position = ::std::option::Option::Some(v);
    }

    pub fn get_position(&self) -> u32 {
        self.position.unwrap_or(0)
    }

    fn get_position_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.position
    }

    fn mut_position_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.position
    }

    // optional uint32 volume = 14;

    pub fn clear_volume(&mut self) {
        self.volume = ::std::option::Option::None;
    }

    pub fn has_volume(&self) -> bool {
        self.volume.is_some()
    }

    // Param is passed by value, moved
    pub fn set_volume(&mut self, v: u32) {
        self.volume = ::std::option::Option::Some(v);
    }

    pub fn get_volume(&self) -> u32 {
        self.volume.unwrap_or(0)
    }

    fn get_volume_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.volume
    }

    fn mut_volume_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.volume
    }

    // optional int64 state_update_id = 17;

    pub fn clear_state_update_id(&mut self) {
        self.state_update_id = ::std::option::Option::None;
    }

    pub fn has_state_update_id(&self) -> bool {
        self.state_update_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state_update_id(&mut self, v: i64) {
        self.state_update_id = ::std::option::Option::Some(v);
    }

    pub fn get_state_update_id(&self) -> i64 {
        self.state_update_id.unwrap_or(0)
    }

    fn get_state_update_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.state_update_id
    }

    fn mut_state_update_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.state_update_id
    }

    // repeated string recipient = 18;

    pub fn clear_recipient(&mut self) {
        self.recipient.clear();
    }

    // Param is passed by value, moved
    pub fn set_recipient(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.recipient = v;
    }

    // Mutable pointer to the field.
    pub fn mut_recipient(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.recipient
    }

    // Take field
    pub fn take_recipient(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.recipient, ::protobuf::RepeatedField::new())
    }

    pub fn get_recipient(&self) -> &[::std::string::String] {
        &self.recipient
    }

    fn get_recipient_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.recipient
    }

    fn mut_recipient_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.recipient
    }

    // optional bytes context_player_state = 19;

    pub fn clear_context_player_state(&mut self) {
        self.context_player_state.clear();
    }

    pub fn has_context_player_state(&self) -> bool {
        self.context_player_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_player_state(&mut self, v: ::std::vec::Vec<u8>) {
        self.context_player_state = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context_player_state(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.context_player_state.is_none() {
            self.context_player_state.set_default();
        };
        self.context_player_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_context_player_state(&mut self) -> ::std::vec::Vec<u8> {
        self.context_player_state.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_context_player_state(&self) -> &[u8] {
        match self.context_player_state.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_context_player_state_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.context_player_state
    }

    fn mut_context_player_state_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.context_player_state
    }

    // optional string new_name = 20;

    pub fn clear_new_name(&mut self) {
        self.new_name.clear();
    }

    pub fn has_new_name(&self) -> bool {
        self.new_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_name(&mut self, v: ::std::string::String) {
        self.new_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_name(&mut self) -> &mut ::std::string::String {
        if self.new_name.is_none() {
            self.new_name.set_default();
        };
        self.new_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_name(&mut self) -> ::std::string::String {
        self.new_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_new_name(&self) -> &str {
        match self.new_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_new_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.new_name
    }

    fn mut_new_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.new_name
    }

    // optional .Metadata metadata = 25;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        };
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        self.metadata.take().unwrap_or_else(|| Metadata::new())
    }

    pub fn get_metadata(&self) -> &Metadata {
        self.metadata.as_ref().unwrap_or_else(|| Metadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Metadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for Frame {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ident)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.protocol_version)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_nr = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.typ = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.device_state)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.goodbye)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.state)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.position = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.volume = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.state_update_id = ::std::option::Option::Some(tmp);
                },
                18 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.recipient)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.context_player_state)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.new_name)?;
                },
                25 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ident.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.protocol_version.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.seq_nr {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.typ {
            my_size += ::protobuf::rt::enum_size(5, v);
        };
        if let Some(v) = self.device_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.goodbye.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.position {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.volume {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.state_update_id {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.recipient {
            my_size += ::protobuf::rt::string_size(18, &value);
        };
        if let Some(v) = self.context_player_state.as_ref() {
            my_size += ::protobuf::rt::bytes_size(19, &v);
        };
        if let Some(v) = self.new_name.as_ref() {
            my_size += ::protobuf::rt::string_size(20, &v);
        };
        if let Some(v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.ident.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.protocol_version.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.seq_nr {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.typ {
            os.write_enum(5, v.value())?;
        };
        if let Some(v) = self.device_state.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.goodbye.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.state.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.position {
            os.write_uint32(13, v)?;
        };
        if let Some(v) = self.volume {
            os.write_uint32(14, v)?;
        };
        if let Some(v) = self.state_update_id {
            os.write_int64(17, v)?;
        };
        for v in &self.recipient {
            os.write_string(18, &v)?;
        };
        if let Some(v) = self.context_player_state.as_ref() {
            os.write_bytes(19, &v)?;
        };
        if let Some(v) = self.new_name.as_ref() {
            os.write_string(20, &v)?;
        };
        if let Some(v) = self.metadata.as_ref() {
            os.write_tag(25, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Frame {
    fn new() -> Frame {
        Frame::new()
    }

    fn descriptor_static(_: ::std::option::Option<Frame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    Frame::get_version_for_reflect,
                    Frame::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    Frame::get_ident_for_reflect,
                    Frame::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "protocol_version",
                    Frame::get_protocol_version_for_reflect,
                    Frame::mut_protocol_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_nr",
                    Frame::get_seq_nr_for_reflect,
                    Frame::mut_seq_nr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MessageType>>(
                    "typ",
                    Frame::get_typ_for_reflect,
                    Frame::mut_typ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeviceState>>(
                    "device_state",
                    Frame::get_device_state_for_reflect,
                    Frame::mut_device_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Goodbye>>(
                    "goodbye",
                    Frame::get_goodbye_for_reflect,
                    Frame::mut_goodbye_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<State>>(
                    "state",
                    Frame::get_state_for_reflect,
                    Frame::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "position",
                    Frame::get_position_for_reflect,
                    Frame::mut_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "volume",
                    Frame::get_volume_for_reflect,
                    Frame::mut_volume_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "state_update_id",
                    Frame::get_state_update_id_for_reflect,
                    Frame::mut_state_update_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "recipient",
                    Frame::get_recipient_for_reflect,
                    Frame::mut_recipient_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "context_player_state",
                    Frame::get_context_player_state_for_reflect,
                    Frame::mut_context_player_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "new_name",
                    Frame::get_new_name_for_reflect,
                    Frame::mut_new_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                    "metadata",
                    Frame::get_metadata_for_reflect,
                    Frame::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Frame>(
                    "Frame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Frame {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_ident();
        self.clear_protocol_version();
        self.clear_seq_nr();
        self.clear_typ();
        self.clear_device_state();
        self.clear_goodbye();
        self.clear_state();
        self.clear_position();
        self.clear_volume();
        self.clear_state_update_id();
        self.clear_recipient();
        self.clear_context_player_state();
        self.clear_new_name();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Frame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Frame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeviceState {
    // message fields
    sw_version: ::protobuf::SingularField<::std::string::String>,
    is_active: ::std::option::Option<bool>,
    can_play: ::std::option::Option<bool>,
    volume: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    error_code: ::std::option::Option<u32>,
    became_active_at: ::std::option::Option<i64>,
    error_message: ::protobuf::SingularField<::std::string::String>,
    capabilities: ::protobuf::RepeatedField<Capability>,
    context_player_error: ::protobuf::SingularField<::std::string::String>,
    metadata: ::protobuf::RepeatedField<Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeviceState {}

impl DeviceState {
    pub fn new() -> DeviceState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeviceState {
        static mut instance: ::protobuf::lazy::Lazy<DeviceState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeviceState,
        };
        unsafe {
            instance.get(DeviceState::new)
        }
    }

    // optional string sw_version = 1;

    pub fn clear_sw_version(&mut self) {
        self.sw_version.clear();
    }

    pub fn has_sw_version(&self) -> bool {
        self.sw_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sw_version(&mut self, v: ::std::string::String) {
        self.sw_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sw_version(&mut self) -> &mut ::std::string::String {
        if self.sw_version.is_none() {
            self.sw_version.set_default();
        };
        self.sw_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_sw_version(&mut self) -> ::std::string::String {
        self.sw_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sw_version(&self) -> &str {
        match self.sw_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sw_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sw_version
    }

    fn mut_sw_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sw_version
    }

    // optional bool is_active = 10;

    pub fn clear_is_active(&mut self) {
        self.is_active = ::std::option::Option::None;
    }

    pub fn has_is_active(&self) -> bool {
        self.is_active.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_active(&mut self, v: bool) {
        self.is_active = ::std::option::Option::Some(v);
    }

    pub fn get_is_active(&self) -> bool {
        self.is_active.unwrap_or(false)
    }

    fn get_is_active_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_active
    }

    fn mut_is_active_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_active
    }

    // optional bool can_play = 11;

    pub fn clear_can_play(&mut self) {
        self.can_play = ::std::option::Option::None;
    }

    pub fn has_can_play(&self) -> bool {
        self.can_play.is_some()
    }

    // Param is passed by value, moved
    pub fn set_can_play(&mut self, v: bool) {
        self.can_play = ::std::option::Option::Some(v);
    }

    pub fn get_can_play(&self) -> bool {
        self.can_play.unwrap_or(false)
    }

    fn get_can_play_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.can_play
    }

    fn mut_can_play_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.can_play
    }

    // optional uint32 volume = 12;

    pub fn clear_volume(&mut self) {
        self.volume = ::std::option::Option::None;
    }

    pub fn has_volume(&self) -> bool {
        self.volume.is_some()
    }

    // Param is passed by value, moved
    pub fn set_volume(&mut self, v: u32) {
        self.volume = ::std::option::Option::Some(v);
    }

    pub fn get_volume(&self) -> u32 {
        self.volume.unwrap_or(0)
    }

    fn get_volume_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.volume
    }

    fn mut_volume_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.volume
    }

    // optional string name = 13;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional uint32 error_code = 14;

    pub fn clear_error_code(&mut self) {
        self.error_code = ::std::option::Option::None;
    }

    pub fn has_error_code(&self) -> bool {
        self.error_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_code(&mut self, v: u32) {
        self.error_code = ::std::option::Option::Some(v);
    }

    pub fn get_error_code(&self) -> u32 {
        self.error_code.unwrap_or(0)
    }

    fn get_error_code_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.error_code
    }

    fn mut_error_code_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.error_code
    }

    // optional int64 became_active_at = 15;

    pub fn clear_became_active_at(&mut self) {
        self.became_active_at = ::std::option::Option::None;
    }

    pub fn has_became_active_at(&self) -> bool {
        self.became_active_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_became_active_at(&mut self, v: i64) {
        self.became_active_at = ::std::option::Option::Some(v);
    }

    pub fn get_became_active_at(&self) -> i64 {
        self.became_active_at.unwrap_or(0)
    }

    fn get_became_active_at_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.became_active_at
    }

    fn mut_became_active_at_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.became_active_at
    }

    // optional string error_message = 16;

    pub fn clear_error_message(&mut self) {
        self.error_message.clear();
    }

    pub fn has_error_message(&self) -> bool {
        self.error_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_message(&mut self, v: ::std::string::String) {
        self.error_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_message(&mut self) -> &mut ::std::string::String {
        if self.error_message.is_none() {
            self.error_message.set_default();
        };
        self.error_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_message(&mut self) -> ::std::string::String {
        self.error_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_message(&self) -> &str {
        match self.error_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_message
    }

    fn mut_error_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_message
    }

    // repeated .Capability capabilities = 17;

    pub fn clear_capabilities(&mut self) {
        self.capabilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_capabilities(&mut self, v: ::protobuf::RepeatedField<Capability>) {
        self.capabilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_capabilities(&mut self) -> &mut ::protobuf::RepeatedField<Capability> {
        &mut self.capabilities
    }

    // Take field
    pub fn take_capabilities(&mut self) -> ::protobuf::RepeatedField<Capability> {
        ::std::mem::replace(&mut self.capabilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    fn get_capabilities_for_reflect(&self) -> &::protobuf::RepeatedField<Capability> {
        &self.capabilities
    }

    fn mut_capabilities_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Capability> {
        &mut self.capabilities
    }

    // optional string context_player_error = 20;

    pub fn clear_context_player_error(&mut self) {
        self.context_player_error.clear();
    }

    pub fn has_context_player_error(&self) -> bool {
        self.context_player_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_player_error(&mut self, v: ::std::string::String) {
        self.context_player_error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context_player_error(&mut self) -> &mut ::std::string::String {
        if self.context_player_error.is_none() {
            self.context_player_error.set_default();
        };
        self.context_player_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_context_player_error(&mut self) -> ::std::string::String {
        self.context_player_error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_context_player_error(&self) -> &str {
        match self.context_player_error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_context_player_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.context_player_error
    }

    fn mut_context_player_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.context_player_error
    }

    // repeated .Metadata metadata = 25;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::protobuf::RepeatedField<Metadata>) {
        self.metadata = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut ::protobuf::RepeatedField<Metadata> {
        &mut self.metadata
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::protobuf::RepeatedField<Metadata> {
        ::std::mem::replace(&mut self.metadata, ::protobuf::RepeatedField::new())
    }

    pub fn get_metadata(&self) -> &[Metadata] {
        &self.metadata
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::RepeatedField<Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Metadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for DeviceState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sw_version)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_active = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.can_play = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.volume = ::std::option::Option::Some(tmp);
                },
                13 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.error_code = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.became_active_at = ::std::option::Option::Some(tmp);
                },
                16 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_message)?;
                },
                17 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.capabilities)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.context_player_error)?;
                },
                25 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metadata)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.sw_version.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.is_active {
            my_size += 2;
        };
        if let Some(v) = self.can_play {
            my_size += 2;
        };
        if let Some(v) = self.volume {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(13, &v);
        };
        if let Some(v) = self.error_code {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.became_active_at {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.error_message.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        };
        for value in &self.capabilities {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.context_player_error.as_ref() {
            my_size += ::protobuf::rt::string_size(20, &v);
        };
        for value in &self.metadata {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sw_version.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.is_active {
            os.write_bool(10, v)?;
        };
        if let Some(v) = self.can_play {
            os.write_bool(11, v)?;
        };
        if let Some(v) = self.volume {
            os.write_uint32(12, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(13, &v)?;
        };
        if let Some(v) = self.error_code {
            os.write_uint32(14, v)?;
        };
        if let Some(v) = self.became_active_at {
            os.write_int64(15, v)?;
        };
        if let Some(v) = self.error_message.as_ref() {
            os.write_string(16, &v)?;
        };
        for v in &self.capabilities {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.context_player_error.as_ref() {
            os.write_string(20, &v)?;
        };
        for v in &self.metadata {
            os.write_tag(25, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeviceState {
    fn new() -> DeviceState {
        DeviceState::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeviceState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sw_version",
                    DeviceState::get_sw_version_for_reflect,
                    DeviceState::mut_sw_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_active",
                    DeviceState::get_is_active_for_reflect,
                    DeviceState::mut_is_active_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "can_play",
                    DeviceState::get_can_play_for_reflect,
                    DeviceState::mut_can_play_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "volume",
                    DeviceState::get_volume_for_reflect,
                    DeviceState::mut_volume_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    DeviceState::get_name_for_reflect,
                    DeviceState::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "error_code",
                    DeviceState::get_error_code_for_reflect,
                    DeviceState::mut_error_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "became_active_at",
                    DeviceState::get_became_active_at_for_reflect,
                    DeviceState::mut_became_active_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_message",
                    DeviceState::get_error_message_for_reflect,
                    DeviceState::mut_error_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Capability>>(
                    "capabilities",
                    DeviceState::get_capabilities_for_reflect,
                    DeviceState::mut_capabilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "context_player_error",
                    DeviceState::get_context_player_error_for_reflect,
                    DeviceState::mut_context_player_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                    "metadata",
                    DeviceState::get_metadata_for_reflect,
                    DeviceState::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeviceState>(
                    "DeviceState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeviceState {
    fn clear(&mut self) {
        self.clear_sw_version();
        self.clear_is_active();
        self.clear_can_play();
        self.clear_volume();
        self.clear_name();
        self.clear_error_code();
        self.clear_became_active_at();
        self.clear_error_message();
        self.clear_capabilities();
        self.clear_context_player_error();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeviceState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeviceState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Capability {
    // message fields
    typ: ::std::option::Option<CapabilityType>,
    intValue: ::std::vec::Vec<i64>,
    stringValue: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Capability {}

impl Capability {
    pub fn new() -> Capability {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Capability {
        static mut instance: ::protobuf::lazy::Lazy<Capability> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Capability,
        };
        unsafe {
            instance.get(Capability::new)
        }
    }

    // optional .CapabilityType typ = 1;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: CapabilityType) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> CapabilityType {
        self.typ.unwrap_or(CapabilityType::kSupportedContexts)
    }

    fn get_typ_for_reflect(&self) -> &::std::option::Option<CapabilityType> {
        &self.typ
    }

    fn mut_typ_for_reflect(&mut self) -> &mut ::std::option::Option<CapabilityType> {
        &mut self.typ
    }

    // repeated int64 intValue = 2;

    pub fn clear_intValue(&mut self) {
        self.intValue.clear();
    }

    // Param is passed by value, moved
    pub fn set_intValue(&mut self, v: ::std::vec::Vec<i64>) {
        self.intValue = v;
    }

    // Mutable pointer to the field.
    pub fn mut_intValue(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.intValue
    }

    // Take field
    pub fn take_intValue(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.intValue, ::std::vec::Vec::new())
    }

    pub fn get_intValue(&self) -> &[i64] {
        &self.intValue
    }

    fn get_intValue_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.intValue
    }

    fn mut_intValue_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.intValue
    }

    // repeated string stringValue = 3;

    pub fn clear_stringValue(&mut self) {
        self.stringValue.clear();
    }

    // Param is passed by value, moved
    pub fn set_stringValue(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.stringValue = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stringValue(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.stringValue
    }

    // Take field
    pub fn take_stringValue(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.stringValue, ::protobuf::RepeatedField::new())
    }

    pub fn get_stringValue(&self) -> &[::std::string::String] {
        &self.stringValue
    }

    fn get_stringValue_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.stringValue
    }

    fn mut_stringValue_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.stringValue
    }
}

impl ::protobuf::Message for Capability {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.typ = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.intValue)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.stringValue)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.typ {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        for value in &self.intValue {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.stringValue {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.typ {
            os.write_enum(1, v.value())?;
        };
        for v in &self.intValue {
            os.write_int64(2, *v)?;
        };
        for v in &self.stringValue {
            os.write_string(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Capability {
    fn new() -> Capability {
        Capability::new()
    }

    fn descriptor_static(_: ::std::option::Option<Capability>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CapabilityType>>(
                    "typ",
                    Capability::get_typ_for_reflect,
                    Capability::mut_typ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "intValue",
                    Capability::get_intValue_for_reflect,
                    Capability::mut_intValue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "stringValue",
                    Capability::get_stringValue_for_reflect,
                    Capability::mut_stringValue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Capability>(
                    "Capability",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Capability {
    fn clear(&mut self) {
        self.clear_typ();
        self.clear_intValue();
        self.clear_stringValue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Capability {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Capability {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Goodbye {
    // message fields
    reason: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Goodbye {}

impl Goodbye {
    pub fn new() -> Goodbye {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Goodbye {
        static mut instance: ::protobuf::lazy::Lazy<Goodbye> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Goodbye,
        };
        unsafe {
            instance.get(Goodbye::new)
        }
    }

    // optional string reason = 1;

    pub fn clear_reason(&mut self) {
        self.reason.clear();
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&mut self) -> &mut ::std::string::String {
        if self.reason.is_none() {
            self.reason.set_default();
        };
        self.reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_reason(&mut self) -> ::std::string::String {
        self.reason.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_reason(&self) -> &str {
        match self.reason.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_reason_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.reason
    }
}

impl ::protobuf::Message for Goodbye {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.reason)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reason.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Goodbye {
    fn new() -> Goodbye {
        Goodbye::new()
    }

    fn descriptor_static(_: ::std::option::Option<Goodbye>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    Goodbye::get_reason_for_reflect,
                    Goodbye::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Goodbye>(
                    "Goodbye",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Goodbye {
    fn clear(&mut self) {
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Goodbye {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Goodbye {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct State {
    // message fields
    context_uri: ::protobuf::SingularField<::std::string::String>,
    index: ::std::option::Option<u32>,
    position_ms: ::std::option::Option<u32>,
    status: ::std::option::Option<PlayStatus>,
    position_measured_at: ::std::option::Option<u64>,
    context_description: ::protobuf::SingularField<::std::string::String>,
    shuffle: ::std::option::Option<bool>,
    repeat: ::std::option::Option<bool>,
    last_command_ident: ::protobuf::SingularField<::std::string::String>,
    last_command_msgid: ::std::option::Option<u32>,
    playing_from_fallback: ::std::option::Option<bool>,
    row: ::std::option::Option<u32>,
    playing_track_index: ::std::option::Option<u32>,
    track: ::protobuf::RepeatedField<TrackRef>,
    ad: ::protobuf::SingularPtrField<Ad>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for State {}

impl State {
    pub fn new() -> State {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static State {
        static mut instance: ::protobuf::lazy::Lazy<State> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const State,
        };
        unsafe {
            instance.get(State::new)
        }
    }

    // optional string context_uri = 2;

    pub fn clear_context_uri(&mut self) {
        self.context_uri.clear();
    }

    pub fn has_context_uri(&self) -> bool {
        self.context_uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_uri(&mut self, v: ::std::string::String) {
        self.context_uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context_uri(&mut self) -> &mut ::std::string::String {
        if self.context_uri.is_none() {
            self.context_uri.set_default();
        };
        self.context_uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_context_uri(&mut self) -> ::std::string::String {
        self.context_uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_context_uri(&self) -> &str {
        match self.context_uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_context_uri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.context_uri
    }

    fn mut_context_uri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.context_uri
    }

    // optional uint32 index = 3;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.index
    }

    // optional uint32 position_ms = 4;

    pub fn clear_position_ms(&mut self) {
        self.position_ms = ::std::option::Option::None;
    }

    pub fn has_position_ms(&self) -> bool {
        self.position_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position_ms(&mut self, v: u32) {
        self.position_ms = ::std::option::Option::Some(v);
    }

    pub fn get_position_ms(&self) -> u32 {
        self.position_ms.unwrap_or(0)
    }

    fn get_position_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.position_ms
    }

    fn mut_position_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.position_ms
    }

    // optional .PlayStatus status = 5;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: PlayStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> PlayStatus {
        self.status.unwrap_or(PlayStatus::kPlayStatusStop)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<PlayStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<PlayStatus> {
        &mut self.status
    }

    // optional uint64 position_measured_at = 7;

    pub fn clear_position_measured_at(&mut self) {
        self.position_measured_at = ::std::option::Option::None;
    }

    pub fn has_position_measured_at(&self) -> bool {
        self.position_measured_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position_measured_at(&mut self, v: u64) {
        self.position_measured_at = ::std::option::Option::Some(v);
    }

    pub fn get_position_measured_at(&self) -> u64 {
        self.position_measured_at.unwrap_or(0)
    }

    fn get_position_measured_at_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.position_measured_at
    }

    fn mut_position_measured_at_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.position_measured_at
    }

    // optional string context_description = 8;

    pub fn clear_context_description(&mut self) {
        self.context_description.clear();
    }

    pub fn has_context_description(&self) -> bool {
        self.context_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_description(&mut self, v: ::std::string::String) {
        self.context_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context_description(&mut self) -> &mut ::std::string::String {
        if self.context_description.is_none() {
            self.context_description.set_default();
        };
        self.context_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_context_description(&mut self) -> ::std::string::String {
        self.context_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_context_description(&self) -> &str {
        match self.context_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_context_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.context_description
    }

    fn mut_context_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.context_description
    }

    // optional bool shuffle = 13;

    pub fn clear_shuffle(&mut self) {
        self.shuffle = ::std::option::Option::None;
    }

    pub fn has_shuffle(&self) -> bool {
        self.shuffle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shuffle(&mut self, v: bool) {
        self.shuffle = ::std::option::Option::Some(v);
    }

    pub fn get_shuffle(&self) -> bool {
        self.shuffle.unwrap_or(false)
    }

    fn get_shuffle_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.shuffle
    }

    fn mut_shuffle_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.shuffle
    }

    // optional bool repeat = 14;

    pub fn clear_repeat(&mut self) {
        self.repeat = ::std::option::Option::None;
    }

    pub fn has_repeat(&self) -> bool {
        self.repeat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_repeat(&mut self, v: bool) {
        self.repeat = ::std::option::Option::Some(v);
    }

    pub fn get_repeat(&self) -> bool {
        self.repeat.unwrap_or(false)
    }

    fn get_repeat_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.repeat
    }

    fn mut_repeat_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.repeat
    }

    // optional string last_command_ident = 20;

    pub fn clear_last_command_ident(&mut self) {
        self.last_command_ident.clear();
    }

    pub fn has_last_command_ident(&self) -> bool {
        self.last_command_ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_command_ident(&mut self, v: ::std::string::String) {
        self.last_command_ident = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_command_ident(&mut self) -> &mut ::std::string::String {
        if self.last_command_ident.is_none() {
            self.last_command_ident.set_default();
        };
        self.last_command_ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_command_ident(&mut self) -> ::std::string::String {
        self.last_command_ident.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_last_command_ident(&self) -> &str {
        match self.last_command_ident.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_last_command_ident_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.last_command_ident
    }

    fn mut_last_command_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.last_command_ident
    }

    // optional uint32 last_command_msgid = 21;

    pub fn clear_last_command_msgid(&mut self) {
        self.last_command_msgid = ::std::option::Option::None;
    }

    pub fn has_last_command_msgid(&self) -> bool {
        self.last_command_msgid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_command_msgid(&mut self, v: u32) {
        self.last_command_msgid = ::std::option::Option::Some(v);
    }

    pub fn get_last_command_msgid(&self) -> u32 {
        self.last_command_msgid.unwrap_or(0)
    }

    fn get_last_command_msgid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.last_command_msgid
    }

    fn mut_last_command_msgid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.last_command_msgid
    }

    // optional bool playing_from_fallback = 24;

    pub fn clear_playing_from_fallback(&mut self) {
        self.playing_from_fallback = ::std::option::Option::None;
    }

    pub fn has_playing_from_fallback(&self) -> bool {
        self.playing_from_fallback.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playing_from_fallback(&mut self, v: bool) {
        self.playing_from_fallback = ::std::option::Option::Some(v);
    }

    pub fn get_playing_from_fallback(&self) -> bool {
        self.playing_from_fallback.unwrap_or(false)
    }

    fn get_playing_from_fallback_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.playing_from_fallback
    }

    fn mut_playing_from_fallback_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.playing_from_fallback
    }

    // optional uint32 row = 25;

    pub fn clear_row(&mut self) {
        self.row = ::std::option::Option::None;
    }

    pub fn has_row(&self) -> bool {
        self.row.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row(&mut self, v: u32) {
        self.row = ::std::option::Option::Some(v);
    }

    pub fn get_row(&self) -> u32 {
        self.row.unwrap_or(0)
    }

    fn get_row_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.row
    }

    fn mut_row_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.row
    }

    // optional uint32 playing_track_index = 26;

    pub fn clear_playing_track_index(&mut self) {
        self.playing_track_index = ::std::option::Option::None;
    }

    pub fn has_playing_track_index(&self) -> bool {
        self.playing_track_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playing_track_index(&mut self, v: u32) {
        self.playing_track_index = ::std::option::Option::Some(v);
    }

    pub fn get_playing_track_index(&self) -> u32 {
        self.playing_track_index.unwrap_or(0)
    }

    fn get_playing_track_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.playing_track_index
    }

    fn mut_playing_track_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.playing_track_index
    }

    // repeated .TrackRef track = 27;

    pub fn clear_track(&mut self) {
        self.track.clear();
    }

    // Param is passed by value, moved
    pub fn set_track(&mut self, v: ::protobuf::RepeatedField<TrackRef>) {
        self.track = v;
    }

    // Mutable pointer to the field.
    pub fn mut_track(&mut self) -> &mut ::protobuf::RepeatedField<TrackRef> {
        &mut self.track
    }

    // Take field
    pub fn take_track(&mut self) -> ::protobuf::RepeatedField<TrackRef> {
        ::std::mem::replace(&mut self.track, ::protobuf::RepeatedField::new())
    }

    pub fn get_track(&self) -> &[TrackRef] {
        &self.track
    }

    fn get_track_for_reflect(&self) -> &::protobuf::RepeatedField<TrackRef> {
        &self.track
    }

    fn mut_track_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TrackRef> {
        &mut self.track
    }

    // optional .Ad ad = 28;

    pub fn clear_ad(&mut self) {
        self.ad.clear();
    }

    pub fn has_ad(&self) -> bool {
        self.ad.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ad(&mut self, v: Ad) {
        self.ad = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ad(&mut self) -> &mut Ad {
        if self.ad.is_none() {
            self.ad.set_default();
        };
        self.ad.as_mut().unwrap()
    }

    // Take field
    pub fn take_ad(&mut self) -> Ad {
        self.ad.take().unwrap_or_else(|| Ad::new())
    }

    pub fn get_ad(&self) -> &Ad {
        self.ad.as_ref().unwrap_or_else(|| Ad::default_instance())
    }

    fn get_ad_for_reflect(&self) -> &::protobuf::SingularPtrField<Ad> {
        &self.ad
    }

    fn mut_ad_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Ad> {
        &mut self.ad
    }
}

impl ::protobuf::Message for State {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.context_uri)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.position_ms = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.position_measured_at = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.context_description)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.shuffle = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.repeat = ::std::option::Option::Some(tmp);
                },
                20 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.last_command_ident)?;
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.last_command_msgid = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.playing_from_fallback = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.row = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.playing_track_index = ::std::option::Option::Some(tmp);
                },
                27 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.track)?;
                },
                28 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ad)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.context_uri.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.position_ms {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(5, v);
        };
        if let Some(v) = self.position_measured_at {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.context_description.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        };
        if let Some(v) = self.shuffle {
            my_size += 2;
        };
        if let Some(v) = self.repeat {
            my_size += 2;
        };
        if let Some(v) = self.last_command_ident.as_ref() {
            my_size += ::protobuf::rt::string_size(20, &v);
        };
        if let Some(v) = self.last_command_msgid {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.playing_from_fallback {
            my_size += 3;
        };
        if let Some(v) = self.row {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.playing_track_index {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.track {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.ad.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.context_uri.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.index {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.position_ms {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.status {
            os.write_enum(5, v.value())?;
        };
        if let Some(v) = self.position_measured_at {
            os.write_uint64(7, v)?;
        };
        if let Some(v) = self.context_description.as_ref() {
            os.write_string(8, &v)?;
        };
        if let Some(v) = self.shuffle {
            os.write_bool(13, v)?;
        };
        if let Some(v) = self.repeat {
            os.write_bool(14, v)?;
        };
        if let Some(v) = self.last_command_ident.as_ref() {
            os.write_string(20, &v)?;
        };
        if let Some(v) = self.last_command_msgid {
            os.write_uint32(21, v)?;
        };
        if let Some(v) = self.playing_from_fallback {
            os.write_bool(24, v)?;
        };
        if let Some(v) = self.row {
            os.write_uint32(25, v)?;
        };
        if let Some(v) = self.playing_track_index {
            os.write_uint32(26, v)?;
        };
        for v in &self.track {
            os.write_tag(27, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.ad.as_ref() {
            os.write_tag(28, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for State {
    fn new() -> State {
        State::new()
    }

    fn descriptor_static(_: ::std::option::Option<State>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "context_uri",
                    State::get_context_uri_for_reflect,
                    State::mut_context_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    State::get_index_for_reflect,
                    State::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "position_ms",
                    State::get_position_ms_for_reflect,
                    State::mut_position_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PlayStatus>>(
                    "status",
                    State::get_status_for_reflect,
                    State::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "position_measured_at",
                    State::get_position_measured_at_for_reflect,
                    State::mut_position_measured_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "context_description",
                    State::get_context_description_for_reflect,
                    State::mut_context_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "shuffle",
                    State::get_shuffle_for_reflect,
                    State::mut_shuffle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "repeat",
                    State::get_repeat_for_reflect,
                    State::mut_repeat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "last_command_ident",
                    State::get_last_command_ident_for_reflect,
                    State::mut_last_command_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "last_command_msgid",
                    State::get_last_command_msgid_for_reflect,
                    State::mut_last_command_msgid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "playing_from_fallback",
                    State::get_playing_from_fallback_for_reflect,
                    State::mut_playing_from_fallback_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "row",
                    State::get_row_for_reflect,
                    State::mut_row_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "playing_track_index",
                    State::get_playing_track_index_for_reflect,
                    State::mut_playing_track_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TrackRef>>(
                    "track",
                    State::get_track_for_reflect,
                    State::mut_track_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Ad>>(
                    "ad",
                    State::get_ad_for_reflect,
                    State::mut_ad_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<State>(
                    "State",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for State {
    fn clear(&mut self) {
        self.clear_context_uri();
        self.clear_index();
        self.clear_position_ms();
        self.clear_status();
        self.clear_position_measured_at();
        self.clear_context_description();
        self.clear_shuffle();
        self.clear_repeat();
        self.clear_last_command_ident();
        self.clear_last_command_msgid();
        self.clear_playing_from_fallback();
        self.clear_row();
        self.clear_playing_track_index();
        self.clear_track();
        self.clear_ad();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for State {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for State {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TrackRef {
    // message fields
    gid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    uri: ::protobuf::SingularField<::std::string::String>,
    queued: ::std::option::Option<bool>,
    context: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TrackRef {}

impl TrackRef {
    pub fn new() -> TrackRef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TrackRef {
        static mut instance: ::protobuf::lazy::Lazy<TrackRef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TrackRef,
        };
        unsafe {
            instance.get(TrackRef::new)
        }
    }

    // optional bytes gid = 1;

    pub fn clear_gid(&mut self) {
        self.gid.clear();
    }

    pub fn has_gid(&self) -> bool {
        self.gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gid(&mut self, v: ::std::vec::Vec<u8>) {
        self.gid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gid.is_none() {
            self.gid.set_default();
        };
        self.gid.as_mut().unwrap()
    }

    // Take field
    pub fn take_gid(&mut self) -> ::std::vec::Vec<u8> {
        self.gid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gid(&self) -> &[u8] {
        match self.gid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gid
    }

    fn mut_gid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gid
    }

    // optional string uri = 2;

    pub fn clear_uri(&mut self) {
        self.uri.clear();
    }

    pub fn has_uri(&self) -> bool {
        self.uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uri(&mut self) -> &mut ::std::string::String {
        if self.uri.is_none() {
            self.uri.set_default();
        };
        self.uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_uri(&mut self) -> ::std::string::String {
        self.uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_uri(&self) -> &str {
        match self.uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_uri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.uri
    }

    fn mut_uri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.uri
    }

    // optional bool queued = 3;

    pub fn clear_queued(&mut self) {
        self.queued = ::std::option::Option::None;
    }

    pub fn has_queued(&self) -> bool {
        self.queued.is_some()
    }

    // Param is passed by value, moved
    pub fn set_queued(&mut self, v: bool) {
        self.queued = ::std::option::Option::Some(v);
    }

    pub fn get_queued(&self) -> bool {
        self.queued.unwrap_or(false)
    }

    fn get_queued_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.queued
    }

    fn mut_queued_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.queued
    }

    // optional string context = 4;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: ::std::string::String) {
        self.context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut ::std::string::String {
        if self.context.is_none() {
            self.context.set_default();
        };
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> ::std::string::String {
        self.context.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_context(&self) -> &str {
        match self.context.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.context
    }
}

impl ::protobuf::Message for TrackRef {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uri)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.queued = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.context)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.gid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.uri.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.queued {
            my_size += 2;
        };
        if let Some(v) = self.context.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gid.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.uri.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.queued {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.context.as_ref() {
            os.write_string(4, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TrackRef {
    fn new() -> TrackRef {
        TrackRef::new()
    }

    fn descriptor_static(_: ::std::option::Option<TrackRef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gid",
                    TrackRef::get_gid_for_reflect,
                    TrackRef::mut_gid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uri",
                    TrackRef::get_uri_for_reflect,
                    TrackRef::mut_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "queued",
                    TrackRef::get_queued_for_reflect,
                    TrackRef::mut_queued_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "context",
                    TrackRef::get_context_for_reflect,
                    TrackRef::mut_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TrackRef>(
                    "TrackRef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TrackRef {
    fn clear(&mut self) {
        self.clear_gid();
        self.clear_uri();
        self.clear_queued();
        self.clear_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TrackRef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrackRef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Ad {
    // message fields
    next: ::std::option::Option<i32>,
    ogg_fid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    image_fid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    duration: ::std::option::Option<i32>,
    click_url: ::protobuf::SingularField<::std::string::String>,
    impression_url: ::protobuf::SingularField<::std::string::String>,
    product: ::protobuf::SingularField<::std::string::String>,
    advertiser: ::protobuf::SingularField<::std::string::String>,
    gid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Ad {}

impl Ad {
    pub fn new() -> Ad {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ad {
        static mut instance: ::protobuf::lazy::Lazy<Ad> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ad,
        };
        unsafe {
            instance.get(Ad::new)
        }
    }

    // optional int32 next = 1;

    pub fn clear_next(&mut self) {
        self.next = ::std::option::Option::None;
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next(&mut self, v: i32) {
        self.next = ::std::option::Option::Some(v);
    }

    pub fn get_next(&self) -> i32 {
        self.next.unwrap_or(0)
    }

    fn get_next_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.next
    }

    fn mut_next_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.next
    }

    // optional bytes ogg_fid = 2;

    pub fn clear_ogg_fid(&mut self) {
        self.ogg_fid.clear();
    }

    pub fn has_ogg_fid(&self) -> bool {
        self.ogg_fid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ogg_fid(&mut self, v: ::std::vec::Vec<u8>) {
        self.ogg_fid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ogg_fid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ogg_fid.is_none() {
            self.ogg_fid.set_default();
        };
        self.ogg_fid.as_mut().unwrap()
    }

    // Take field
    pub fn take_ogg_fid(&mut self) -> ::std::vec::Vec<u8> {
        self.ogg_fid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ogg_fid(&self) -> &[u8] {
        match self.ogg_fid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_ogg_fid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.ogg_fid
    }

    fn mut_ogg_fid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.ogg_fid
    }

    // optional bytes image_fid = 3;

    pub fn clear_image_fid(&mut self) {
        self.image_fid.clear();
    }

    pub fn has_image_fid(&self) -> bool {
        self.image_fid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image_fid(&mut self, v: ::std::vec::Vec<u8>) {
        self.image_fid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image_fid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.image_fid.is_none() {
            self.image_fid.set_default();
        };
        self.image_fid.as_mut().unwrap()
    }

    // Take field
    pub fn take_image_fid(&mut self) -> ::std::vec::Vec<u8> {
        self.image_fid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_image_fid(&self) -> &[u8] {
        match self.image_fid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_image_fid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.image_fid
    }

    fn mut_image_fid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.image_fid
    }

    // optional int32 duration = 4;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: i32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> i32 {
        self.duration.unwrap_or(0)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.duration
    }

    // optional string click_url = 5;

    pub fn clear_click_url(&mut self) {
        self.click_url.clear();
    }

    pub fn has_click_url(&self) -> bool {
        self.click_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_click_url(&mut self, v: ::std::string::String) {
        self.click_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_click_url(&mut self) -> &mut ::std::string::String {
        if self.click_url.is_none() {
            self.click_url.set_default();
        };
        self.click_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_click_url(&mut self) -> ::std::string::String {
        self.click_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_click_url(&self) -> &str {
        match self.click_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_click_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.click_url
    }

    fn mut_click_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.click_url
    }

    // optional string impression_url = 6;

    pub fn clear_impression_url(&mut self) {
        self.impression_url.clear();
    }

    pub fn has_impression_url(&self) -> bool {
        self.impression_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_impression_url(&mut self, v: ::std::string::String) {
        self.impression_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_impression_url(&mut self) -> &mut ::std::string::String {
        if self.impression_url.is_none() {
            self.impression_url.set_default();
        };
        self.impression_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_impression_url(&mut self) -> ::std::string::String {
        self.impression_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_impression_url(&self) -> &str {
        match self.impression_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_impression_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.impression_url
    }

    fn mut_impression_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.impression_url
    }

    // optional string product = 7;

    pub fn clear_product(&mut self) {
        self.product.clear();
    }

    pub fn has_product(&self) -> bool {
        self.product.is_some()
    }

    // Param is passed by value, moved
    pub fn set_product(&mut self, v: ::std::string::String) {
        self.product = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_product(&mut self) -> &mut ::std::string::String {
        if self.product.is_none() {
            self.product.set_default();
        };
        self.product.as_mut().unwrap()
    }

    // Take field
    pub fn take_product(&mut self) -> ::std::string::String {
        self.product.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_product(&self) -> &str {
        match self.product.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_product_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.product
    }

    fn mut_product_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.product
    }

    // optional string advertiser = 8;

    pub fn clear_advertiser(&mut self) {
        self.advertiser.clear();
    }

    pub fn has_advertiser(&self) -> bool {
        self.advertiser.is_some()
    }

    // Param is passed by value, moved
    pub fn set_advertiser(&mut self, v: ::std::string::String) {
        self.advertiser = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_advertiser(&mut self) -> &mut ::std::string::String {
        if self.advertiser.is_none() {
            self.advertiser.set_default();
        };
        self.advertiser.as_mut().unwrap()
    }

    // Take field
    pub fn take_advertiser(&mut self) -> ::std::string::String {
        self.advertiser.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_advertiser(&self) -> &str {
        match self.advertiser.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_advertiser_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.advertiser
    }

    fn mut_advertiser_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.advertiser
    }

    // optional bytes gid = 9;

    pub fn clear_gid(&mut self) {
        self.gid.clear();
    }

    pub fn has_gid(&self) -> bool {
        self.gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gid(&mut self, v: ::std::vec::Vec<u8>) {
        self.gid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gid.is_none() {
            self.gid.set_default();
        };
        self.gid.as_mut().unwrap()
    }

    // Take field
    pub fn take_gid(&mut self) -> ::std::vec::Vec<u8> {
        self.gid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gid(&self) -> &[u8] {
        match self.gid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gid
    }

    fn mut_gid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gid
    }
}

impl ::protobuf::Message for Ad {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.next = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ogg_fid)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.image_fid)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.click_url)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.impression_url)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.product)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.advertiser)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gid)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.next {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ogg_fid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.image_fid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.duration {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.click_url.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.impression_url.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.product.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.advertiser.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        };
        if let Some(v) = self.gid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(9, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.next {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.ogg_fid.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.image_fid.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.duration {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.click_url.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.impression_url.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.product.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.advertiser.as_ref() {
            os.write_string(8, &v)?;
        };
        if let Some(v) = self.gid.as_ref() {
            os.write_bytes(9, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Ad {
    fn new() -> Ad {
        Ad::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ad>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "next",
                    Ad::get_next_for_reflect,
                    Ad::mut_next_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ogg_fid",
                    Ad::get_ogg_fid_for_reflect,
                    Ad::mut_ogg_fid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "image_fid",
                    Ad::get_image_fid_for_reflect,
                    Ad::mut_image_fid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "duration",
                    Ad::get_duration_for_reflect,
                    Ad::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "click_url",
                    Ad::get_click_url_for_reflect,
                    Ad::mut_click_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "impression_url",
                    Ad::get_impression_url_for_reflect,
                    Ad::mut_impression_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "product",
                    Ad::get_product_for_reflect,
                    Ad::mut_product_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "advertiser",
                    Ad::get_advertiser_for_reflect,
                    Ad::mut_advertiser_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gid",
                    Ad::get_gid_for_reflect,
                    Ad::mut_gid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Ad>(
                    "Ad",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ad {
    fn clear(&mut self) {
        self.clear_next();
        self.clear_ogg_fid();
        self.clear_image_fid();
        self.clear_duration();
        self.clear_click_url();
        self.clear_impression_url();
        self.clear_product();
        self.clear_advertiser();
        self.clear_gid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ad {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ad {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Metadata {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    metadata: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Metadata {}

impl Metadata {
    pub fn new() -> Metadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Metadata {
        static mut instance: ::protobuf::lazy::Lazy<Metadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Metadata,
        };
        unsafe {
            instance.get(Metadata::new)
        }
    }

    // optional string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.field_type
    }

    // optional string metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::std::string::String) {
        self.metadata = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut ::std::string::String {
        if self.metadata.is_none() {
            self.metadata.set_default();
        };
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::std::string::String {
        self.metadata.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_metadata(&self) -> &str {
        match self.metadata.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for Metadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.metadata)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.metadata.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.metadata.as_ref() {
            os.write_string(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Metadata {
    fn new() -> Metadata {
        Metadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<Metadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    Metadata::get_field_type_for_reflect,
                    Metadata::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "metadata",
                    Metadata::get_metadata_for_reflect,
                    Metadata::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Metadata>(
                    "Metadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Metadata {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    kMessageTypeHello = 1,
    kMessageTypeGoodbye = 2,
    kMessageTypeProbe = 3,
    kMessageTypeNotify = 10,
    kMessageTypeLoad = 20,
    kMessageTypePlay = 21,
    kMessageTypePause = 22,
    kMessageTypePlayPause = 23,
    kMessageTypeSeek = 24,
    kMessageTypePrev = 25,
    kMessageTypeNext = 26,
    kMessageTypeVolume = 27,
    kMessageTypeShuffle = 28,
    kMessageTypeRepeat = 29,
    kMessageTypeVolumeDown = 31,
    kMessageTypeVolumeUp = 32,
    kMessageTypeReplace = 33,
    kMessageTypeLogout = 34,
    kMessageTypeAction = 35,
    kMessageTypeRename = 36,
    kMessageTypeUpdateMetadata = 128,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            1 => ::std::option::Option::Some(MessageType::kMessageTypeHello),
            2 => ::std::option::Option::Some(MessageType::kMessageTypeGoodbye),
            3 => ::std::option::Option::Some(MessageType::kMessageTypeProbe),
            10 => ::std::option::Option::Some(MessageType::kMessageTypeNotify),
            20 => ::std::option::Option::Some(MessageType::kMessageTypeLoad),
            21 => ::std::option::Option::Some(MessageType::kMessageTypePlay),
            22 => ::std::option::Option::Some(MessageType::kMessageTypePause),
            23 => ::std::option::Option::Some(MessageType::kMessageTypePlayPause),
            24 => ::std::option::Option::Some(MessageType::kMessageTypeSeek),
            25 => ::std::option::Option::Some(MessageType::kMessageTypePrev),
            26 => ::std::option::Option::Some(MessageType::kMessageTypeNext),
            27 => ::std::option::Option::Some(MessageType::kMessageTypeVolume),
            28 => ::std::option::Option::Some(MessageType::kMessageTypeShuffle),
            29 => ::std::option::Option::Some(MessageType::kMessageTypeRepeat),
            31 => ::std::option::Option::Some(MessageType::kMessageTypeVolumeDown),
            32 => ::std::option::Option::Some(MessageType::kMessageTypeVolumeUp),
            33 => ::std::option::Option::Some(MessageType::kMessageTypeReplace),
            34 => ::std::option::Option::Some(MessageType::kMessageTypeLogout),
            35 => ::std::option::Option::Some(MessageType::kMessageTypeAction),
            36 => ::std::option::Option::Some(MessageType::kMessageTypeRename),
            128 => ::std::option::Option::Some(MessageType::kMessageTypeUpdateMetadata),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] = &[
            MessageType::kMessageTypeHello,
            MessageType::kMessageTypeGoodbye,
            MessageType::kMessageTypeProbe,
            MessageType::kMessageTypeNotify,
            MessageType::kMessageTypeLoad,
            MessageType::kMessageTypePlay,
            MessageType::kMessageTypePause,
            MessageType::kMessageTypePlayPause,
            MessageType::kMessageTypeSeek,
            MessageType::kMessageTypePrev,
            MessageType::kMessageTypeNext,
            MessageType::kMessageTypeVolume,
            MessageType::kMessageTypeShuffle,
            MessageType::kMessageTypeRepeat,
            MessageType::kMessageTypeVolumeDown,
            MessageType::kMessageTypeVolumeUp,
            MessageType::kMessageTypeReplace,
            MessageType::kMessageTypeLogout,
            MessageType::kMessageTypeAction,
            MessageType::kMessageTypeRename,
            MessageType::kMessageTypeUpdateMetadata,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageType {
}

impl ::protobuf::reflect::ProtobufValue for MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CapabilityType {
    kSupportedContexts = 1,
    kCanBePlayer = 2,
    kRestrictToLocal = 3,
    kDeviceType = 4,
    kGaiaEqConnectId = 5,
    kSupportsLogout = 6,
    kIsObservable = 7,
    kVolumeSteps = 8,
    kSupportedTypes = 9,
    kCommandAcks = 10,
    kSupportsRename = 11,
    kHidden = 12,
    kSupportsPlaylistV2 = 13,
    kUnknown = 14,
}

impl ::protobuf::ProtobufEnum for CapabilityType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CapabilityType> {
        match value {
            1 => ::std::option::Option::Some(CapabilityType::kSupportedContexts),
            2 => ::std::option::Option::Some(CapabilityType::kCanBePlayer),
            3 => ::std::option::Option::Some(CapabilityType::kRestrictToLocal),
            4 => ::std::option::Option::Some(CapabilityType::kDeviceType),
            5 => ::std::option::Option::Some(CapabilityType::kGaiaEqConnectId),
            6 => ::std::option::Option::Some(CapabilityType::kSupportsLogout),
            7 => ::std::option::Option::Some(CapabilityType::kIsObservable),
            8 => ::std::option::Option::Some(CapabilityType::kVolumeSteps),
            9 => ::std::option::Option::Some(CapabilityType::kSupportedTypes),
            10 => ::std::option::Option::Some(CapabilityType::kCommandAcks),
            11 => ::std::option::Option::Some(CapabilityType::kSupportsRename),
            12 => ::std::option::Option::Some(CapabilityType::kHidden),
            13 => ::std::option::Option::Some(CapabilityType::kSupportsPlaylistV2),
            14 => ::std::option::Option::Some(CapabilityType::kUnknown),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CapabilityType] = &[
            CapabilityType::kSupportedContexts,
            CapabilityType::kCanBePlayer,
            CapabilityType::kRestrictToLocal,
            CapabilityType::kDeviceType,
            CapabilityType::kGaiaEqConnectId,
            CapabilityType::kSupportsLogout,
            CapabilityType::kIsObservable,
            CapabilityType::kVolumeSteps,
            CapabilityType::kSupportedTypes,
            CapabilityType::kCommandAcks,
            CapabilityType::kSupportsRename,
            CapabilityType::kHidden,
            CapabilityType::kSupportsPlaylistV2,
            CapabilityType::kUnknown,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CapabilityType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CapabilityType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CapabilityType {
}

impl ::protobuf::reflect::ProtobufValue for CapabilityType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PlayStatus {
    kPlayStatusStop = 0,
    kPlayStatusPlay = 1,
    kPlayStatusPause = 2,
    kPlayStatusLoading = 3,
}

impl ::protobuf::ProtobufEnum for PlayStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PlayStatus> {
        match value {
            0 => ::std::option::Option::Some(PlayStatus::kPlayStatusStop),
            1 => ::std::option::Option::Some(PlayStatus::kPlayStatusPlay),
            2 => ::std::option::Option::Some(PlayStatus::kPlayStatusPause),
            3 => ::std::option::Option::Some(PlayStatus::kPlayStatusLoading),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PlayStatus] = &[
            PlayStatus::kPlayStatusStop,
            PlayStatus::kPlayStatusPlay,
            PlayStatus::kPlayStatusPause,
            PlayStatus::kPlayStatusLoading,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<PlayStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PlayStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PlayStatus {
}

impl ::protobuf::reflect::ProtobufValue for PlayStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x73, 0x70, 0x69, 0x72, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xfa, 0x03,
    0x0a, 0x05, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x05, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x56, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x12, 0x15, 0x0a, 0x06, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x72, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x05, 0x73, 0x65, 0x71, 0x4e, 0x72, 0x12, 0x1e, 0x0a, 0x03, 0x74, 0x79, 0x70,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0c, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x54, 0x79, 0x70, 0x65, 0x52, 0x03, 0x74, 0x79, 0x70, 0x12, 0x2f, 0x0a, 0x0c, 0x64, 0x65, 0x76,
    0x69, 0x63, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0c, 0x2e, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x0b, 0x64,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x22, 0x0a, 0x07, 0x67, 0x6f,
    0x6f, 0x64, 0x62, 0x79, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x47, 0x6f,
    0x6f, 0x64, 0x62, 0x79, 0x65, 0x52, 0x07, 0x67, 0x6f, 0x6f, 0x64, 0x62, 0x79, 0x65, 0x12, 0x1c,
    0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x12, 0x1a, 0x0a, 0x08,
    0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08,
    0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x76, 0x6f, 0x6c, 0x75,
    0x6d, 0x65, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65,
    0x12, 0x26, 0x0a, 0x0f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x11, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0d, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x49, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x72, 0x65, 0x63, 0x69,
    0x70, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x12, 0x20, 0x03, 0x28, 0x09, 0x52, 0x09, 0x72, 0x65, 0x63,
    0x69, 0x70, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x30, 0x0a, 0x14, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78,
    0x74, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x13,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x12, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x50, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x6e, 0x65, 0x77, 0x5f,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x14, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6e, 0x65, 0x77, 0x4e,
    0x61, 0x6d, 0x65, 0x12, 0x25, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x18,
    0x19, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61,
    0x52, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0x88, 0x03, 0x0a, 0x0b, 0x44,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x77,
    0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
    0x73, 0x77, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x73, 0x5f,
    0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x69, 0x73,
    0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x63, 0x61, 0x6e, 0x5f, 0x70, 0x6c,
    0x61, 0x79, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x63, 0x61, 0x6e, 0x50, 0x6c, 0x61,
    0x79, 0x12, 0x16, 0x0a, 0x06, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x06, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a,
    0x0a, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x0e, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x09, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x28, 0x0a, 0x10,
    0x62, 0x65, 0x63, 0x61, 0x6d, 0x65, 0x5f, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x61, 0x74,
    0x18, 0x0f, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0e, 0x62, 0x65, 0x63, 0x61, 0x6d, 0x65, 0x41, 0x63,
    0x74, 0x69, 0x76, 0x65, 0x41, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2f, 0x0a, 0x0c, 0x63,
    0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x69, 0x65, 0x73, 0x18, 0x11, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x0c,
    0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x69, 0x65, 0x73, 0x12, 0x30, 0x0a, 0x14,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x18, 0x14, 0x20, 0x01, 0x28, 0x09, 0x52, 0x12, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x78, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x25,
    0x0a, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x18, 0x19, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x09, 0x2e, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x52, 0x08, 0x6d, 0x65, 0x74,
    0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0x6d, 0x0a, 0x0a, 0x43, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c,
    0x69, 0x74, 0x79, 0x12, 0x21, 0x0a, 0x03, 0x74, 0x79, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x0f, 0x2e, 0x43, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70,
    0x65, 0x52, 0x03, 0x74, 0x79, 0x70, 0x12, 0x1a, 0x0a, 0x08, 0x69, 0x6e, 0x74, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x03, 0x52, 0x08, 0x69, 0x6e, 0x74, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x22, 0x21, 0x0a, 0x07, 0x47, 0x6f, 0x6f, 0x64, 0x62, 0x79, 0x65, 0x12,
    0x16, 0x0a, 0x06, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x06, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x22, 0xa1, 0x04, 0x0a, 0x05, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x5f, 0x75, 0x72, 0x69,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x55,
    0x72, 0x69, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x6f, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x70,
    0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x73, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0b, 0x2e, 0x50, 0x6c, 0x61, 0x79,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x30,
    0x0a, 0x14, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x65, 0x61, 0x73, 0x75,
    0x72, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x04, 0x52, 0x12, 0x70, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x64, 0x41, 0x74,
    0x12, 0x2f, 0x0a, 0x13, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x5f, 0x64, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x12, 0x63,
    0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x68, 0x75, 0x66, 0x66, 0x6c, 0x65, 0x18, 0x0d, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x07, 0x73, 0x68, 0x75, 0x66, 0x66, 0x6c, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x72,
    0x65, 0x70, 0x65, 0x61, 0x74, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x72, 0x65, 0x70,
    0x65, 0x61, 0x74, 0x12, 0x2c, 0x0a, 0x12, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x63, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x5f, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x18, 0x14, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x10, 0x6c, 0x61, 0x73, 0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x49, 0x64, 0x65, 0x6e,
    0x74, 0x12, 0x2c, 0x0a, 0x12, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e,
    0x64, 0x5f, 0x6d, 0x73, 0x67, 0x69, 0x64, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x6c,
    0x61, 0x73, 0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x4d, 0x73, 0x67, 0x69, 0x64, 0x12,
    0x32, 0x0a, 0x15, 0x70, 0x6c, 0x61, 0x79, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f,
    0x66, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x18, 0x18, 0x20, 0x01, 0x28, 0x08, 0x52, 0x13,
    0x70, 0x6c, 0x61, 0x79, 0x69, 0x6e, 0x67, 0x46, 0x72, 0x6f, 0x6d, 0x46, 0x61, 0x6c, 0x6c, 0x62,
    0x61, 0x63, 0x6b, 0x12, 0x10, 0x0a, 0x03, 0x72, 0x6f, 0x77, 0x18, 0x19, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x03, 0x72, 0x6f, 0x77, 0x12, 0x2e, 0x0a, 0x13, 0x70, 0x6c, 0x61, 0x79, 0x69, 0x6e, 0x67,
    0x5f, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x1a, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x11, 0x70, 0x6c, 0x61, 0x79, 0x69, 0x6e, 0x67, 0x54, 0x72, 0x61, 0x63, 0x6b,
    0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x1f, 0x0a, 0x05, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x18, 0x1b,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x66, 0x52,
    0x05, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x12, 0x13, 0x0a, 0x02, 0x61, 0x64, 0x18, 0x1c, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x03, 0x2e, 0x41, 0x64, 0x52, 0x02, 0x61, 0x64, 0x22, 0x60, 0x0a, 0x08, 0x54,
    0x72, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x66, 0x12, 0x10, 0x0a, 0x03, 0x67, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x67, 0x69, 0x64, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x69,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72, 0x69, 0x12, 0x16, 0x0a, 0x06, 0x71,
    0x75, 0x65, 0x75, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x71, 0x75, 0x65,
    0x75, 0x65, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x22, 0xfa, 0x01,
    0x0a, 0x02, 0x41, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x65, 0x78, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x04, 0x6e, 0x65, 0x78, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x6f, 0x67, 0x67, 0x5f,
    0x66, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x6f, 0x67, 0x67, 0x46, 0x69,
    0x64, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x5f, 0x66, 0x69, 0x64, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x46, 0x69, 0x64, 0x12, 0x1a,
    0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6c,
    0x69, 0x63, 0x6b, 0x5f, 0x75, 0x72, 0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63,
    0x6c, 0x69, 0x63, 0x6b, 0x55, 0x72, 0x6c, 0x12, 0x25, 0x0a, 0x0e, 0x69, 0x6d, 0x70, 0x72, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x75, 0x72, 0x6c, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0d, 0x69, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x55, 0x72, 0x6c, 0x12, 0x18,
    0x0a, 0x07, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x12, 0x1e, 0x0a, 0x0a, 0x61, 0x64, 0x76, 0x65,
    0x72, 0x74, 0x69, 0x73, 0x65, 0x72, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x61, 0x64,
    0x76, 0x65, 0x72, 0x74, 0x69, 0x73, 0x65, 0x72, 0x12, 0x10, 0x0a, 0x03, 0x67, 0x69, 0x64, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x67, 0x69, 0x64, 0x22, 0x3a, 0x0a, 0x08, 0x4d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x6d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2a, 0x8d, 0x04, 0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x15, 0x0a, 0x11, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x10, 0x01, 0x12, 0x17, 0x0a,
    0x13, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x47, 0x6f, 0x6f,
    0x64, 0x62, 0x79, 0x65, 0x10, 0x02, 0x12, 0x15, 0x0a, 0x11, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x10, 0x03, 0x12, 0x16, 0x0a,
    0x12, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x4e, 0x6f, 0x74,
    0x69, 0x66, 0x79, 0x10, 0x0a, 0x12, 0x14, 0x0a, 0x10, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x4c, 0x6f, 0x61, 0x64, 0x10, 0x14, 0x12, 0x14, 0x0a, 0x10, 0x6b,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50, 0x6c, 0x61, 0x79, 0x10,
    0x15, 0x12, 0x15, 0x0a, 0x11, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70,
    0x65, 0x50, 0x61, 0x75, 0x73, 0x65, 0x10, 0x16, 0x12, 0x19, 0x0a, 0x15, 0x6b, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50, 0x6c, 0x61, 0x79, 0x50, 0x61, 0x75, 0x73,
    0x65, 0x10, 0x17, 0x12, 0x14, 0x0a, 0x10, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x53, 0x65, 0x65, 0x6b, 0x10, 0x18, 0x12, 0x14, 0x0a, 0x10, 0x6b, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x65, 0x76, 0x10, 0x19, 0x12,
    0x14, 0x0a, 0x10, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x4e,
    0x65, 0x78, 0x74, 0x10, 0x1a, 0x12, 0x16, 0x0a, 0x12, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x56, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x10, 0x1b, 0x12, 0x17, 0x0a,
    0x13, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x53, 0x68, 0x75,
    0x66, 0x66, 0x6c, 0x65, 0x10, 0x1c, 0x12, 0x16, 0x0a, 0x12, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x10, 0x1d, 0x12, 0x1a,
    0x0a, 0x16, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x56, 0x6f,
    0x6c, 0x75, 0x6d, 0x65, 0x44, 0x6f, 0x77, 0x6e, 0x10, 0x1f, 0x12, 0x18, 0x0a, 0x14, 0x6b, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x56, 0x6f, 0x6c, 0x75, 0x6d, 0x65,
    0x55, 0x70, 0x10, 0x20, 0x12, 0x17, 0x0a, 0x13, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x54, 0x79, 0x70, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x10, 0x21, 0x12, 0x16, 0x0a,
    0x12, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x4c, 0x6f, 0x67,
    0x6f, 0x75, 0x74, 0x10, 0x22, 0x12, 0x16, 0x0a, 0x12, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0x23, 0x12, 0x16, 0x0a,
    0x12, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x65, 0x6e,
    0x61, 0x6d, 0x65, 0x10, 0x24, 0x12, 0x1f, 0x0a, 0x1a, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x10, 0x80, 0x01, 0x2a, 0xa1, 0x02, 0x0a, 0x0e, 0x43, 0x61, 0x70, 0x61, 0x62,
    0x69, 0x6c, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x12, 0x6b, 0x53, 0x75,
    0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x73, 0x10,
    0x01, 0x12, 0x10, 0x0a, 0x0c, 0x6b, 0x43, 0x61, 0x6e, 0x42, 0x65, 0x50, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x6b, 0x52, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74,
    0x54, 0x6f, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x10, 0x03, 0x12, 0x0f, 0x0a, 0x0b, 0x6b, 0x44, 0x65,
    0x76, 0x69, 0x63, 0x65, 0x54, 0x79, 0x70, 0x65, 0x10, 0x04, 0x12, 0x14, 0x0a, 0x10, 0x6b, 0x47,
    0x61, 0x69, 0x61, 0x45, 0x71, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x49, 0x64, 0x10, 0x05,
    0x12, 0x13, 0x0a, 0x0f, 0x6b, 0x53, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x4c, 0x6f, 0x67,
    0x6f, 0x75, 0x74, 0x10, 0x06, 0x12, 0x11, 0x0a, 0x0d, 0x6b, 0x49, 0x73, 0x4f, 0x62, 0x73, 0x65,
    0x72, 0x76, 0x61, 0x62, 0x6c, 0x65, 0x10, 0x07, 0x12, 0x10, 0x0a, 0x0c, 0x6b, 0x56, 0x6f, 0x6c,
    0x75, 0x6d, 0x65, 0x53, 0x74, 0x65, 0x70, 0x73, 0x10, 0x08, 0x12, 0x13, 0x0a, 0x0f, 0x6b, 0x53,
    0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x54, 0x79, 0x70, 0x65, 0x73, 0x10, 0x09, 0x12,
    0x10, 0x0a, 0x0c, 0x6b, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x41, 0x63, 0x6b, 0x73, 0x10,
    0x0a, 0x12, 0x13, 0x0a, 0x0f, 0x6b, 0x53, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x52, 0x65,
    0x6e, 0x61, 0x6d, 0x65, 0x10, 0x0b, 0x12, 0x0b, 0x0a, 0x07, 0x6b, 0x48, 0x69, 0x64, 0x64, 0x65,
    0x6e, 0x10, 0x0c, 0x12, 0x17, 0x0a, 0x13, 0x6b, 0x53, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x73,
    0x50, 0x6c, 0x61, 0x79, 0x6c, 0x69, 0x73, 0x74, 0x56, 0x32, 0x10, 0x0d, 0x12, 0x0c, 0x0a, 0x08,
    0x6b, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x10, 0x0e, 0x2a, 0x64, 0x0a, 0x0a, 0x50, 0x6c,
    0x61, 0x79, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x13, 0x0a, 0x0f, 0x6b, 0x50, 0x6c, 0x61,
    0x79, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x53, 0x74, 0x6f, 0x70, 0x10, 0x00, 0x12, 0x13, 0x0a,
    0x0f, 0x6b, 0x50, 0x6c, 0x61, 0x79, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x6c, 0x61, 0x79,
    0x10, 0x01, 0x12, 0x14, 0x0a, 0x10, 0x6b, 0x50, 0x6c, 0x61, 0x79, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x50, 0x61, 0x75, 0x73, 0x65, 0x10, 0x02, 0x12, 0x16, 0x0a, 0x12, 0x6b, 0x50, 0x6c, 0x61,
    0x79, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x4c, 0x6f, 0x61, 0x64, 0x69, 0x6e, 0x67, 0x10, 0x03,
    0x4a, 0x93, 0x2f, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0x84, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00,
    0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x0d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x03, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x03, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x03, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x04, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x05, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x05, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x05, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x14, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x27, 0x2a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x06, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x06, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x06,
    0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07, 0x04, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x07, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x07, 0x19, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x07, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12,
    0x03, 0x08, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x08,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x08, 0x0d, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x08, 0x19, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x08, 0x28, 0x2b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x09, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x04, 0x12, 0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06,
    0x12, 0x03, 0x09, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x09, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x09, 0x1f,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0a, 0x04, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x0a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x0a, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x0a, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03,
    0x0b, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x0b, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x0b, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0b, 0x14, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x0b, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x09, 0x12, 0x03, 0x0c, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09,
    0x04, 0x12, 0x03, 0x0c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12,
    0x03, 0x0c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x0c,
    0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x0c, 0x1d, 0x20,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x0d, 0x04, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x0d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x01, 0x12, 0x03, 0x0d, 0x13, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03,
    0x12, 0x03, 0x0d, 0x25, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x0e,
    0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x0e, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x0e, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x0e, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x0e, 0x20, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0c, 0x12, 0x03, 0x0f, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04,
    0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03,
    0x0f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x0f, 0x13,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x0f, 0x2a, 0x2e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x10, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0d, 0x05, 0x12, 0x03, 0x10, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d,
    0x01, 0x12, 0x03, 0x10, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12,
    0x03, 0x10, 0x1f, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x11, 0x04,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x06, 0x12, 0x03, 0x11, 0x0d, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x11, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x11, 0x21, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12,
    0x04, 0x14, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x14, 0x05,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x15, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x16, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x16, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x16, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x17, 0x04, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x17, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x18, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x18, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x18, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x19,
    0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x19, 0x04, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x19, 0x17, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1a, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1a, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x05, 0x02, 0x12, 0x03, 0x1a, 0x17, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12,
    0x03, 0x1b, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1b,
    0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x1b, 0x18, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x1c, 0x04, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x1c, 0x1c, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x08, 0x12, 0x03, 0x1d, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x1d, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x1d,
    0x17, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x1e, 0x04, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1e, 0x04, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x1e, 0x17, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x0a, 0x12, 0x03, 0x1f, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x1f, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12,
    0x03, 0x1f, 0x17, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x20, 0x04,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x20, 0x04, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x20, 0x19, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x21, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0c, 0x01, 0x12, 0x03, 0x21, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c,
    0x02, 0x12, 0x03, 0x21, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0d, 0x12, 0x03,
    0x22, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x22, 0x04,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x22, 0x19, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x23, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x23, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0e, 0x02, 0x12, 0x03, 0x23, 0x1d, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0f,
    0x12, 0x03, 0x24, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03,
    0x24, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x24, 0x1b,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x10, 0x12, 0x03, 0x25, 0x04, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x25, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x10, 0x02, 0x12, 0x03, 0x25, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x11, 0x12, 0x03, 0x26, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x01,
    0x12, 0x03, 0x26, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x02, 0x12, 0x03,
    0x26, 0x19, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x12, 0x12, 0x03, 0x27, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x27, 0x04, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x12, 0x02, 0x12, 0x03, 0x27, 0x19, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x13, 0x12, 0x03, 0x28, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x13, 0x01, 0x12, 0x03, 0x28, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x13, 0x02,
    0x12, 0x03, 0x28, 0x19, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x14, 0x12, 0x03, 0x29,
    0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x14, 0x01, 0x12, 0x03, 0x29, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x14, 0x02, 0x12, 0x03, 0x29, 0x21, 0x25, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x2c, 0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x2c, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x2d, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x14, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x2e, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e,
    0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x1e, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x04, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2f, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x2f, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x2f, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x30,
    0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x30, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x30, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x30, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x30, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x04, 0x12, 0x03, 0x31, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x31, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x31, 0x14,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x31, 0x1b, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x32, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x32, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x05, 0x05, 0x12, 0x03, 0x32, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x32, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x32, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x33, 0x04,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x33, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x33, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x33, 0x13, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x33, 0x26, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x07, 0x12, 0x03, 0x34, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x34, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x34,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x34, 0x14, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x34, 0x24, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x35, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x08, 0x04, 0x12, 0x03, 0x35, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x08, 0x06, 0x12, 0x03, 0x35, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x35, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x35, 0x27, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09, 0x12, 0x03, 0x36, 0x04, 0x30,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x04, 0x12, 0x03, 0x36, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03, 0x36, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x36, 0x14, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x36, 0x2b, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a,
    0x12, 0x03, 0x37, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x04, 0x12, 0x03,
    0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x37, 0x0d,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x37, 0x16, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x37, 0x21, 0x25, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x3a, 0x00, 0x3e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x3a, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x3b,
    0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3b, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3b, 0x0d, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x1c, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x3c, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x3c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x3c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3c, 0x13,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3c, 0x1e, 0x21, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x3d, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x3d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x3d, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x3d, 0x22, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x40, 0x00, 0x4f, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x40, 0x05, 0x13, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x41, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x41, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x41, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x42,
    0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x42, 0x04, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x42, 0x13, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x43, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x43, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x43, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x03, 0x12,
    0x03, 0x44, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x44,
    0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02, 0x12, 0x03, 0x44, 0x12, 0x15,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x04, 0x12, 0x03, 0x45, 0x04, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x45, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x04, 0x02, 0x12, 0x03, 0x45, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x05, 0x12, 0x03, 0x46, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x46, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x02, 0x12, 0x03, 0x46,
    0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x06, 0x12, 0x03, 0x47, 0x04, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x47, 0x04, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x06, 0x02, 0x12, 0x03, 0x47, 0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x07, 0x12, 0x03, 0x48, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x48, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x07, 0x02, 0x12,
    0x03, 0x48, 0x13, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x08, 0x12, 0x03, 0x49, 0x04,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x49, 0x04, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x08, 0x02, 0x12, 0x03, 0x49, 0x16, 0x19, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x01, 0x02, 0x09, 0x12, 0x03, 0x4a, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x4a, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x09,
    0x02, 0x12, 0x03, 0x4a, 0x13, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0a, 0x12, 0x03,
    0x4b, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x4b, 0x04,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x4b, 0x16, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0b, 0x12, 0x03, 0x4c, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x4c, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x0b, 0x02, 0x12, 0x03, 0x4c, 0x0e, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0c,
    0x12, 0x03, 0x4d, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x03,
    0x4d, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x4d, 0x1a,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0d, 0x12, 0x03, 0x4e, 0x04, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x4e, 0x0f, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x51, 0x00, 0x53, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x51,
    0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x52, 0x04, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x52, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x52, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x52, 0x1d, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x55,
    0x00, 0x65, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x55, 0x08, 0x0d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x56, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x56, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x56, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x56, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x57, 0x04,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x57, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x57, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x57, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x57, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x02, 0x12, 0x03, 0x58, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x58, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x58,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x58, 0x14, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x58, 0x22, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x59, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x59, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x59, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x59, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x59, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x5a, 0x04, 0x2f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x5a, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x5a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x5a, 0x14, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x5a, 0x2b, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05,
    0x12, 0x03, 0x5b, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x5b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x5b, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x5b, 0x14, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x5b, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x06, 0x12, 0x03, 0x5c, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x5c, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x5c, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x5c,
    0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x07, 0x12, 0x03, 0x5d, 0x04, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x04, 0x12, 0x03, 0x5d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x07, 0x05, 0x12, 0x03, 0x5d, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x07, 0x01, 0x12, 0x03, 0x5d, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x07, 0x03, 0x12, 0x03, 0x5d, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x08, 0x12,
    0x03, 0x5e, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x04, 0x12, 0x03, 0x5e,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x05, 0x12, 0x03, 0x5e, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x01, 0x12, 0x03, 0x5e, 0x14, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x03, 0x12, 0x03, 0x5e, 0x29, 0x2d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x09, 0x12, 0x03, 0x5f, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x09, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x05,
    0x12, 0x03, 0x5f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x5f, 0x14, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x03, 0x12, 0x03, 0x5f, 0x29,
    0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0a, 0x12, 0x03, 0x60, 0x04, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x60, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x60, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x60, 0x12, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a,
    0x03, 0x12, 0x03, 0x60, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0b, 0x12, 0x03,
    0x61, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x61, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x61, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x61, 0x14, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x61, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x0c, 0x12, 0x03, 0x62, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c,
    0x04, 0x12, 0x03, 0x62, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x05, 0x12,
    0x03, 0x62, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x62,
    0x14, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x62, 0x2a, 0x2e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0d, 0x12, 0x03, 0x63, 0x04, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x63, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x0d, 0x06, 0x12, 0x03, 0x63, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0d, 0x01, 0x12, 0x03, 0x63, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x03,
    0x12, 0x03, 0x63, 0x1e, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0e, 0x12, 0x03, 0x64,
    0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x64, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x06, 0x12, 0x03, 0x64, 0x0d, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x64, 0x10, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x64, 0x15, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02,
    0x12, 0x04, 0x67, 0x00, 0x6c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x67,
    0x05, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12, 0x03, 0x68, 0x04, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x68, 0x04, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x68, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x69, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x69, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x69, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x6a, 0x04,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x04, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x03, 0x6a, 0x17, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6b, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x6b, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x6b, 0x19, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x6e, 0x00,
    0x73, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x6e, 0x08, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x6f, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x6f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x6f, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x6f, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x70, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x70, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x70, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x70, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02,
    0x12, 0x03, 0x71, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x71, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x71, 0x0d,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x71, 0x12, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x71, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x72, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x72, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x72, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x72, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x72,
    0x1e, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x75, 0x00, 0x7f, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x75, 0x08, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x76, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x76, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x76, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x76, 0x13,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x76, 0x1a, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x77, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x77, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x77, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x77, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x77, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x78, 0x04,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x78, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x78, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x78, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x78, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x03, 0x12, 0x03, 0x79, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x79, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x79,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x79, 0x13, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x79, 0x1e, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x7a, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x7a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x7a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x7a, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x7a, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x7b, 0x04, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x03, 0x7b, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x05, 0x12, 0x03, 0x7b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x7b, 0x14, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x7b, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06,
    0x12, 0x03, 0x7c, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x7c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x05, 0x12, 0x03, 0x7c, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12, 0x03, 0x7c, 0x14, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x03, 0x7c, 0x1e, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x07, 0x12, 0x03, 0x7d, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x07, 0x04, 0x12, 0x03, 0x7d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07,
    0x05, 0x12, 0x03, 0x7d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x7d, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x03, 0x12, 0x03, 0x7d,
    0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x08, 0x12, 0x03, 0x7e, 0x04, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x04, 0x12, 0x03, 0x7e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x08, 0x05, 0x12, 0x03, 0x7e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x08, 0x01, 0x12, 0x03, 0x7e, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x7e, 0x19, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x06, 0x81,
    0x01, 0x00, 0x84, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x04, 0x81, 0x01,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x04, 0x82, 0x01, 0x04, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0x82, 0x01, 0x14, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0x82, 0x01, 0x1b, 0x1e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x01, 0x12, 0x04, 0x83, 0x01, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x83, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x05, 0x12, 0x04, 0x83, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x83, 0x01, 0x14, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x83, 0x01, 0x1f, 0x22,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
