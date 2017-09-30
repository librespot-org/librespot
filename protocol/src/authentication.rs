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
pub struct ClientResponseEncrypted {
    // message fields
    login_credentials: ::protobuf::SingularPtrField<LoginCredentials>,
    account_creation: ::std::option::Option<AccountCreation>,
    fingerprint_response: ::protobuf::SingularPtrField<FingerprintResponseUnion>,
    peer_ticket: ::protobuf::SingularPtrField<PeerTicketUnion>,
    system_info: ::protobuf::SingularPtrField<SystemInfo>,
    platform_model: ::protobuf::SingularField<::std::string::String>,
    version_string: ::protobuf::SingularField<::std::string::String>,
    appkey: ::protobuf::SingularPtrField<LibspotifyAppKey>,
    client_info: ::protobuf::SingularPtrField<ClientInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientResponseEncrypted {}

impl ClientResponseEncrypted {
    pub fn new() -> ClientResponseEncrypted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientResponseEncrypted {
        static mut instance: ::protobuf::lazy::Lazy<ClientResponseEncrypted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientResponseEncrypted,
        };
        unsafe {
            instance.get(ClientResponseEncrypted::new)
        }
    }

    // required .LoginCredentials login_credentials = 10;

    pub fn clear_login_credentials(&mut self) {
        self.login_credentials.clear();
    }

    pub fn has_login_credentials(&self) -> bool {
        self.login_credentials.is_some()
    }

    // Param is passed by value, moved
    pub fn set_login_credentials(&mut self, v: LoginCredentials) {
        self.login_credentials = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_login_credentials(&mut self) -> &mut LoginCredentials {
        if self.login_credentials.is_none() {
            self.login_credentials.set_default();
        }
        self.login_credentials.as_mut().unwrap()
    }

    // Take field
    pub fn take_login_credentials(&mut self) -> LoginCredentials {
        self.login_credentials.take().unwrap_or_else(|| LoginCredentials::new())
    }

    pub fn get_login_credentials(&self) -> &LoginCredentials {
        self.login_credentials.as_ref().unwrap_or_else(|| LoginCredentials::default_instance())
    }

    fn get_login_credentials_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCredentials> {
        &self.login_credentials
    }

    fn mut_login_credentials_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCredentials> {
        &mut self.login_credentials
    }

    // optional .AccountCreation account_creation = 20;

    pub fn clear_account_creation(&mut self) {
        self.account_creation = ::std::option::Option::None;
    }

    pub fn has_account_creation(&self) -> bool {
        self.account_creation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_creation(&mut self, v: AccountCreation) {
        self.account_creation = ::std::option::Option::Some(v);
    }

    pub fn get_account_creation(&self) -> AccountCreation {
        self.account_creation.unwrap_or(AccountCreation::ACCOUNT_CREATION_ALWAYS_PROMPT)
    }

    fn get_account_creation_for_reflect(&self) -> &::std::option::Option<AccountCreation> {
        &self.account_creation
    }

    fn mut_account_creation_for_reflect(&mut self) -> &mut ::std::option::Option<AccountCreation> {
        &mut self.account_creation
    }

    // optional .FingerprintResponseUnion fingerprint_response = 30;

    pub fn clear_fingerprint_response(&mut self) {
        self.fingerprint_response.clear();
    }

    pub fn has_fingerprint_response(&self) -> bool {
        self.fingerprint_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fingerprint_response(&mut self, v: FingerprintResponseUnion) {
        self.fingerprint_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fingerprint_response(&mut self) -> &mut FingerprintResponseUnion {
        if self.fingerprint_response.is_none() {
            self.fingerprint_response.set_default();
        }
        self.fingerprint_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_fingerprint_response(&mut self) -> FingerprintResponseUnion {
        self.fingerprint_response.take().unwrap_or_else(|| FingerprintResponseUnion::new())
    }

    pub fn get_fingerprint_response(&self) -> &FingerprintResponseUnion {
        self.fingerprint_response.as_ref().unwrap_or_else(|| FingerprintResponseUnion::default_instance())
    }

    fn get_fingerprint_response_for_reflect(&self) -> &::protobuf::SingularPtrField<FingerprintResponseUnion> {
        &self.fingerprint_response
    }

    fn mut_fingerprint_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FingerprintResponseUnion> {
        &mut self.fingerprint_response
    }

    // optional .PeerTicketUnion peer_ticket = 40;

    pub fn clear_peer_ticket(&mut self) {
        self.peer_ticket.clear();
    }

    pub fn has_peer_ticket(&self) -> bool {
        self.peer_ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer_ticket(&mut self, v: PeerTicketUnion) {
        self.peer_ticket = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer_ticket(&mut self) -> &mut PeerTicketUnion {
        if self.peer_ticket.is_none() {
            self.peer_ticket.set_default();
        }
        self.peer_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer_ticket(&mut self) -> PeerTicketUnion {
        self.peer_ticket.take().unwrap_or_else(|| PeerTicketUnion::new())
    }

    pub fn get_peer_ticket(&self) -> &PeerTicketUnion {
        self.peer_ticket.as_ref().unwrap_or_else(|| PeerTicketUnion::default_instance())
    }

    fn get_peer_ticket_for_reflect(&self) -> &::protobuf::SingularPtrField<PeerTicketUnion> {
        &self.peer_ticket
    }

    fn mut_peer_ticket_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PeerTicketUnion> {
        &mut self.peer_ticket
    }

    // required .SystemInfo system_info = 50;

    pub fn clear_system_info(&mut self) {
        self.system_info.clear();
    }

    pub fn has_system_info(&self) -> bool {
        self.system_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_system_info(&mut self, v: SystemInfo) {
        self.system_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_system_info(&mut self) -> &mut SystemInfo {
        if self.system_info.is_none() {
            self.system_info.set_default();
        }
        self.system_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_system_info(&mut self) -> SystemInfo {
        self.system_info.take().unwrap_or_else(|| SystemInfo::new())
    }

    pub fn get_system_info(&self) -> &SystemInfo {
        self.system_info.as_ref().unwrap_or_else(|| SystemInfo::default_instance())
    }

    fn get_system_info_for_reflect(&self) -> &::protobuf::SingularPtrField<SystemInfo> {
        &self.system_info
    }

    fn mut_system_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SystemInfo> {
        &mut self.system_info
    }

    // optional string platform_model = 60;

    pub fn clear_platform_model(&mut self) {
        self.platform_model.clear();
    }

    pub fn has_platform_model(&self) -> bool {
        self.platform_model.is_some()
    }

    // Param is passed by value, moved
    pub fn set_platform_model(&mut self, v: ::std::string::String) {
        self.platform_model = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_platform_model(&mut self) -> &mut ::std::string::String {
        if self.platform_model.is_none() {
            self.platform_model.set_default();
        }
        self.platform_model.as_mut().unwrap()
    }

    // Take field
    pub fn take_platform_model(&mut self) -> ::std::string::String {
        self.platform_model.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_platform_model(&self) -> &str {
        match self.platform_model.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_platform_model_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.platform_model
    }

    fn mut_platform_model_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.platform_model
    }

    // optional string version_string = 70;

    pub fn clear_version_string(&mut self) {
        self.version_string.clear();
    }

    pub fn has_version_string(&self) -> bool {
        self.version_string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version_string(&mut self, v: ::std::string::String) {
        self.version_string = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version_string(&mut self) -> &mut ::std::string::String {
        if self.version_string.is_none() {
            self.version_string.set_default();
        }
        self.version_string.as_mut().unwrap()
    }

    // Take field
    pub fn take_version_string(&mut self) -> ::std::string::String {
        self.version_string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version_string(&self) -> &str {
        match self.version_string.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_version_string_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.version_string
    }

    fn mut_version_string_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.version_string
    }

    // optional .LibspotifyAppKey appkey = 80;

    pub fn clear_appkey(&mut self) {
        self.appkey.clear();
    }

    pub fn has_appkey(&self) -> bool {
        self.appkey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appkey(&mut self, v: LibspotifyAppKey) {
        self.appkey = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_appkey(&mut self) -> &mut LibspotifyAppKey {
        if self.appkey.is_none() {
            self.appkey.set_default();
        }
        self.appkey.as_mut().unwrap()
    }

    // Take field
    pub fn take_appkey(&mut self) -> LibspotifyAppKey {
        self.appkey.take().unwrap_or_else(|| LibspotifyAppKey::new())
    }

    pub fn get_appkey(&self) -> &LibspotifyAppKey {
        self.appkey.as_ref().unwrap_or_else(|| LibspotifyAppKey::default_instance())
    }

    fn get_appkey_for_reflect(&self) -> &::protobuf::SingularPtrField<LibspotifyAppKey> {
        &self.appkey
    }

    fn mut_appkey_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LibspotifyAppKey> {
        &mut self.appkey
    }

    // optional .ClientInfo client_info = 90;

    pub fn clear_client_info(&mut self) {
        self.client_info.clear();
    }

    pub fn has_client_info(&self) -> bool {
        self.client_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_info(&mut self, v: ClientInfo) {
        self.client_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_info(&mut self) -> &mut ClientInfo {
        if self.client_info.is_none() {
            self.client_info.set_default();
        }
        self.client_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_info(&mut self) -> ClientInfo {
        self.client_info.take().unwrap_or_else(|| ClientInfo::new())
    }

    pub fn get_client_info(&self) -> &ClientInfo {
        self.client_info.as_ref().unwrap_or_else(|| ClientInfo::default_instance())
    }

    fn get_client_info_for_reflect(&self) -> &::protobuf::SingularPtrField<ClientInfo> {
        &self.client_info
    }

    fn mut_client_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClientInfo> {
        &mut self.client_info
    }
}

impl ::protobuf::Message for ClientResponseEncrypted {
    fn is_initialized(&self) -> bool {
        if self.login_credentials.is_none() {
            return false;
        }
        if self.system_info.is_none() {
            return false;
        }
        for v in &self.login_credentials {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fingerprint_response {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.peer_ticket {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.system_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.appkey {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.client_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login_credentials)?;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.account_creation = ::std::option::Option::Some(tmp);
                },
                30 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fingerprint_response)?;
                },
                40 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer_ticket)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.system_info)?;
                },
                60 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.platform_model)?;
                },
                70 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version_string)?;
                },
                80 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.appkey)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.client_info)?;
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
        if let Some(ref v) = self.login_credentials.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.account_creation {
            my_size += ::protobuf::rt::enum_size(20, v);
        }
        if let Some(ref v) = self.fingerprint_response.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.peer_ticket.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.system_info.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.platform_model.as_ref() {
            my_size += ::protobuf::rt::string_size(60, &v);
        }
        if let Some(ref v) = self.version_string.as_ref() {
            my_size += ::protobuf::rt::string_size(70, &v);
        }
        if let Some(ref v) = self.appkey.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.client_info.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.login_credentials.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.account_creation {
            os.write_enum(20, v.value())?;
        }
        if let Some(ref v) = self.fingerprint_response.as_ref() {
            os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.peer_ticket.as_ref() {
            os.write_tag(40, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.system_info.as_ref() {
            os.write_tag(50, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.platform_model.as_ref() {
            os.write_string(60, &v)?;
        }
        if let Some(ref v) = self.version_string.as_ref() {
            os.write_string(70, &v)?;
        }
        if let Some(ref v) = self.appkey.as_ref() {
            os.write_tag(80, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.client_info.as_ref() {
            os.write_tag(90, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for ClientResponseEncrypted {
    fn new() -> ClientResponseEncrypted {
        ClientResponseEncrypted::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientResponseEncrypted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCredentials>>(
                    "login_credentials",
                    ClientResponseEncrypted::get_login_credentials_for_reflect,
                    ClientResponseEncrypted::mut_login_credentials_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AccountCreation>>(
                    "account_creation",
                    ClientResponseEncrypted::get_account_creation_for_reflect,
                    ClientResponseEncrypted::mut_account_creation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FingerprintResponseUnion>>(
                    "fingerprint_response",
                    ClientResponseEncrypted::get_fingerprint_response_for_reflect,
                    ClientResponseEncrypted::mut_fingerprint_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PeerTicketUnion>>(
                    "peer_ticket",
                    ClientResponseEncrypted::get_peer_ticket_for_reflect,
                    ClientResponseEncrypted::mut_peer_ticket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SystemInfo>>(
                    "system_info",
                    ClientResponseEncrypted::get_system_info_for_reflect,
                    ClientResponseEncrypted::mut_system_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "platform_model",
                    ClientResponseEncrypted::get_platform_model_for_reflect,
                    ClientResponseEncrypted::mut_platform_model_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version_string",
                    ClientResponseEncrypted::get_version_string_for_reflect,
                    ClientResponseEncrypted::mut_version_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LibspotifyAppKey>>(
                    "appkey",
                    ClientResponseEncrypted::get_appkey_for_reflect,
                    ClientResponseEncrypted::mut_appkey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientInfo>>(
                    "client_info",
                    ClientResponseEncrypted::get_client_info_for_reflect,
                    ClientResponseEncrypted::mut_client_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientResponseEncrypted>(
                    "ClientResponseEncrypted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientResponseEncrypted {
    fn clear(&mut self) {
        self.clear_login_credentials();
        self.clear_account_creation();
        self.clear_fingerprint_response();
        self.clear_peer_ticket();
        self.clear_system_info();
        self.clear_platform_model();
        self.clear_version_string();
        self.clear_appkey();
        self.clear_client_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientResponseEncrypted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientResponseEncrypted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCredentials {
    // message fields
    username: ::protobuf::SingularField<::std::string::String>,
    typ: ::std::option::Option<AuthenticationType>,
    auth_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCredentials {}

impl LoginCredentials {
    pub fn new() -> LoginCredentials {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCredentials {
        static mut instance: ::protobuf::lazy::Lazy<LoginCredentials> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCredentials,
        };
        unsafe {
            instance.get(LoginCredentials::new)
        }
    }

    // optional string username = 10;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    pub fn has_username(&self) -> bool {
        self.username.is_some()
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        if self.username.is_none() {
            self.username.set_default();
        }
        self.username.as_mut().unwrap()
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        self.username.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        match self.username.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_username_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.username
    }

    // required .AuthenticationType typ = 20;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: AuthenticationType) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> AuthenticationType {
        self.typ.unwrap_or(AuthenticationType::AUTHENTICATION_USER_PASS)
    }

    fn get_typ_for_reflect(&self) -> &::std::option::Option<AuthenticationType> {
        &self.typ
    }

    fn mut_typ_for_reflect(&mut self) -> &mut ::std::option::Option<AuthenticationType> {
        &mut self.typ
    }

    // optional bytes auth_data = 30;

    pub fn clear_auth_data(&mut self) {
        self.auth_data.clear();
    }

    pub fn has_auth_data(&self) -> bool {
        self.auth_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.auth_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.auth_data.is_none() {
            self.auth_data.set_default();
        }
        self.auth_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_data(&mut self) -> ::std::vec::Vec<u8> {
        self.auth_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_auth_data(&self) -> &[u8] {
        match self.auth_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_auth_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.auth_data
    }

    fn mut_auth_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.auth_data
    }
}

impl ::protobuf::Message for LoginCredentials {
    fn is_initialized(&self) -> bool {
        if self.typ.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.username)?;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.typ = ::std::option::Option::Some(tmp);
                },
                30 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.auth_data)?;
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
        if let Some(ref v) = self.username.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(v) = self.typ {
            my_size += ::protobuf::rt::enum_size(20, v);
        }
        if let Some(ref v) = self.auth_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(30, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.username.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(v) = self.typ {
            os.write_enum(20, v.value())?;
        }
        if let Some(ref v) = self.auth_data.as_ref() {
            os.write_bytes(30, &v)?;
        }
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

impl ::protobuf::MessageStatic for LoginCredentials {
    fn new() -> LoginCredentials {
        LoginCredentials::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCredentials>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    LoginCredentials::get_username_for_reflect,
                    LoginCredentials::mut_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AuthenticationType>>(
                    "typ",
                    LoginCredentials::get_typ_for_reflect,
                    LoginCredentials::mut_typ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "auth_data",
                    LoginCredentials::get_auth_data_for_reflect,
                    LoginCredentials::mut_auth_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCredentials>(
                    "LoginCredentials",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCredentials {
    fn clear(&mut self) {
        self.clear_username();
        self.clear_typ();
        self.clear_auth_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCredentials {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCredentials {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FingerprintResponseUnion {
    // message fields
    grain: ::protobuf::SingularPtrField<FingerprintGrainResponse>,
    hmac_ripemd: ::protobuf::SingularPtrField<FingerprintHmacRipemdResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FingerprintResponseUnion {}

impl FingerprintResponseUnion {
    pub fn new() -> FingerprintResponseUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FingerprintResponseUnion {
        static mut instance: ::protobuf::lazy::Lazy<FingerprintResponseUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FingerprintResponseUnion,
        };
        unsafe {
            instance.get(FingerprintResponseUnion::new)
        }
    }

    // optional .FingerprintGrainResponse grain = 10;

    pub fn clear_grain(&mut self) {
        self.grain.clear();
    }

    pub fn has_grain(&self) -> bool {
        self.grain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_grain(&mut self, v: FingerprintGrainResponse) {
        self.grain = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_grain(&mut self) -> &mut FingerprintGrainResponse {
        if self.grain.is_none() {
            self.grain.set_default();
        }
        self.grain.as_mut().unwrap()
    }

    // Take field
    pub fn take_grain(&mut self) -> FingerprintGrainResponse {
        self.grain.take().unwrap_or_else(|| FingerprintGrainResponse::new())
    }

    pub fn get_grain(&self) -> &FingerprintGrainResponse {
        self.grain.as_ref().unwrap_or_else(|| FingerprintGrainResponse::default_instance())
    }

    fn get_grain_for_reflect(&self) -> &::protobuf::SingularPtrField<FingerprintGrainResponse> {
        &self.grain
    }

    fn mut_grain_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FingerprintGrainResponse> {
        &mut self.grain
    }

    // optional .FingerprintHmacRipemdResponse hmac_ripemd = 20;

    pub fn clear_hmac_ripemd(&mut self) {
        self.hmac_ripemd.clear();
    }

    pub fn has_hmac_ripemd(&self) -> bool {
        self.hmac_ripemd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hmac_ripemd(&mut self, v: FingerprintHmacRipemdResponse) {
        self.hmac_ripemd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hmac_ripemd(&mut self) -> &mut FingerprintHmacRipemdResponse {
        if self.hmac_ripemd.is_none() {
            self.hmac_ripemd.set_default();
        }
        self.hmac_ripemd.as_mut().unwrap()
    }

    // Take field
    pub fn take_hmac_ripemd(&mut self) -> FingerprintHmacRipemdResponse {
        self.hmac_ripemd.take().unwrap_or_else(|| FingerprintHmacRipemdResponse::new())
    }

    pub fn get_hmac_ripemd(&self) -> &FingerprintHmacRipemdResponse {
        self.hmac_ripemd.as_ref().unwrap_or_else(|| FingerprintHmacRipemdResponse::default_instance())
    }

    fn get_hmac_ripemd_for_reflect(&self) -> &::protobuf::SingularPtrField<FingerprintHmacRipemdResponse> {
        &self.hmac_ripemd
    }

    fn mut_hmac_ripemd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FingerprintHmacRipemdResponse> {
        &mut self.hmac_ripemd
    }
}

impl ::protobuf::Message for FingerprintResponseUnion {
    fn is_initialized(&self) -> bool {
        for v in &self.grain {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.hmac_ripemd {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.grain)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hmac_ripemd)?;
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
        if let Some(ref v) = self.grain.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.hmac_ripemd.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.grain.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.hmac_ripemd.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for FingerprintResponseUnion {
    fn new() -> FingerprintResponseUnion {
        FingerprintResponseUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<FingerprintResponseUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FingerprintGrainResponse>>(
                    "grain",
                    FingerprintResponseUnion::get_grain_for_reflect,
                    FingerprintResponseUnion::mut_grain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FingerprintHmacRipemdResponse>>(
                    "hmac_ripemd",
                    FingerprintResponseUnion::get_hmac_ripemd_for_reflect,
                    FingerprintResponseUnion::mut_hmac_ripemd_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FingerprintResponseUnion>(
                    "FingerprintResponseUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FingerprintResponseUnion {
    fn clear(&mut self) {
        self.clear_grain();
        self.clear_hmac_ripemd();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FingerprintResponseUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FingerprintResponseUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FingerprintGrainResponse {
    // message fields
    encrypted_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FingerprintGrainResponse {}

impl FingerprintGrainResponse {
    pub fn new() -> FingerprintGrainResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FingerprintGrainResponse {
        static mut instance: ::protobuf::lazy::Lazy<FingerprintGrainResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FingerprintGrainResponse,
        };
        unsafe {
            instance.get(FingerprintGrainResponse::new)
        }
    }

    // required bytes encrypted_key = 10;

    pub fn clear_encrypted_key(&mut self) {
        self.encrypted_key.clear();
    }

    pub fn has_encrypted_key(&self) -> bool {
        self.encrypted_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encrypted_key.is_none() {
            self.encrypted_key.set_default();
        }
        self.encrypted_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_key(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_key(&self) -> &[u8] {
        match self.encrypted_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_encrypted_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.encrypted_key
    }

    fn mut_encrypted_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.encrypted_key
    }
}

impl ::protobuf::Message for FingerprintGrainResponse {
    fn is_initialized(&self) -> bool {
        if self.encrypted_key.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encrypted_key)?;
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
        if let Some(ref v) = self.encrypted_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.encrypted_key.as_ref() {
            os.write_bytes(10, &v)?;
        }
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

impl ::protobuf::MessageStatic for FingerprintGrainResponse {
    fn new() -> FingerprintGrainResponse {
        FingerprintGrainResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FingerprintGrainResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encrypted_key",
                    FingerprintGrainResponse::get_encrypted_key_for_reflect,
                    FingerprintGrainResponse::mut_encrypted_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FingerprintGrainResponse>(
                    "FingerprintGrainResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FingerprintGrainResponse {
    fn clear(&mut self) {
        self.clear_encrypted_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FingerprintGrainResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FingerprintGrainResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FingerprintHmacRipemdResponse {
    // message fields
    hmac: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FingerprintHmacRipemdResponse {}

impl FingerprintHmacRipemdResponse {
    pub fn new() -> FingerprintHmacRipemdResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FingerprintHmacRipemdResponse {
        static mut instance: ::protobuf::lazy::Lazy<FingerprintHmacRipemdResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FingerprintHmacRipemdResponse,
        };
        unsafe {
            instance.get(FingerprintHmacRipemdResponse::new)
        }
    }

    // required bytes hmac = 10;

    pub fn clear_hmac(&mut self) {
        self.hmac.clear();
    }

    pub fn has_hmac(&self) -> bool {
        self.hmac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hmac(&mut self, v: ::std::vec::Vec<u8>) {
        self.hmac = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hmac(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hmac.is_none() {
            self.hmac.set_default();
        }
        self.hmac.as_mut().unwrap()
    }

    // Take field
    pub fn take_hmac(&mut self) -> ::std::vec::Vec<u8> {
        self.hmac.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hmac(&self) -> &[u8] {
        match self.hmac.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_hmac_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.hmac
    }

    fn mut_hmac_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.hmac
    }
}

impl ::protobuf::Message for FingerprintHmacRipemdResponse {
    fn is_initialized(&self) -> bool {
        if self.hmac.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hmac)?;
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
        if let Some(ref v) = self.hmac.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.hmac.as_ref() {
            os.write_bytes(10, &v)?;
        }
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

impl ::protobuf::MessageStatic for FingerprintHmacRipemdResponse {
    fn new() -> FingerprintHmacRipemdResponse {
        FingerprintHmacRipemdResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FingerprintHmacRipemdResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hmac",
                    FingerprintHmacRipemdResponse::get_hmac_for_reflect,
                    FingerprintHmacRipemdResponse::mut_hmac_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FingerprintHmacRipemdResponse>(
                    "FingerprintHmacRipemdResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FingerprintHmacRipemdResponse {
    fn clear(&mut self) {
        self.clear_hmac();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FingerprintHmacRipemdResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FingerprintHmacRipemdResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PeerTicketUnion {
    // message fields
    public_key: ::protobuf::SingularPtrField<PeerTicketPublicKey>,
    old_ticket: ::protobuf::SingularPtrField<PeerTicketOld>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PeerTicketUnion {}

impl PeerTicketUnion {
    pub fn new() -> PeerTicketUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerTicketUnion {
        static mut instance: ::protobuf::lazy::Lazy<PeerTicketUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerTicketUnion,
        };
        unsafe {
            instance.get(PeerTicketUnion::new)
        }
    }

    // optional .PeerTicketPublicKey public_key = 10;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: PeerTicketPublicKey) {
        self.public_key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut PeerTicketPublicKey {
        if self.public_key.is_none() {
            self.public_key.set_default();
        }
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> PeerTicketPublicKey {
        self.public_key.take().unwrap_or_else(|| PeerTicketPublicKey::new())
    }

    pub fn get_public_key(&self) -> &PeerTicketPublicKey {
        self.public_key.as_ref().unwrap_or_else(|| PeerTicketPublicKey::default_instance())
    }

    fn get_public_key_for_reflect(&self) -> &::protobuf::SingularPtrField<PeerTicketPublicKey> {
        &self.public_key
    }

    fn mut_public_key_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PeerTicketPublicKey> {
        &mut self.public_key
    }

    // optional .PeerTicketOld old_ticket = 20;

    pub fn clear_old_ticket(&mut self) {
        self.old_ticket.clear();
    }

    pub fn has_old_ticket(&self) -> bool {
        self.old_ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_ticket(&mut self, v: PeerTicketOld) {
        self.old_ticket = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_old_ticket(&mut self) -> &mut PeerTicketOld {
        if self.old_ticket.is_none() {
            self.old_ticket.set_default();
        }
        self.old_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_ticket(&mut self) -> PeerTicketOld {
        self.old_ticket.take().unwrap_or_else(|| PeerTicketOld::new())
    }

    pub fn get_old_ticket(&self) -> &PeerTicketOld {
        self.old_ticket.as_ref().unwrap_or_else(|| PeerTicketOld::default_instance())
    }

    fn get_old_ticket_for_reflect(&self) -> &::protobuf::SingularPtrField<PeerTicketOld> {
        &self.old_ticket
    }

    fn mut_old_ticket_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PeerTicketOld> {
        &mut self.old_ticket
    }
}

impl ::protobuf::Message for PeerTicketUnion {
    fn is_initialized(&self) -> bool {
        for v in &self.public_key {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.old_ticket {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.public_key)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.old_ticket)?;
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
        if let Some(ref v) = self.public_key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.old_ticket.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.public_key.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.old_ticket.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for PeerTicketUnion {
    fn new() -> PeerTicketUnion {
        PeerTicketUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerTicketUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PeerTicketPublicKey>>(
                    "public_key",
                    PeerTicketUnion::get_public_key_for_reflect,
                    PeerTicketUnion::mut_public_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PeerTicketOld>>(
                    "old_ticket",
                    PeerTicketUnion::get_old_ticket_for_reflect,
                    PeerTicketUnion::mut_old_ticket_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerTicketUnion>(
                    "PeerTicketUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerTicketUnion {
    fn clear(&mut self) {
        self.clear_public_key();
        self.clear_old_ticket();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PeerTicketUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PeerTicketUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PeerTicketPublicKey {
    // message fields
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PeerTicketPublicKey {}

impl PeerTicketPublicKey {
    pub fn new() -> PeerTicketPublicKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerTicketPublicKey {
        static mut instance: ::protobuf::lazy::Lazy<PeerTicketPublicKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerTicketPublicKey,
        };
        unsafe {
            instance.get(PeerTicketPublicKey::new)
        }
    }

    // required bytes public_key = 10;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.public_key.is_none() {
            self.public_key.set_default();
        }
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        self.public_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_public_key(&self) -> &[u8] {
        match self.public_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_public_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.public_key
    }

    fn mut_public_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.public_key
    }
}

impl ::protobuf::Message for PeerTicketPublicKey {
    fn is_initialized(&self) -> bool {
        if self.public_key.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.public_key)?;
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
        if let Some(ref v) = self.public_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.public_key.as_ref() {
            os.write_bytes(10, &v)?;
        }
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

impl ::protobuf::MessageStatic for PeerTicketPublicKey {
    fn new() -> PeerTicketPublicKey {
        PeerTicketPublicKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerTicketPublicKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    PeerTicketPublicKey::get_public_key_for_reflect,
                    PeerTicketPublicKey::mut_public_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerTicketPublicKey>(
                    "PeerTicketPublicKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerTicketPublicKey {
    fn clear(&mut self) {
        self.clear_public_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PeerTicketPublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PeerTicketPublicKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PeerTicketOld {
    // message fields
    peer_ticket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    peer_ticket_signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PeerTicketOld {}

impl PeerTicketOld {
    pub fn new() -> PeerTicketOld {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerTicketOld {
        static mut instance: ::protobuf::lazy::Lazy<PeerTicketOld> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerTicketOld,
        };
        unsafe {
            instance.get(PeerTicketOld::new)
        }
    }

    // required bytes peer_ticket = 10;

    pub fn clear_peer_ticket(&mut self) {
        self.peer_ticket.clear();
    }

    pub fn has_peer_ticket(&self) -> bool {
        self.peer_ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer_ticket(&mut self, v: ::std::vec::Vec<u8>) {
        self.peer_ticket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer_ticket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.peer_ticket.is_none() {
            self.peer_ticket.set_default();
        }
        self.peer_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer_ticket(&mut self) -> ::std::vec::Vec<u8> {
        self.peer_ticket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_peer_ticket(&self) -> &[u8] {
        match self.peer_ticket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_peer_ticket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.peer_ticket
    }

    fn mut_peer_ticket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.peer_ticket
    }

    // required bytes peer_ticket_signature = 20;

    pub fn clear_peer_ticket_signature(&mut self) {
        self.peer_ticket_signature.clear();
    }

    pub fn has_peer_ticket_signature(&self) -> bool {
        self.peer_ticket_signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer_ticket_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.peer_ticket_signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer_ticket_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.peer_ticket_signature.is_none() {
            self.peer_ticket_signature.set_default();
        }
        self.peer_ticket_signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer_ticket_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.peer_ticket_signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_peer_ticket_signature(&self) -> &[u8] {
        match self.peer_ticket_signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_peer_ticket_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.peer_ticket_signature
    }

    fn mut_peer_ticket_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.peer_ticket_signature
    }
}

impl ::protobuf::Message for PeerTicketOld {
    fn is_initialized(&self) -> bool {
        if self.peer_ticket.is_none() {
            return false;
        }
        if self.peer_ticket_signature.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.peer_ticket)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.peer_ticket_signature)?;
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
        if let Some(ref v) = self.peer_ticket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        if let Some(ref v) = self.peer_ticket_signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(20, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.peer_ticket.as_ref() {
            os.write_bytes(10, &v)?;
        }
        if let Some(ref v) = self.peer_ticket_signature.as_ref() {
            os.write_bytes(20, &v)?;
        }
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

impl ::protobuf::MessageStatic for PeerTicketOld {
    fn new() -> PeerTicketOld {
        PeerTicketOld::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerTicketOld>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "peer_ticket",
                    PeerTicketOld::get_peer_ticket_for_reflect,
                    PeerTicketOld::mut_peer_ticket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "peer_ticket_signature",
                    PeerTicketOld::get_peer_ticket_signature_for_reflect,
                    PeerTicketOld::mut_peer_ticket_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerTicketOld>(
                    "PeerTicketOld",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerTicketOld {
    fn clear(&mut self) {
        self.clear_peer_ticket();
        self.clear_peer_ticket_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PeerTicketOld {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PeerTicketOld {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SystemInfo {
    // message fields
    cpu_family: ::std::option::Option<CpuFamily>,
    cpu_subtype: ::std::option::Option<u32>,
    cpu_ext: ::std::option::Option<u32>,
    brand: ::std::option::Option<Brand>,
    brand_flags: ::std::option::Option<u32>,
    os: ::std::option::Option<Os>,
    os_version: ::std::option::Option<u32>,
    os_ext: ::std::option::Option<u32>,
    system_information_string: ::protobuf::SingularField<::std::string::String>,
    device_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SystemInfo {}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SystemInfo {
        static mut instance: ::protobuf::lazy::Lazy<SystemInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SystemInfo,
        };
        unsafe {
            instance.get(SystemInfo::new)
        }
    }

    // required .CpuFamily cpu_family = 10;

    pub fn clear_cpu_family(&mut self) {
        self.cpu_family = ::std::option::Option::None;
    }

    pub fn has_cpu_family(&self) -> bool {
        self.cpu_family.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_family(&mut self, v: CpuFamily) {
        self.cpu_family = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_family(&self) -> CpuFamily {
        self.cpu_family.unwrap_or(CpuFamily::CPU_UNKNOWN)
    }

    fn get_cpu_family_for_reflect(&self) -> &::std::option::Option<CpuFamily> {
        &self.cpu_family
    }

    fn mut_cpu_family_for_reflect(&mut self) -> &mut ::std::option::Option<CpuFamily> {
        &mut self.cpu_family
    }

    // optional uint32 cpu_subtype = 20;

    pub fn clear_cpu_subtype(&mut self) {
        self.cpu_subtype = ::std::option::Option::None;
    }

    pub fn has_cpu_subtype(&self) -> bool {
        self.cpu_subtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_subtype(&mut self, v: u32) {
        self.cpu_subtype = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_subtype(&self) -> u32 {
        self.cpu_subtype.unwrap_or(0)
    }

    fn get_cpu_subtype_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.cpu_subtype
    }

    fn mut_cpu_subtype_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.cpu_subtype
    }

    // optional uint32 cpu_ext = 30;

    pub fn clear_cpu_ext(&mut self) {
        self.cpu_ext = ::std::option::Option::None;
    }

    pub fn has_cpu_ext(&self) -> bool {
        self.cpu_ext.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_ext(&mut self, v: u32) {
        self.cpu_ext = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_ext(&self) -> u32 {
        self.cpu_ext.unwrap_or(0)
    }

    fn get_cpu_ext_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.cpu_ext
    }

    fn mut_cpu_ext_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.cpu_ext
    }

    // optional .Brand brand = 40;

    pub fn clear_brand(&mut self) {
        self.brand = ::std::option::Option::None;
    }

    pub fn has_brand(&self) -> bool {
        self.brand.is_some()
    }

    // Param is passed by value, moved
    pub fn set_brand(&mut self, v: Brand) {
        self.brand = ::std::option::Option::Some(v);
    }

    pub fn get_brand(&self) -> Brand {
        self.brand.unwrap_or(Brand::BRAND_UNBRANDED)
    }

    fn get_brand_for_reflect(&self) -> &::std::option::Option<Brand> {
        &self.brand
    }

    fn mut_brand_for_reflect(&mut self) -> &mut ::std::option::Option<Brand> {
        &mut self.brand
    }

    // optional uint32 brand_flags = 50;

    pub fn clear_brand_flags(&mut self) {
        self.brand_flags = ::std::option::Option::None;
    }

    pub fn has_brand_flags(&self) -> bool {
        self.brand_flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_brand_flags(&mut self, v: u32) {
        self.brand_flags = ::std::option::Option::Some(v);
    }

    pub fn get_brand_flags(&self) -> u32 {
        self.brand_flags.unwrap_or(0)
    }

    fn get_brand_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.brand_flags
    }

    fn mut_brand_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.brand_flags
    }

    // required .Os os = 60;

    pub fn clear_os(&mut self) {
        self.os = ::std::option::Option::None;
    }

    pub fn has_os(&self) -> bool {
        self.os.is_some()
    }

    // Param is passed by value, moved
    pub fn set_os(&mut self, v: Os) {
        self.os = ::std::option::Option::Some(v);
    }

    pub fn get_os(&self) -> Os {
        self.os.unwrap_or(Os::OS_UNKNOWN)
    }

    fn get_os_for_reflect(&self) -> &::std::option::Option<Os> {
        &self.os
    }

    fn mut_os_for_reflect(&mut self) -> &mut ::std::option::Option<Os> {
        &mut self.os
    }

    // optional uint32 os_version = 70;

    pub fn clear_os_version(&mut self) {
        self.os_version = ::std::option::Option::None;
    }

    pub fn has_os_version(&self) -> bool {
        self.os_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_os_version(&mut self, v: u32) {
        self.os_version = ::std::option::Option::Some(v);
    }

    pub fn get_os_version(&self) -> u32 {
        self.os_version.unwrap_or(0)
    }

    fn get_os_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.os_version
    }

    fn mut_os_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.os_version
    }

    // optional uint32 os_ext = 80;

    pub fn clear_os_ext(&mut self) {
        self.os_ext = ::std::option::Option::None;
    }

    pub fn has_os_ext(&self) -> bool {
        self.os_ext.is_some()
    }

    // Param is passed by value, moved
    pub fn set_os_ext(&mut self, v: u32) {
        self.os_ext = ::std::option::Option::Some(v);
    }

    pub fn get_os_ext(&self) -> u32 {
        self.os_ext.unwrap_or(0)
    }

    fn get_os_ext_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.os_ext
    }

    fn mut_os_ext_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.os_ext
    }

    // optional string system_information_string = 90;

    pub fn clear_system_information_string(&mut self) {
        self.system_information_string.clear();
    }

    pub fn has_system_information_string(&self) -> bool {
        self.system_information_string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_system_information_string(&mut self, v: ::std::string::String) {
        self.system_information_string = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_system_information_string(&mut self) -> &mut ::std::string::String {
        if self.system_information_string.is_none() {
            self.system_information_string.set_default();
        }
        self.system_information_string.as_mut().unwrap()
    }

    // Take field
    pub fn take_system_information_string(&mut self) -> ::std::string::String {
        self.system_information_string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_system_information_string(&self) -> &str {
        match self.system_information_string.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_system_information_string_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.system_information_string
    }

    fn mut_system_information_string_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.system_information_string
    }

    // optional string device_id = 100;

    pub fn clear_device_id(&mut self) {
        self.device_id.clear();
    }

    pub fn has_device_id(&self) -> bool {
        self.device_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_id(&mut self, v: ::std::string::String) {
        self.device_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_id(&mut self) -> &mut ::std::string::String {
        if self.device_id.is_none() {
            self.device_id.set_default();
        }
        self.device_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_id(&mut self) -> ::std::string::String {
        self.device_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_device_id(&self) -> &str {
        match self.device_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_device_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.device_id
    }

    fn mut_device_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.device_id
    }
}

impl ::protobuf::Message for SystemInfo {
    fn is_initialized(&self) -> bool {
        if self.cpu_family.is_none() {
            return false;
        }
        if self.os.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.cpu_family = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cpu_subtype = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cpu_ext = ::std::option::Option::Some(tmp);
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.brand = ::std::option::Option::Some(tmp);
                },
                50 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.brand_flags = ::std::option::Option::Some(tmp);
                },
                60 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.os = ::std::option::Option::Some(tmp);
                },
                70 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.os_version = ::std::option::Option::Some(tmp);
                },
                80 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.os_ext = ::std::option::Option::Some(tmp);
                },
                90 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.system_information_string)?;
                },
                100 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_id)?;
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
        if let Some(v) = self.cpu_family {
            my_size += ::protobuf::rt::enum_size(10, v);
        }
        if let Some(v) = self.cpu_subtype {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cpu_ext {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.brand {
            my_size += ::protobuf::rt::enum_size(40, v);
        }
        if let Some(v) = self.brand_flags {
            my_size += ::protobuf::rt::value_size(50, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.os {
            my_size += ::protobuf::rt::enum_size(60, v);
        }
        if let Some(v) = self.os_version {
            my_size += ::protobuf::rt::value_size(70, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.os_ext {
            my_size += ::protobuf::rt::value_size(80, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.system_information_string.as_ref() {
            my_size += ::protobuf::rt::string_size(90, &v);
        }
        if let Some(ref v) = self.device_id.as_ref() {
            my_size += ::protobuf::rt::string_size(100, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cpu_family {
            os.write_enum(10, v.value())?;
        }
        if let Some(v) = self.cpu_subtype {
            os.write_uint32(20, v)?;
        }
        if let Some(v) = self.cpu_ext {
            os.write_uint32(30, v)?;
        }
        if let Some(v) = self.brand {
            os.write_enum(40, v.value())?;
        }
        if let Some(v) = self.brand_flags {
            os.write_uint32(50, v)?;
        }
        if let Some(v) = self.os {
            os.write_enum(60, v.value())?;
        }
        if let Some(v) = self.os_version {
            os.write_uint32(70, v)?;
        }
        if let Some(v) = self.os_ext {
            os.write_uint32(80, v)?;
        }
        if let Some(ref v) = self.system_information_string.as_ref() {
            os.write_string(90, &v)?;
        }
        if let Some(ref v) = self.device_id.as_ref() {
            os.write_string(100, &v)?;
        }
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

impl ::protobuf::MessageStatic for SystemInfo {
    fn new() -> SystemInfo {
        SystemInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<SystemInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CpuFamily>>(
                    "cpu_family",
                    SystemInfo::get_cpu_family_for_reflect,
                    SystemInfo::mut_cpu_family_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cpu_subtype",
                    SystemInfo::get_cpu_subtype_for_reflect,
                    SystemInfo::mut_cpu_subtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cpu_ext",
                    SystemInfo::get_cpu_ext_for_reflect,
                    SystemInfo::mut_cpu_ext_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Brand>>(
                    "brand",
                    SystemInfo::get_brand_for_reflect,
                    SystemInfo::mut_brand_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "brand_flags",
                    SystemInfo::get_brand_flags_for_reflect,
                    SystemInfo::mut_brand_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Os>>(
                    "os",
                    SystemInfo::get_os_for_reflect,
                    SystemInfo::mut_os_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "os_version",
                    SystemInfo::get_os_version_for_reflect,
                    SystemInfo::mut_os_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "os_ext",
                    SystemInfo::get_os_ext_for_reflect,
                    SystemInfo::mut_os_ext_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "system_information_string",
                    SystemInfo::get_system_information_string_for_reflect,
                    SystemInfo::mut_system_information_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device_id",
                    SystemInfo::get_device_id_for_reflect,
                    SystemInfo::mut_device_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SystemInfo>(
                    "SystemInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SystemInfo {
    fn clear(&mut self) {
        self.clear_cpu_family();
        self.clear_cpu_subtype();
        self.clear_cpu_ext();
        self.clear_brand();
        self.clear_brand_flags();
        self.clear_os();
        self.clear_os_version();
        self.clear_os_ext();
        self.clear_system_information_string();
        self.clear_device_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SystemInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SystemInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LibspotifyAppKey {
    // message fields
    version: ::std::option::Option<u32>,
    devkey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    useragent: ::protobuf::SingularField<::std::string::String>,
    callback_hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LibspotifyAppKey {}

impl LibspotifyAppKey {
    pub fn new() -> LibspotifyAppKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LibspotifyAppKey {
        static mut instance: ::protobuf::lazy::Lazy<LibspotifyAppKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LibspotifyAppKey,
        };
        unsafe {
            instance.get(LibspotifyAppKey::new)
        }
    }

    // required uint32 version = 1;

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

    // required bytes devkey = 2;

    pub fn clear_devkey(&mut self) {
        self.devkey.clear();
    }

    pub fn has_devkey(&self) -> bool {
        self.devkey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_devkey(&mut self, v: ::std::vec::Vec<u8>) {
        self.devkey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_devkey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.devkey.is_none() {
            self.devkey.set_default();
        }
        self.devkey.as_mut().unwrap()
    }

    // Take field
    pub fn take_devkey(&mut self) -> ::std::vec::Vec<u8> {
        self.devkey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_devkey(&self) -> &[u8] {
        match self.devkey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_devkey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.devkey
    }

    fn mut_devkey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.devkey
    }

    // required bytes signature = 3;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }

    // required string useragent = 4;

    pub fn clear_useragent(&mut self) {
        self.useragent.clear();
    }

    pub fn has_useragent(&self) -> bool {
        self.useragent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_useragent(&mut self, v: ::std::string::String) {
        self.useragent = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_useragent(&mut self) -> &mut ::std::string::String {
        if self.useragent.is_none() {
            self.useragent.set_default();
        }
        self.useragent.as_mut().unwrap()
    }

    // Take field
    pub fn take_useragent(&mut self) -> ::std::string::String {
        self.useragent.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_useragent(&self) -> &str {
        match self.useragent.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_useragent_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.useragent
    }

    fn mut_useragent_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.useragent
    }

    // required bytes callback_hash = 5;

    pub fn clear_callback_hash(&mut self) {
        self.callback_hash.clear();
    }

    pub fn has_callback_hash(&self) -> bool {
        self.callback_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_callback_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.callback_hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_callback_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.callback_hash.is_none() {
            self.callback_hash.set_default();
        }
        self.callback_hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_callback_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.callback_hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_callback_hash(&self) -> &[u8] {
        match self.callback_hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_callback_hash_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.callback_hash
    }

    fn mut_callback_hash_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.callback_hash
    }
}

impl ::protobuf::Message for LibspotifyAppKey {
    fn is_initialized(&self) -> bool {
        if self.version.is_none() {
            return false;
        }
        if self.devkey.is_none() {
            return false;
        }
        if self.signature.is_none() {
            return false;
        }
        if self.useragent.is_none() {
            return false;
        }
        if self.callback_hash.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.devkey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.useragent)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.callback_hash)?;
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
        }
        if let Some(ref v) = self.devkey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.useragent.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.callback_hash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.devkey.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.signature.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.useragent.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.callback_hash.as_ref() {
            os.write_bytes(5, &v)?;
        }
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

impl ::protobuf::MessageStatic for LibspotifyAppKey {
    fn new() -> LibspotifyAppKey {
        LibspotifyAppKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<LibspotifyAppKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    LibspotifyAppKey::get_version_for_reflect,
                    LibspotifyAppKey::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "devkey",
                    LibspotifyAppKey::get_devkey_for_reflect,
                    LibspotifyAppKey::mut_devkey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    LibspotifyAppKey::get_signature_for_reflect,
                    LibspotifyAppKey::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "useragent",
                    LibspotifyAppKey::get_useragent_for_reflect,
                    LibspotifyAppKey::mut_useragent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "callback_hash",
                    LibspotifyAppKey::get_callback_hash_for_reflect,
                    LibspotifyAppKey::mut_callback_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LibspotifyAppKey>(
                    "LibspotifyAppKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LibspotifyAppKey {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_devkey();
        self.clear_signature();
        self.clear_useragent();
        self.clear_callback_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LibspotifyAppKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LibspotifyAppKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientInfo {
    // message fields
    limited: ::std::option::Option<bool>,
    fb: ::protobuf::SingularPtrField<ClientInfoFacebook>,
    language: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientInfo {}

impl ClientInfo {
    pub fn new() -> ClientInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientInfo {
        static mut instance: ::protobuf::lazy::Lazy<ClientInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientInfo,
        };
        unsafe {
            instance.get(ClientInfo::new)
        }
    }

    // optional bool limited = 1;

    pub fn clear_limited(&mut self) {
        self.limited = ::std::option::Option::None;
    }

    pub fn has_limited(&self) -> bool {
        self.limited.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limited(&mut self, v: bool) {
        self.limited = ::std::option::Option::Some(v);
    }

    pub fn get_limited(&self) -> bool {
        self.limited.unwrap_or(false)
    }

    fn get_limited_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.limited
    }

    fn mut_limited_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.limited
    }

    // optional .ClientInfoFacebook fb = 2;

    pub fn clear_fb(&mut self) {
        self.fb.clear();
    }

    pub fn has_fb(&self) -> bool {
        self.fb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fb(&mut self, v: ClientInfoFacebook) {
        self.fb = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fb(&mut self) -> &mut ClientInfoFacebook {
        if self.fb.is_none() {
            self.fb.set_default();
        }
        self.fb.as_mut().unwrap()
    }

    // Take field
    pub fn take_fb(&mut self) -> ClientInfoFacebook {
        self.fb.take().unwrap_or_else(|| ClientInfoFacebook::new())
    }

    pub fn get_fb(&self) -> &ClientInfoFacebook {
        self.fb.as_ref().unwrap_or_else(|| ClientInfoFacebook::default_instance())
    }

    fn get_fb_for_reflect(&self) -> &::protobuf::SingularPtrField<ClientInfoFacebook> {
        &self.fb
    }

    fn mut_fb_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClientInfoFacebook> {
        &mut self.fb
    }

    // optional string language = 3;

    pub fn clear_language(&mut self) {
        self.language.clear();
    }

    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    // Param is passed by value, moved
    pub fn set_language(&mut self, v: ::std::string::String) {
        self.language = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_language(&mut self) -> &mut ::std::string::String {
        if self.language.is_none() {
            self.language.set_default();
        }
        self.language.as_mut().unwrap()
    }

    // Take field
    pub fn take_language(&mut self) -> ::std::string::String {
        self.language.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_language(&self) -> &str {
        match self.language.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_language_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.language
    }

    fn mut_language_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.language
    }
}

impl ::protobuf::Message for ClientInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.fb {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.limited = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fb)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.language)?;
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
        if let Some(v) = self.limited {
            my_size += 2;
        }
        if let Some(ref v) = self.fb.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.language.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.limited {
            os.write_bool(1, v)?;
        }
        if let Some(ref v) = self.fb.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.language.as_ref() {
            os.write_string(3, &v)?;
        }
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

impl ::protobuf::MessageStatic for ClientInfo {
    fn new() -> ClientInfo {
        ClientInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "limited",
                    ClientInfo::get_limited_for_reflect,
                    ClientInfo::mut_limited_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientInfoFacebook>>(
                    "fb",
                    ClientInfo::get_fb_for_reflect,
                    ClientInfo::mut_fb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "language",
                    ClientInfo::get_language_for_reflect,
                    ClientInfo::mut_language_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientInfo>(
                    "ClientInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientInfo {
    fn clear(&mut self) {
        self.clear_limited();
        self.clear_fb();
        self.clear_language();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientInfoFacebook {
    // message fields
    machine_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientInfoFacebook {}

impl ClientInfoFacebook {
    pub fn new() -> ClientInfoFacebook {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientInfoFacebook {
        static mut instance: ::protobuf::lazy::Lazy<ClientInfoFacebook> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientInfoFacebook,
        };
        unsafe {
            instance.get(ClientInfoFacebook::new)
        }
    }

    // optional string machine_id = 1;

    pub fn clear_machine_id(&mut self) {
        self.machine_id.clear();
    }

    pub fn has_machine_id(&self) -> bool {
        self.machine_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_machine_id(&mut self, v: ::std::string::String) {
        self.machine_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_machine_id(&mut self) -> &mut ::std::string::String {
        if self.machine_id.is_none() {
            self.machine_id.set_default();
        }
        self.machine_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_machine_id(&mut self) -> ::std::string::String {
        self.machine_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_machine_id(&self) -> &str {
        match self.machine_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_machine_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.machine_id
    }

    fn mut_machine_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.machine_id
    }
}

impl ::protobuf::Message for ClientInfoFacebook {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.machine_id)?;
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
        if let Some(ref v) = self.machine_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.machine_id.as_ref() {
            os.write_string(1, &v)?;
        }
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

impl ::protobuf::MessageStatic for ClientInfoFacebook {
    fn new() -> ClientInfoFacebook {
        ClientInfoFacebook::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientInfoFacebook>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "machine_id",
                    ClientInfoFacebook::get_machine_id_for_reflect,
                    ClientInfoFacebook::mut_machine_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientInfoFacebook>(
                    "ClientInfoFacebook",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientInfoFacebook {
    fn clear(&mut self) {
        self.clear_machine_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientInfoFacebook {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientInfoFacebook {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct APWelcome {
    // message fields
    canonical_username: ::protobuf::SingularField<::std::string::String>,
    account_type_logged_in: ::std::option::Option<AccountType>,
    credentials_type_logged_in: ::std::option::Option<AccountType>,
    reusable_auth_credentials_type: ::std::option::Option<AuthenticationType>,
    reusable_auth_credentials: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lfs_secret: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    account_info: ::protobuf::SingularPtrField<AccountInfo>,
    fb: ::protobuf::SingularPtrField<AccountInfoFacebook>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for APWelcome {}

impl APWelcome {
    pub fn new() -> APWelcome {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static APWelcome {
        static mut instance: ::protobuf::lazy::Lazy<APWelcome> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const APWelcome,
        };
        unsafe {
            instance.get(APWelcome::new)
        }
    }

    // required string canonical_username = 10;

    pub fn clear_canonical_username(&mut self) {
        self.canonical_username.clear();
    }

    pub fn has_canonical_username(&self) -> bool {
        self.canonical_username.is_some()
    }

    // Param is passed by value, moved
    pub fn set_canonical_username(&mut self, v: ::std::string::String) {
        self.canonical_username = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_canonical_username(&mut self) -> &mut ::std::string::String {
        if self.canonical_username.is_none() {
            self.canonical_username.set_default();
        }
        self.canonical_username.as_mut().unwrap()
    }

    // Take field
    pub fn take_canonical_username(&mut self) -> ::std::string::String {
        self.canonical_username.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_canonical_username(&self) -> &str {
        match self.canonical_username.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_canonical_username_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.canonical_username
    }

    fn mut_canonical_username_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.canonical_username
    }

    // required .AccountType account_type_logged_in = 20;

    pub fn clear_account_type_logged_in(&mut self) {
        self.account_type_logged_in = ::std::option::Option::None;
    }

    pub fn has_account_type_logged_in(&self) -> bool {
        self.account_type_logged_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_type_logged_in(&mut self, v: AccountType) {
        self.account_type_logged_in = ::std::option::Option::Some(v);
    }

    pub fn get_account_type_logged_in(&self) -> AccountType {
        self.account_type_logged_in.unwrap_or(AccountType::Spotify)
    }

    fn get_account_type_logged_in_for_reflect(&self) -> &::std::option::Option<AccountType> {
        &self.account_type_logged_in
    }

    fn mut_account_type_logged_in_for_reflect(&mut self) -> &mut ::std::option::Option<AccountType> {
        &mut self.account_type_logged_in
    }

    // required .AccountType credentials_type_logged_in = 25;

    pub fn clear_credentials_type_logged_in(&mut self) {
        self.credentials_type_logged_in = ::std::option::Option::None;
    }

    pub fn has_credentials_type_logged_in(&self) -> bool {
        self.credentials_type_logged_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_credentials_type_logged_in(&mut self, v: AccountType) {
        self.credentials_type_logged_in = ::std::option::Option::Some(v);
    }

    pub fn get_credentials_type_logged_in(&self) -> AccountType {
        self.credentials_type_logged_in.unwrap_or(AccountType::Spotify)
    }

    fn get_credentials_type_logged_in_for_reflect(&self) -> &::std::option::Option<AccountType> {
        &self.credentials_type_logged_in
    }

    fn mut_credentials_type_logged_in_for_reflect(&mut self) -> &mut ::std::option::Option<AccountType> {
        &mut self.credentials_type_logged_in
    }

    // required .AuthenticationType reusable_auth_credentials_type = 30;

    pub fn clear_reusable_auth_credentials_type(&mut self) {
        self.reusable_auth_credentials_type = ::std::option::Option::None;
    }

    pub fn has_reusable_auth_credentials_type(&self) -> bool {
        self.reusable_auth_credentials_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reusable_auth_credentials_type(&mut self, v: AuthenticationType) {
        self.reusable_auth_credentials_type = ::std::option::Option::Some(v);
    }

    pub fn get_reusable_auth_credentials_type(&self) -> AuthenticationType {
        self.reusable_auth_credentials_type.unwrap_or(AuthenticationType::AUTHENTICATION_USER_PASS)
    }

    fn get_reusable_auth_credentials_type_for_reflect(&self) -> &::std::option::Option<AuthenticationType> {
        &self.reusable_auth_credentials_type
    }

    fn mut_reusable_auth_credentials_type_for_reflect(&mut self) -> &mut ::std::option::Option<AuthenticationType> {
        &mut self.reusable_auth_credentials_type
    }

    // required bytes reusable_auth_credentials = 40;

    pub fn clear_reusable_auth_credentials(&mut self) {
        self.reusable_auth_credentials.clear();
    }

    pub fn has_reusable_auth_credentials(&self) -> bool {
        self.reusable_auth_credentials.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reusable_auth_credentials(&mut self, v: ::std::vec::Vec<u8>) {
        self.reusable_auth_credentials = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reusable_auth_credentials(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.reusable_auth_credentials.is_none() {
            self.reusable_auth_credentials.set_default();
        }
        self.reusable_auth_credentials.as_mut().unwrap()
    }

    // Take field
    pub fn take_reusable_auth_credentials(&mut self) -> ::std::vec::Vec<u8> {
        self.reusable_auth_credentials.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_reusable_auth_credentials(&self) -> &[u8] {
        match self.reusable_auth_credentials.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_reusable_auth_credentials_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.reusable_auth_credentials
    }

    fn mut_reusable_auth_credentials_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.reusable_auth_credentials
    }

    // optional bytes lfs_secret = 50;

    pub fn clear_lfs_secret(&mut self) {
        self.lfs_secret.clear();
    }

    pub fn has_lfs_secret(&self) -> bool {
        self.lfs_secret.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lfs_secret(&mut self, v: ::std::vec::Vec<u8>) {
        self.lfs_secret = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lfs_secret(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.lfs_secret.is_none() {
            self.lfs_secret.set_default();
        }
        self.lfs_secret.as_mut().unwrap()
    }

    // Take field
    pub fn take_lfs_secret(&mut self) -> ::std::vec::Vec<u8> {
        self.lfs_secret.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_lfs_secret(&self) -> &[u8] {
        match self.lfs_secret.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_lfs_secret_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.lfs_secret
    }

    fn mut_lfs_secret_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.lfs_secret
    }

    // optional .AccountInfo account_info = 60;

    pub fn clear_account_info(&mut self) {
        self.account_info.clear();
    }

    pub fn has_account_info(&self) -> bool {
        self.account_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_info(&mut self, v: AccountInfo) {
        self.account_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_info(&mut self) -> &mut AccountInfo {
        if self.account_info.is_none() {
            self.account_info.set_default();
        }
        self.account_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_info(&mut self) -> AccountInfo {
        self.account_info.take().unwrap_or_else(|| AccountInfo::new())
    }

    pub fn get_account_info(&self) -> &AccountInfo {
        self.account_info.as_ref().unwrap_or_else(|| AccountInfo::default_instance())
    }

    fn get_account_info_for_reflect(&self) -> &::protobuf::SingularPtrField<AccountInfo> {
        &self.account_info
    }

    fn mut_account_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AccountInfo> {
        &mut self.account_info
    }

    // optional .AccountInfoFacebook fb = 70;

    pub fn clear_fb(&mut self) {
        self.fb.clear();
    }

    pub fn has_fb(&self) -> bool {
        self.fb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fb(&mut self, v: AccountInfoFacebook) {
        self.fb = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fb(&mut self) -> &mut AccountInfoFacebook {
        if self.fb.is_none() {
            self.fb.set_default();
        }
        self.fb.as_mut().unwrap()
    }

    // Take field
    pub fn take_fb(&mut self) -> AccountInfoFacebook {
        self.fb.take().unwrap_or_else(|| AccountInfoFacebook::new())
    }

    pub fn get_fb(&self) -> &AccountInfoFacebook {
        self.fb.as_ref().unwrap_or_else(|| AccountInfoFacebook::default_instance())
    }

    fn get_fb_for_reflect(&self) -> &::protobuf::SingularPtrField<AccountInfoFacebook> {
        &self.fb
    }

    fn mut_fb_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AccountInfoFacebook> {
        &mut self.fb
    }
}

impl ::protobuf::Message for APWelcome {
    fn is_initialized(&self) -> bool {
        if self.canonical_username.is_none() {
            return false;
        }
        if self.account_type_logged_in.is_none() {
            return false;
        }
        if self.credentials_type_logged_in.is_none() {
            return false;
        }
        if self.reusable_auth_credentials_type.is_none() {
            return false;
        }
        if self.reusable_auth_credentials.is_none() {
            return false;
        }
        for v in &self.account_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fb {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.canonical_username)?;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.account_type_logged_in = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.credentials_type_logged_in = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.reusable_auth_credentials_type = ::std::option::Option::Some(tmp);
                },
                40 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.reusable_auth_credentials)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.lfs_secret)?;
                },
                60 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.account_info)?;
                },
                70 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fb)?;
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
        if let Some(ref v) = self.canonical_username.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(v) = self.account_type_logged_in {
            my_size += ::protobuf::rt::enum_size(20, v);
        }
        if let Some(v) = self.credentials_type_logged_in {
            my_size += ::protobuf::rt::enum_size(25, v);
        }
        if let Some(v) = self.reusable_auth_credentials_type {
            my_size += ::protobuf::rt::enum_size(30, v);
        }
        if let Some(ref v) = self.reusable_auth_credentials.as_ref() {
            my_size += ::protobuf::rt::bytes_size(40, &v);
        }
        if let Some(ref v) = self.lfs_secret.as_ref() {
            my_size += ::protobuf::rt::bytes_size(50, &v);
        }
        if let Some(ref v) = self.account_info.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.fb.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.canonical_username.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(v) = self.account_type_logged_in {
            os.write_enum(20, v.value())?;
        }
        if let Some(v) = self.credentials_type_logged_in {
            os.write_enum(25, v.value())?;
        }
        if let Some(v) = self.reusable_auth_credentials_type {
            os.write_enum(30, v.value())?;
        }
        if let Some(ref v) = self.reusable_auth_credentials.as_ref() {
            os.write_bytes(40, &v)?;
        }
        if let Some(ref v) = self.lfs_secret.as_ref() {
            os.write_bytes(50, &v)?;
        }
        if let Some(ref v) = self.account_info.as_ref() {
            os.write_tag(60, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.fb.as_ref() {
            os.write_tag(70, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for APWelcome {
    fn new() -> APWelcome {
        APWelcome::new()
    }

    fn descriptor_static(_: ::std::option::Option<APWelcome>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "canonical_username",
                    APWelcome::get_canonical_username_for_reflect,
                    APWelcome::mut_canonical_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AccountType>>(
                    "account_type_logged_in",
                    APWelcome::get_account_type_logged_in_for_reflect,
                    APWelcome::mut_account_type_logged_in_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AccountType>>(
                    "credentials_type_logged_in",
                    APWelcome::get_credentials_type_logged_in_for_reflect,
                    APWelcome::mut_credentials_type_logged_in_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AuthenticationType>>(
                    "reusable_auth_credentials_type",
                    APWelcome::get_reusable_auth_credentials_type_for_reflect,
                    APWelcome::mut_reusable_auth_credentials_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "reusable_auth_credentials",
                    APWelcome::get_reusable_auth_credentials_for_reflect,
                    APWelcome::mut_reusable_auth_credentials_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "lfs_secret",
                    APWelcome::get_lfs_secret_for_reflect,
                    APWelcome::mut_lfs_secret_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AccountInfo>>(
                    "account_info",
                    APWelcome::get_account_info_for_reflect,
                    APWelcome::mut_account_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AccountInfoFacebook>>(
                    "fb",
                    APWelcome::get_fb_for_reflect,
                    APWelcome::mut_fb_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<APWelcome>(
                    "APWelcome",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for APWelcome {
    fn clear(&mut self) {
        self.clear_canonical_username();
        self.clear_account_type_logged_in();
        self.clear_credentials_type_logged_in();
        self.clear_reusable_auth_credentials_type();
        self.clear_reusable_auth_credentials();
        self.clear_lfs_secret();
        self.clear_account_info();
        self.clear_fb();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for APWelcome {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for APWelcome {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountInfo {
    // message fields
    spotify: ::protobuf::SingularPtrField<AccountInfoSpotify>,
    facebook: ::protobuf::SingularPtrField<AccountInfoFacebook>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInfo {}

impl AccountInfo {
    pub fn new() -> AccountInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInfo {
        static mut instance: ::protobuf::lazy::Lazy<AccountInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInfo,
        };
        unsafe {
            instance.get(AccountInfo::new)
        }
    }

    // optional .AccountInfoSpotify spotify = 1;

    pub fn clear_spotify(&mut self) {
        self.spotify.clear();
    }

    pub fn has_spotify(&self) -> bool {
        self.spotify.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spotify(&mut self, v: AccountInfoSpotify) {
        self.spotify = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spotify(&mut self) -> &mut AccountInfoSpotify {
        if self.spotify.is_none() {
            self.spotify.set_default();
        }
        self.spotify.as_mut().unwrap()
    }

    // Take field
    pub fn take_spotify(&mut self) -> AccountInfoSpotify {
        self.spotify.take().unwrap_or_else(|| AccountInfoSpotify::new())
    }

    pub fn get_spotify(&self) -> &AccountInfoSpotify {
        self.spotify.as_ref().unwrap_or_else(|| AccountInfoSpotify::default_instance())
    }

    fn get_spotify_for_reflect(&self) -> &::protobuf::SingularPtrField<AccountInfoSpotify> {
        &self.spotify
    }

    fn mut_spotify_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AccountInfoSpotify> {
        &mut self.spotify
    }

    // optional .AccountInfoFacebook facebook = 2;

    pub fn clear_facebook(&mut self) {
        self.facebook.clear();
    }

    pub fn has_facebook(&self) -> bool {
        self.facebook.is_some()
    }

    // Param is passed by value, moved
    pub fn set_facebook(&mut self, v: AccountInfoFacebook) {
        self.facebook = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_facebook(&mut self) -> &mut AccountInfoFacebook {
        if self.facebook.is_none() {
            self.facebook.set_default();
        }
        self.facebook.as_mut().unwrap()
    }

    // Take field
    pub fn take_facebook(&mut self) -> AccountInfoFacebook {
        self.facebook.take().unwrap_or_else(|| AccountInfoFacebook::new())
    }

    pub fn get_facebook(&self) -> &AccountInfoFacebook {
        self.facebook.as_ref().unwrap_or_else(|| AccountInfoFacebook::default_instance())
    }

    fn get_facebook_for_reflect(&self) -> &::protobuf::SingularPtrField<AccountInfoFacebook> {
        &self.facebook
    }

    fn mut_facebook_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AccountInfoFacebook> {
        &mut self.facebook
    }
}

impl ::protobuf::Message for AccountInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.spotify {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.facebook {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.spotify)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.facebook)?;
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
        if let Some(ref v) = self.spotify.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.facebook.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.spotify.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.facebook.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for AccountInfo {
    fn new() -> AccountInfo {
        AccountInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AccountInfoSpotify>>(
                    "spotify",
                    AccountInfo::get_spotify_for_reflect,
                    AccountInfo::mut_spotify_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AccountInfoFacebook>>(
                    "facebook",
                    AccountInfo::get_facebook_for_reflect,
                    AccountInfo::mut_facebook_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountInfo>(
                    "AccountInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInfo {
    fn clear(&mut self) {
        self.clear_spotify();
        self.clear_facebook();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountInfoSpotify {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInfoSpotify {}

impl AccountInfoSpotify {
    pub fn new() -> AccountInfoSpotify {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInfoSpotify {
        static mut instance: ::protobuf::lazy::Lazy<AccountInfoSpotify> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInfoSpotify,
        };
        unsafe {
            instance.get(AccountInfoSpotify::new)
        }
    }
}

impl ::protobuf::Message for AccountInfoSpotify {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for AccountInfoSpotify {
    fn new() -> AccountInfoSpotify {
        AccountInfoSpotify::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInfoSpotify>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<AccountInfoSpotify>(
                    "AccountInfoSpotify",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInfoSpotify {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInfoSpotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInfoSpotify {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountInfoFacebook {
    // message fields
    access_token: ::protobuf::SingularField<::std::string::String>,
    machine_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountInfoFacebook {}

impl AccountInfoFacebook {
    pub fn new() -> AccountInfoFacebook {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountInfoFacebook {
        static mut instance: ::protobuf::lazy::Lazy<AccountInfoFacebook> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountInfoFacebook,
        };
        unsafe {
            instance.get(AccountInfoFacebook::new)
        }
    }

    // optional string access_token = 1;

    pub fn clear_access_token(&mut self) {
        self.access_token.clear();
    }

    pub fn has_access_token(&self) -> bool {
        self.access_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_access_token(&mut self, v: ::std::string::String) {
        self.access_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_access_token(&mut self) -> &mut ::std::string::String {
        if self.access_token.is_none() {
            self.access_token.set_default();
        }
        self.access_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_access_token(&mut self) -> ::std::string::String {
        self.access_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_access_token(&self) -> &str {
        match self.access_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_access_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.access_token
    }

    fn mut_access_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.access_token
    }

    // optional string machine_id = 2;

    pub fn clear_machine_id(&mut self) {
        self.machine_id.clear();
    }

    pub fn has_machine_id(&self) -> bool {
        self.machine_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_machine_id(&mut self, v: ::std::string::String) {
        self.machine_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_machine_id(&mut self) -> &mut ::std::string::String {
        if self.machine_id.is_none() {
            self.machine_id.set_default();
        }
        self.machine_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_machine_id(&mut self) -> ::std::string::String {
        self.machine_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_machine_id(&self) -> &str {
        match self.machine_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_machine_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.machine_id
    }

    fn mut_machine_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.machine_id
    }
}

impl ::protobuf::Message for AccountInfoFacebook {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.access_token)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.machine_id)?;
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
        if let Some(ref v) = self.access_token.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.machine_id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.access_token.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.machine_id.as_ref() {
            os.write_string(2, &v)?;
        }
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

impl ::protobuf::MessageStatic for AccountInfoFacebook {
    fn new() -> AccountInfoFacebook {
        AccountInfoFacebook::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountInfoFacebook>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "access_token",
                    AccountInfoFacebook::get_access_token_for_reflect,
                    AccountInfoFacebook::mut_access_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "machine_id",
                    AccountInfoFacebook::get_machine_id_for_reflect,
                    AccountInfoFacebook::mut_machine_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountInfoFacebook>(
                    "AccountInfoFacebook",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountInfoFacebook {
    fn clear(&mut self) {
        self.clear_access_token();
        self.clear_machine_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountInfoFacebook {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountInfoFacebook {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AuthenticationType {
    AUTHENTICATION_USER_PASS = 0,
    AUTHENTICATION_STORED_SPOTIFY_CREDENTIALS = 1,
    AUTHENTICATION_STORED_FACEBOOK_CREDENTIALS = 2,
    AUTHENTICATION_SPOTIFY_TOKEN = 3,
    AUTHENTICATION_FACEBOOK_TOKEN = 4,
}

impl ::protobuf::ProtobufEnum for AuthenticationType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AuthenticationType> {
        match value {
            0 => ::std::option::Option::Some(AuthenticationType::AUTHENTICATION_USER_PASS),
            1 => ::std::option::Option::Some(AuthenticationType::AUTHENTICATION_STORED_SPOTIFY_CREDENTIALS),
            2 => ::std::option::Option::Some(AuthenticationType::AUTHENTICATION_STORED_FACEBOOK_CREDENTIALS),
            3 => ::std::option::Option::Some(AuthenticationType::AUTHENTICATION_SPOTIFY_TOKEN),
            4 => ::std::option::Option::Some(AuthenticationType::AUTHENTICATION_FACEBOOK_TOKEN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AuthenticationType] = &[
            AuthenticationType::AUTHENTICATION_USER_PASS,
            AuthenticationType::AUTHENTICATION_STORED_SPOTIFY_CREDENTIALS,
            AuthenticationType::AUTHENTICATION_STORED_FACEBOOK_CREDENTIALS,
            AuthenticationType::AUTHENTICATION_SPOTIFY_TOKEN,
            AuthenticationType::AUTHENTICATION_FACEBOOK_TOKEN,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AuthenticationType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AuthenticationType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AuthenticationType {
}

impl ::protobuf::reflect::ProtobufValue for AuthenticationType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AccountCreation {
    ACCOUNT_CREATION_ALWAYS_PROMPT = 1,
    ACCOUNT_CREATION_ALWAYS_CREATE = 3,
}

impl ::protobuf::ProtobufEnum for AccountCreation {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AccountCreation> {
        match value {
            1 => ::std::option::Option::Some(AccountCreation::ACCOUNT_CREATION_ALWAYS_PROMPT),
            3 => ::std::option::Option::Some(AccountCreation::ACCOUNT_CREATION_ALWAYS_CREATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AccountCreation] = &[
            AccountCreation::ACCOUNT_CREATION_ALWAYS_PROMPT,
            AccountCreation::ACCOUNT_CREATION_ALWAYS_CREATE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AccountCreation>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AccountCreation", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AccountCreation {
}

impl ::protobuf::reflect::ProtobufValue for AccountCreation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CpuFamily {
    CPU_UNKNOWN = 0,
    CPU_X86 = 1,
    CPU_X86_64 = 2,
    CPU_PPC = 3,
    CPU_PPC_64 = 4,
    CPU_ARM = 5,
    CPU_IA64 = 6,
    CPU_SH = 7,
    CPU_MIPS = 8,
    CPU_BLACKFIN = 9,
}

impl ::protobuf::ProtobufEnum for CpuFamily {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CpuFamily> {
        match value {
            0 => ::std::option::Option::Some(CpuFamily::CPU_UNKNOWN),
            1 => ::std::option::Option::Some(CpuFamily::CPU_X86),
            2 => ::std::option::Option::Some(CpuFamily::CPU_X86_64),
            3 => ::std::option::Option::Some(CpuFamily::CPU_PPC),
            4 => ::std::option::Option::Some(CpuFamily::CPU_PPC_64),
            5 => ::std::option::Option::Some(CpuFamily::CPU_ARM),
            6 => ::std::option::Option::Some(CpuFamily::CPU_IA64),
            7 => ::std::option::Option::Some(CpuFamily::CPU_SH),
            8 => ::std::option::Option::Some(CpuFamily::CPU_MIPS),
            9 => ::std::option::Option::Some(CpuFamily::CPU_BLACKFIN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CpuFamily] = &[
            CpuFamily::CPU_UNKNOWN,
            CpuFamily::CPU_X86,
            CpuFamily::CPU_X86_64,
            CpuFamily::CPU_PPC,
            CpuFamily::CPU_PPC_64,
            CpuFamily::CPU_ARM,
            CpuFamily::CPU_IA64,
            CpuFamily::CPU_SH,
            CpuFamily::CPU_MIPS,
            CpuFamily::CPU_BLACKFIN,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CpuFamily>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CpuFamily", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CpuFamily {
}

impl ::protobuf::reflect::ProtobufValue for CpuFamily {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Brand {
    BRAND_UNBRANDED = 0,
    BRAND_INQ = 1,
    BRAND_HTC = 2,
    BRAND_NOKIA = 3,
}

impl ::protobuf::ProtobufEnum for Brand {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Brand> {
        match value {
            0 => ::std::option::Option::Some(Brand::BRAND_UNBRANDED),
            1 => ::std::option::Option::Some(Brand::BRAND_INQ),
            2 => ::std::option::Option::Some(Brand::BRAND_HTC),
            3 => ::std::option::Option::Some(Brand::BRAND_NOKIA),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Brand] = &[
            Brand::BRAND_UNBRANDED,
            Brand::BRAND_INQ,
            Brand::BRAND_HTC,
            Brand::BRAND_NOKIA,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Brand>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Brand", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Brand {
}

impl ::protobuf::reflect::ProtobufValue for Brand {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Os {
    OS_UNKNOWN = 0,
    OS_WINDOWS = 1,
    OS_OSX = 2,
    OS_IPHONE = 3,
    OS_S60 = 4,
    OS_LINUX = 5,
    OS_WINDOWS_CE = 6,
    OS_ANDROID = 7,
    OS_PALM = 8,
    OS_FREEBSD = 9,
    OS_BLACKBERRY = 10,
    OS_SONOS = 11,
    OS_LOGITECH = 12,
    OS_WP7 = 13,
    OS_ONKYO = 14,
    OS_PHILIPS = 15,
    OS_WD = 16,
    OS_VOLVO = 17,
    OS_TIVO = 18,
    OS_AWOX = 19,
    OS_MEEGO = 20,
    OS_QNXNTO = 21,
    OS_BCO = 22,
}

impl ::protobuf::ProtobufEnum for Os {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Os> {
        match value {
            0 => ::std::option::Option::Some(Os::OS_UNKNOWN),
            1 => ::std::option::Option::Some(Os::OS_WINDOWS),
            2 => ::std::option::Option::Some(Os::OS_OSX),
            3 => ::std::option::Option::Some(Os::OS_IPHONE),
            4 => ::std::option::Option::Some(Os::OS_S60),
            5 => ::std::option::Option::Some(Os::OS_LINUX),
            6 => ::std::option::Option::Some(Os::OS_WINDOWS_CE),
            7 => ::std::option::Option::Some(Os::OS_ANDROID),
            8 => ::std::option::Option::Some(Os::OS_PALM),
            9 => ::std::option::Option::Some(Os::OS_FREEBSD),
            10 => ::std::option::Option::Some(Os::OS_BLACKBERRY),
            11 => ::std::option::Option::Some(Os::OS_SONOS),
            12 => ::std::option::Option::Some(Os::OS_LOGITECH),
            13 => ::std::option::Option::Some(Os::OS_WP7),
            14 => ::std::option::Option::Some(Os::OS_ONKYO),
            15 => ::std::option::Option::Some(Os::OS_PHILIPS),
            16 => ::std::option::Option::Some(Os::OS_WD),
            17 => ::std::option::Option::Some(Os::OS_VOLVO),
            18 => ::std::option::Option::Some(Os::OS_TIVO),
            19 => ::std::option::Option::Some(Os::OS_AWOX),
            20 => ::std::option::Option::Some(Os::OS_MEEGO),
            21 => ::std::option::Option::Some(Os::OS_QNXNTO),
            22 => ::std::option::Option::Some(Os::OS_BCO),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Os] = &[
            Os::OS_UNKNOWN,
            Os::OS_WINDOWS,
            Os::OS_OSX,
            Os::OS_IPHONE,
            Os::OS_S60,
            Os::OS_LINUX,
            Os::OS_WINDOWS_CE,
            Os::OS_ANDROID,
            Os::OS_PALM,
            Os::OS_FREEBSD,
            Os::OS_BLACKBERRY,
            Os::OS_SONOS,
            Os::OS_LOGITECH,
            Os::OS_WP7,
            Os::OS_ONKYO,
            Os::OS_PHILIPS,
            Os::OS_WD,
            Os::OS_VOLVO,
            Os::OS_TIVO,
            Os::OS_AWOX,
            Os::OS_MEEGO,
            Os::OS_QNXNTO,
            Os::OS_BCO,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Os>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Os", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Os {
}

impl ::protobuf::reflect::ProtobufValue for Os {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AccountType {
    Spotify = 0,
    Facebook = 1,
}

impl ::protobuf::ProtobufEnum for AccountType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AccountType> {
        match value {
            0 => ::std::option::Option::Some(AccountType::Spotify),
            1 => ::std::option::Option::Some(AccountType::Facebook),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AccountType] = &[
            AccountType::Spotify,
            AccountType::Facebook,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AccountType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AccountType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AccountType {
}

impl ::protobuf::reflect::ProtobufValue for AccountType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14authentication.proto\"\xec\x03\n\x17ClientResponseEncrypted\x12>\n\
    \x11login_credentials\x18\n\x20\x02(\x0b2\x11.LoginCredentialsR\x10login\
    Credentials\x12;\n\x10account_creation\x18\x14\x20\x01(\x0e2\x10.Account\
    CreationR\x0faccountCreation\x12L\n\x14fingerprint_response\x18\x1e\x20\
    \x01(\x0b2\x19.FingerprintResponseUnionR\x13fingerprintResponse\x121\n\
    \x0bpeer_ticket\x18(\x20\x01(\x0b2\x10.PeerTicketUnionR\npeerTicket\x12,\
    \n\x0bsystem_info\x182\x20\x02(\x0b2\x0b.SystemInfoR\nsystemInfo\x12%\n\
    \x0eplatform_model\x18<\x20\x01(\tR\rplatformModel\x12%\n\x0eversion_str\
    ing\x18F\x20\x01(\tR\rversionString\x12)\n\x06appkey\x18P\x20\x01(\x0b2\
    \x11.LibspotifyAppKeyR\x06appkey\x12,\n\x0bclient_info\x18Z\x20\x01(\x0b\
    2\x0b.ClientInfoR\nclientInfo\"r\n\x10LoginCredentials\x12\x1a\n\x08user\
    name\x18\n\x20\x01(\tR\x08username\x12%\n\x03typ\x18\x14\x20\x02(\x0e2\
    \x13.AuthenticationTypeR\x03typ\x12\x1b\n\tauth_data\x18\x1e\x20\x01(\
    \x0cR\x08authData\"\x8c\x01\n\x18FingerprintResponseUnion\x12/\n\x05grai\
    n\x18\n\x20\x01(\x0b2\x19.FingerprintGrainResponseR\x05grain\x12?\n\x0bh\
    mac_ripemd\x18\x14\x20\x01(\x0b2\x1e.FingerprintHmacRipemdResponseR\nhma\
    cRipemd\"?\n\x18FingerprintGrainResponse\x12#\n\rencrypted_key\x18\n\x20\
    \x02(\x0cR\x0cencryptedKey\"3\n\x1dFingerprintHmacRipemdResponse\x12\x12\
    \n\x04hmac\x18\n\x20\x02(\x0cR\x04hmac\"u\n\x0fPeerTicketUnion\x123\n\np\
    ublic_key\x18\n\x20\x01(\x0b2\x14.PeerTicketPublicKeyR\tpublicKey\x12-\n\
    \nold_ticket\x18\x14\x20\x01(\x0b2\x0e.PeerTicketOldR\toldTicket\"4\n\
    \x13PeerTicketPublicKey\x12\x1d\n\npublic_key\x18\n\x20\x02(\x0cR\tpubli\
    cKey\"d\n\rPeerTicketOld\x12\x1f\n\x0bpeer_ticket\x18\n\x20\x02(\x0cR\np\
    eerTicket\x122\n\x15peer_ticket_signature\x18\x14\x20\x02(\x0cR\x13peerT\
    icketSignature\"\xd4\x02\n\nSystemInfo\x12)\n\ncpu_family\x18\n\x20\x02(\
    \x0e2\n.CpuFamilyR\tcpuFamily\x12\x1f\n\x0bcpu_subtype\x18\x14\x20\x01(\
    \rR\ncpuSubtype\x12\x17\n\x07cpu_ext\x18\x1e\x20\x01(\rR\x06cpuExt\x12\
    \x1c\n\x05brand\x18(\x20\x01(\x0e2\x06.BrandR\x05brand\x12\x1f\n\x0bbran\
    d_flags\x182\x20\x01(\rR\nbrandFlags\x12\x13\n\x02os\x18<\x20\x02(\x0e2\
    \x03.OsR\x02os\x12\x1d\n\nos_version\x18F\x20\x01(\rR\tosVersion\x12\x15\
    \n\x06os_ext\x18P\x20\x01(\rR\x05osExt\x12:\n\x19system_information_stri\
    ng\x18Z\x20\x01(\tR\x17systemInformationString\x12\x1b\n\tdevice_id\x18d\
    \x20\x01(\tR\x08deviceId\"\xa5\x01\n\x10LibspotifyAppKey\x12\x18\n\x07ve\
    rsion\x18\x01\x20\x02(\rR\x07version\x12\x16\n\x06devkey\x18\x02\x20\x02\
    (\x0cR\x06devkey\x12\x1c\n\tsignature\x18\x03\x20\x02(\x0cR\tsignature\
    \x12\x1c\n\tuseragent\x18\x04\x20\x02(\tR\tuseragent\x12#\n\rcallback_ha\
    sh\x18\x05\x20\x02(\x0cR\x0ccallbackHash\"g\n\nClientInfo\x12\x18\n\x07l\
    imited\x18\x01\x20\x01(\x08R\x07limited\x12#\n\x02fb\x18\x02\x20\x01(\
    \x0b2\x13.ClientInfoFacebookR\x02fb\x12\x1a\n\x08language\x18\x03\x20\
    \x01(\tR\x08language\"3\n\x12ClientInfoFacebook\x12\x1d\n\nmachine_id\
    \x18\x01\x20\x01(\tR\tmachineId\"\xd4\x03\n\tAPWelcome\x12-\n\x12canonic\
    al_username\x18\n\x20\x02(\tR\x11canonicalUsername\x12A\n\x16account_typ\
    e_logged_in\x18\x14\x20\x02(\x0e2\x0c.AccountTypeR\x13accountTypeLoggedI\
    n\x12I\n\x1acredentials_type_logged_in\x18\x19\x20\x02(\x0e2\x0c.Account\
    TypeR\x17credentialsTypeLoggedIn\x12X\n\x1ereusable_auth_credentials_typ\
    e\x18\x1e\x20\x02(\x0e2\x13.AuthenticationTypeR\x1breusableAuthCredentia\
    lsType\x12:\n\x19reusable_auth_credentials\x18(\x20\x02(\x0cR\x17reusabl\
    eAuthCredentials\x12\x1d\n\nlfs_secret\x182\x20\x01(\x0cR\tlfsSecret\x12\
    /\n\x0caccount_info\x18<\x20\x01(\x0b2\x0c.AccountInfoR\x0baccountInfo\
    \x12$\n\x02fb\x18F\x20\x01(\x0b2\x14.AccountInfoFacebookR\x02fb\"n\n\x0b\
    AccountInfo\x12-\n\x07spotify\x18\x01\x20\x01(\x0b2\x13.AccountInfoSpoti\
    fyR\x07spotify\x120\n\x08facebook\x18\x02\x20\x01(\x0b2\x14.AccountInfoF\
    acebookR\x08facebook\"\x14\n\x12AccountInfoSpotify\"W\n\x13AccountInfoFa\
    cebook\x12!\n\x0caccess_token\x18\x01\x20\x01(\tR\x0baccessToken\x12\x1d\
    \n\nmachine_id\x18\x02\x20\x01(\tR\tmachineId*\xd6\x01\n\x12Authenticati\
    onType\x12\x1c\n\x18AUTHENTICATION_USER_PASS\x10\0\x12-\n)AUTHENTICATION\
    _STORED_SPOTIFY_CREDENTIALS\x10\x01\x12.\n*AUTHENTICATION_STORED_FACEBOO\
    K_CREDENTIALS\x10\x02\x12\x20\n\x1cAUTHENTICATION_SPOTIFY_TOKEN\x10\x03\
    \x12!\n\x1dAUTHENTICATION_FACEBOOK_TOKEN\x10\x04*Y\n\x0fAccountCreation\
    \x12\"\n\x1eACCOUNT_CREATION_ALWAYS_PROMPT\x10\x01\x12\"\n\x1eACCOUNT_CR\
    EATION_ALWAYS_CREATE\x10\x03*\x9d\x01\n\tCpuFamily\x12\x0f\n\x0bCPU_UNKN\
    OWN\x10\0\x12\x0b\n\x07CPU_X86\x10\x01\x12\x0e\n\nCPU_X86_64\x10\x02\x12\
    \x0b\n\x07CPU_PPC\x10\x03\x12\x0e\n\nCPU_PPC_64\x10\x04\x12\x0b\n\x07CPU\
    _ARM\x10\x05\x12\x0c\n\x08CPU_IA64\x10\x06\x12\n\n\x06CPU_SH\x10\x07\x12\
    \x0c\n\x08CPU_MIPS\x10\x08\x12\x10\n\x0cCPU_BLACKFIN\x10\t*K\n\x05Brand\
    \x12\x13\n\x0fBRAND_UNBRANDED\x10\0\x12\r\n\tBRAND_INQ\x10\x01\x12\r\n\t\
    BRAND_HTC\x10\x02\x12\x0f\n\x0bBRAND_NOKIA\x10\x03*\xd1\x02\n\x02Os\x12\
    \x0e\n\nOS_UNKNOWN\x10\0\x12\x0e\n\nOS_WINDOWS\x10\x01\x12\n\n\x06OS_OSX\
    \x10\x02\x12\r\n\tOS_IPHONE\x10\x03\x12\n\n\x06OS_S60\x10\x04\x12\x0c\n\
    \x08OS_LINUX\x10\x05\x12\x11\n\rOS_WINDOWS_CE\x10\x06\x12\x0e\n\nOS_ANDR\
    OID\x10\x07\x12\x0b\n\x07OS_PALM\x10\x08\x12\x0e\n\nOS_FREEBSD\x10\t\x12\
    \x11\n\rOS_BLACKBERRY\x10\n\x12\x0c\n\x08OS_SONOS\x10\x0b\x12\x0f\n\x0bO\
    S_LOGITECH\x10\x0c\x12\n\n\x06OS_WP7\x10\r\x12\x0c\n\x08OS_ONKYO\x10\x0e\
    \x12\x0e\n\nOS_PHILIPS\x10\x0f\x12\t\n\x05OS_WD\x10\x10\x12\x0c\n\x08OS_\
    VOLVO\x10\x11\x12\x0b\n\x07OS_TIVO\x10\x12\x12\x0b\n\x07OS_AWOX\x10\x13\
    \x12\x0c\n\x08OS_MEEGO\x10\x14\x12\r\n\tOS_QNXNTO\x10\x15\x12\n\n\x06OS_\
    BCO\x10\x16*(\n\x0bAccountType\x12\x0b\n\x07Spotify\x10\0\x12\x0c\n\x08F\
    acebook\x10\x01J\xee/\n\x07\x12\x05\0\0\xa4\x01\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x0c\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\x02\x08\x1f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x046\n\x0c\n\
    \x05\x04\0\x02\0\x04\x12\x03\x03\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\x03\r\x1d\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x1e/\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x0325\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\
    5\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x04\x04\x0c\n\x0c\n\x05\x04\0\
    \x02\x01\x06\x12\x03\x04\r\x1c\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\
    \x1d-\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0404\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03\x05\x04B\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x05\x04\x0c\
    \n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x05\r%\n\x0c\n\x05\x04\0\x02\x02\
    \x01\x12\x03\x05&:\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05=A\n\x0b\n\
    \x04\x04\0\x02\x03\x12\x03\x06\x040\n\x0c\n\x05\x04\0\x02\x03\x04\x12\
    \x03\x06\x04\x0c\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x06\r\x1c\n\x0c\n\
    \x05\x04\0\x02\x03\x01\x12\x03\x06\x1d(\n\x0c\n\x05\x04\0\x02\x03\x03\
    \x12\x03\x06+/\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x07\x04+\n\x0c\n\x05\
    \x04\0\x02\x04\x04\x12\x03\x07\x04\x0c\n\x0c\n\x05\x04\0\x02\x04\x06\x12\
    \x03\x07\r\x17\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x07\x18#\n\x0c\n\
    \x05\x04\0\x02\x04\x03\x12\x03\x07&*\n\x0b\n\x04\x04\0\x02\x05\x12\x03\
    \x08\x04*\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x03\x08\x04\x0c\n\x0c\n\x05\
    \x04\0\x02\x05\x05\x12\x03\x08\r\x13\n\x0c\n\x05\x04\0\x02\x05\x01\x12\
    \x03\x08\x14\"\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x08%)\n\x0b\n\x04\
    \x04\0\x02\x06\x12\x03\t\x04*\n\x0c\n\x05\x04\0\x02\x06\x04\x12\x03\t\
    \x04\x0c\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\t\r\x13\n\x0c\n\x05\x04\0\
    \x02\x06\x01\x12\x03\t\x14\"\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\t%)\n\
    \x0b\n\x04\x04\0\x02\x07\x12\x03\n\x04,\n\x0c\n\x05\x04\0\x02\x07\x04\
    \x12\x03\n\x04\x0c\n\x0c\n\x05\x04\0\x02\x07\x06\x12\x03\n\r\x1d\n\x0c\n\
    \x05\x04\0\x02\x07\x01\x12\x03\n\x1e$\n\x0c\n\x05\x04\0\x02\x07\x03\x12\
    \x03\n'+\n\x0b\n\x04\x04\0\x02\x08\x12\x03\x0b\x04+\n\x0c\n\x05\x04\0\
    \x02\x08\x04\x12\x03\x0b\x04\x0c\n\x0c\n\x05\x04\0\x02\x08\x06\x12\x03\
    \x0b\r\x17\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x0b\x18#\n\x0c\n\x05\
    \x04\0\x02\x08\x03\x12\x03\x0b&*\n\n\n\x02\x04\x01\x12\x04\x0e\0\x12\x01\
    \n\n\n\x03\x04\x01\x01\x12\x03\x0e\x08\x18\n\x0b\n\x04\x04\x01\x02\0\x12\
    \x03\x0f\x04#\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x0f\x04\x0c\n\x0c\n\
    \x05\x04\x01\x02\0\x05\x12\x03\x0f\r\x13\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\x0f\x14\x1c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0f\x1f\"\n\
    \x0b\n\x04\x04\x01\x02\x01\x12\x03\x10\x04+\n\x0c\n\x05\x04\x01\x02\x01\
    \x04\x12\x03\x10\x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x10\r\
    \x1f\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x10\x20#\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03\x10&*\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x11\
    \x04$\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\x11\x04\x0c\n\x0c\n\x05\
    \x04\x01\x02\x02\x05\x12\x03\x11\r\x12\n\x0c\n\x05\x04\x01\x02\x02\x01\
    \x12\x03\x11\x13\x1c\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x11\x1f#\n\
    \n\n\x02\x05\0\x12\x04\x14\0\x1a\x01\n\n\n\x03\x05\0\x01\x12\x03\x14\x05\
    \x17\n\x0b\n\x04\x05\0\x02\0\x12\x03\x15\x04#\n\x0c\n\x05\x05\0\x02\0\
    \x01\x12\x03\x15\x04\x1c\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x15\x1f\"\n\
    \x0b\n\x04\x05\0\x02\x01\x12\x03\x16\x044\n\x0c\n\x05\x05\0\x02\x01\x01\
    \x12\x03\x16\x04-\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x1603\n\x0b\n\
    \x04\x05\0\x02\x02\x12\x03\x17\x045\n\x0c\n\x05\x05\0\x02\x02\x01\x12\
    \x03\x17\x04.\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x1714\n\x0b\n\x04\
    \x05\0\x02\x03\x12\x03\x18\x04'\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\
    \x18\x04\x20\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x18#&\n\x0b\n\x04\x05\
    \0\x02\x04\x12\x03\x19\x04(\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x19\
    \x04!\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x19$'\n\n\n\x02\x05\x01\x12\
    \x04\x1c\0\x1f\x01\n\n\n\x03\x05\x01\x01\x12\x03\x1c\x05\x14\n\x0b\n\x04\
    \x05\x01\x02\0\x12\x03\x1d\x04)\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03\
    \x1d\x04\"\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03\x1d%(\n\x0b\n\x04\x05\
    \x01\x02\x01\x12\x03\x1e\x04)\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03\
    \x1e\x04\"\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03\x1e%(\n\n\n\x02\x04\
    \x02\x12\x04!\0$\x01\n\n\n\x03\x04\x02\x01\x12\x03!\x08\x20\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03\"\x042\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\"\
    \x04\x0c\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\"\r%\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x03\"&+\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\".1\n\x0b\n\
    \x04\x04\x02\x02\x01\x12\x03#\x04>\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\
    \x03#\x04\x0c\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03#\r*\n\x0c\n\x05\
    \x04\x02\x02\x01\x01\x12\x03#+6\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03#\
    9=\n\n\n\x02\x04\x03\x12\x04&\0(\x01\n\n\n\x03\x04\x03\x01\x12\x03&\x08\
    \x20\n\x0b\n\x04\x04\x03\x02\0\x12\x03'\x04'\n\x0c\n\x05\x04\x03\x02\0\
    \x04\x12\x03'\x04\x0c\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03'\r\x12\n\x0c\
    \n\x05\x04\x03\x02\0\x01\x12\x03'\x13\x20\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x03'#&\n\n\n\x02\x04\x04\x12\x04*\0,\x01\n\n\n\x03\x04\x04\x01\x12\
    \x03*\x08%\n\x0b\n\x04\x04\x04\x02\0\x12\x03+\x04\x1e\n\x0c\n\x05\x04\
    \x04\x02\0\x04\x12\x03+\x04\x0c\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03+\r\
    \x12\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03+\x13\x17\n\x0c\n\x05\x04\x04\
    \x02\0\x03\x12\x03+\x1a\x1d\n\n\n\x02\x04\x05\x12\x04.\01\x01\n\n\n\x03\
    \x04\x05\x01\x12\x03.\x08\x17\n\x0b\n\x04\x04\x05\x02\0\x12\x03/\x042\n\
    \x0c\n\x05\x04\x05\x02\0\x04\x12\x03/\x04\x0c\n\x0c\n\x05\x04\x05\x02\0\
    \x06\x12\x03/\r\x20\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03/!+\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x03/.1\n\x0b\n\x04\x04\x05\x02\x01\x12\x030\x04-\
    \n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x030\x04\x0c\n\x0c\n\x05\x04\x05\
    \x02\x01\x06\x12\x030\r\x1a\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x030\x1b\
    %\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x030(,\n\n\n\x02\x04\x06\x12\x043\
    \05\x01\n\n\n\x03\x04\x06\x01\x12\x033\x08\x1b\n\x0b\n\x04\x04\x06\x02\0\
    \x12\x034\x04$\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x034\x04\x0c\n\x0c\n\
    \x05\x04\x06\x02\0\x05\x12\x034\r\x12\n\x0c\n\x05\x04\x06\x02\0\x01\x12\
    \x034\x13\x1d\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x034\x20#\n\n\n\x02\x04\
    \x07\x12\x047\0:\x01\n\n\n\x03\x04\x07\x01\x12\x037\x08\x15\n\x0b\n\x04\
    \x04\x07\x02\0\x12\x038\x04%\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x038\x04\
    \x0c\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x038\r\x12\n\x0c\n\x05\x04\x07\
    \x02\0\x01\x12\x038\x13\x1e\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x038!$\n\
    \x0b\n\x04\x04\x07\x02\x01\x12\x039\x040\n\x0c\n\x05\x04\x07\x02\x01\x04\
    \x12\x039\x04\x0c\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x039\r\x12\n\x0c\n\
    \x05\x04\x07\x02\x01\x01\x12\x039\x13(\n\x0c\n\x05\x04\x07\x02\x01\x03\
    \x12\x039+/\n\n\n\x02\x04\x08\x12\x04<\0G\x01\n\n\n\x03\x04\x08\x01\x12\
    \x03<\x08\x12\n\x0b\n\x04\x04\x08\x02\0\x12\x03=\x04(\n\x0c\n\x05\x04\
    \x08\x02\0\x04\x12\x03=\x04\x0c\n\x0c\n\x05\x04\x08\x02\0\x06\x12\x03=\r\
    \x16\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03=\x17!\n\x0c\n\x05\x04\x08\x02\
    \0\x03\x12\x03=$'\n\x0b\n\x04\x04\x08\x02\x01\x12\x03>\x04'\n\x0c\n\x05\
    \x04\x08\x02\x01\x04\x12\x03>\x04\x0c\n\x0c\n\x05\x04\x08\x02\x01\x05\
    \x12\x03>\r\x13\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03>\x14\x1f\n\x0c\n\
    \x05\x04\x08\x02\x01\x03\x12\x03>\"&\n\x0b\n\x04\x04\x08\x02\x02\x12\x03\
    ?\x04#\n\x0c\n\x05\x04\x08\x02\x02\x04\x12\x03?\x04\x0c\n\x0c\n\x05\x04\
    \x08\x02\x02\x05\x12\x03?\r\x13\n\x0c\n\x05\x04\x08\x02\x02\x01\x12\x03?\
    \x14\x1b\n\x0c\n\x05\x04\x08\x02\x02\x03\x12\x03?\x1e\"\n\x0b\n\x04\x04\
    \x08\x02\x03\x12\x03@\x04\x20\n\x0c\n\x05\x04\x08\x02\x03\x04\x12\x03@\
    \x04\x0c\n\x0c\n\x05\x04\x08\x02\x03\x06\x12\x03@\r\x12\n\x0c\n\x05\x04\
    \x08\x02\x03\x01\x12\x03@\x13\x18\n\x0c\n\x05\x04\x08\x02\x03\x03\x12\
    \x03@\x1b\x1f\n\x0b\n\x04\x04\x08\x02\x04\x12\x03A\x04'\n\x0c\n\x05\x04\
    \x08\x02\x04\x04\x12\x03A\x04\x0c\n\x0c\n\x05\x04\x08\x02\x04\x05\x12\
    \x03A\r\x13\n\x0c\n\x05\x04\x08\x02\x04\x01\x12\x03A\x14\x1f\n\x0c\n\x05\
    \x04\x08\x02\x04\x03\x12\x03A\"&\n\x0b\n\x04\x04\x08\x02\x05\x12\x03B\
    \x04\x1a\n\x0c\n\x05\x04\x08\x02\x05\x04\x12\x03B\x04\x0c\n\x0c\n\x05\
    \x04\x08\x02\x05\x06\x12\x03B\r\x0f\n\x0c\n\x05\x04\x08\x02\x05\x01\x12\
    \x03B\x10\x12\n\x0c\n\x05\x04\x08\x02\x05\x03\x12\x03B\x15\x19\n\x0b\n\
    \x04\x04\x08\x02\x06\x12\x03C\x04&\n\x0c\n\x05\x04\x08\x02\x06\x04\x12\
    \x03C\x04\x0c\n\x0c\n\x05\x04\x08\x02\x06\x05\x12\x03C\r\x13\n\x0c\n\x05\
    \x04\x08\x02\x06\x01\x12\x03C\x14\x1e\n\x0c\n\x05\x04\x08\x02\x06\x03\
    \x12\x03C!%\n\x0b\n\x04\x04\x08\x02\x07\x12\x03D\x04\"\n\x0c\n\x05\x04\
    \x08\x02\x07\x04\x12\x03D\x04\x0c\n\x0c\n\x05\x04\x08\x02\x07\x05\x12\
    \x03D\r\x13\n\x0c\n\x05\x04\x08\x02\x07\x01\x12\x03D\x14\x1a\n\x0c\n\x05\
    \x04\x08\x02\x07\x03\x12\x03D\x1d!\n\x0b\n\x04\x04\x08\x02\x08\x12\x03E\
    \x045\n\x0c\n\x05\x04\x08\x02\x08\x04\x12\x03E\x04\x0c\n\x0c\n\x05\x04\
    \x08\x02\x08\x05\x12\x03E\r\x13\n\x0c\n\x05\x04\x08\x02\x08\x01\x12\x03E\
    \x14-\n\x0c\n\x05\x04\x08\x02\x08\x03\x12\x03E04\n\x0b\n\x04\x04\x08\x02\
    \t\x12\x03F\x04%\n\x0c\n\x05\x04\x08\x02\t\x04\x12\x03F\x04\x0c\n\x0c\n\
    \x05\x04\x08\x02\t\x05\x12\x03F\r\x13\n\x0c\n\x05\x04\x08\x02\t\x01\x12\
    \x03F\x14\x1d\n\x0c\n\x05\x04\x08\x02\t\x03\x12\x03F\x20$\n\n\n\x02\x05\
    \x02\x12\x04I\0T\x01\n\n\n\x03\x05\x02\x01\x12\x03I\x05\x0e\n\x0b\n\x04\
    \x05\x02\x02\0\x12\x03J\x04\x16\n\x0c\n\x05\x05\x02\x02\0\x01\x12\x03J\
    \x04\x0f\n\x0c\n\x05\x05\x02\x02\0\x02\x12\x03J\x12\x15\n\x0b\n\x04\x05\
    \x02\x02\x01\x12\x03K\x04\x12\n\x0c\n\x05\x05\x02\x02\x01\x01\x12\x03K\
    \x04\x0b\n\x0c\n\x05\x05\x02\x02\x01\x02\x12\x03K\x0e\x11\n\x0b\n\x04\
    \x05\x02\x02\x02\x12\x03L\x04\x15\n\x0c\n\x05\x05\x02\x02\x02\x01\x12\
    \x03L\x04\x0e\n\x0c\n\x05\x05\x02\x02\x02\x02\x12\x03L\x11\x14\n\x0b\n\
    \x04\x05\x02\x02\x03\x12\x03M\x04\x12\n\x0c\n\x05\x05\x02\x02\x03\x01\
    \x12\x03M\x04\x0b\n\x0c\n\x05\x05\x02\x02\x03\x02\x12\x03M\x0e\x11\n\x0b\
    \n\x04\x05\x02\x02\x04\x12\x03N\x04\x15\n\x0c\n\x05\x05\x02\x02\x04\x01\
    \x12\x03N\x04\x0e\n\x0c\n\x05\x05\x02\x02\x04\x02\x12\x03N\x11\x14\n\x0b\
    \n\x04\x05\x02\x02\x05\x12\x03O\x04\x12\n\x0c\n\x05\x05\x02\x02\x05\x01\
    \x12\x03O\x04\x0b\n\x0c\n\x05\x05\x02\x02\x05\x02\x12\x03O\x0e\x11\n\x0b\
    \n\x04\x05\x02\x02\x06\x12\x03P\x04\x13\n\x0c\n\x05\x05\x02\x02\x06\x01\
    \x12\x03P\x04\x0c\n\x0c\n\x05\x05\x02\x02\x06\x02\x12\x03P\x0f\x12\n\x0b\
    \n\x04\x05\x02\x02\x07\x12\x03Q\x04\x11\n\x0c\n\x05\x05\x02\x02\x07\x01\
    \x12\x03Q\x04\n\n\x0c\n\x05\x05\x02\x02\x07\x02\x12\x03Q\r\x10\n\x0b\n\
    \x04\x05\x02\x02\x08\x12\x03R\x04\x13\n\x0c\n\x05\x05\x02\x02\x08\x01\
    \x12\x03R\x04\x0c\n\x0c\n\x05\x05\x02\x02\x08\x02\x12\x03R\x0f\x12\n\x0b\
    \n\x04\x05\x02\x02\t\x12\x03S\x04\x17\n\x0c\n\x05\x05\x02\x02\t\x01\x12\
    \x03S\x04\x10\n\x0c\n\x05\x05\x02\x02\t\x02\x12\x03S\x13\x16\n\n\n\x02\
    \x05\x03\x12\x04V\0[\x01\n\n\n\x03\x05\x03\x01\x12\x03V\x05\n\n\x0b\n\
    \x04\x05\x03\x02\0\x12\x03W\x04\x1a\n\x0c\n\x05\x05\x03\x02\0\x01\x12\
    \x03W\x04\x13\n\x0c\n\x05\x05\x03\x02\0\x02\x12\x03W\x16\x19\n\x0b\n\x04\
    \x05\x03\x02\x01\x12\x03X\x04\x14\n\x0c\n\x05\x05\x03\x02\x01\x01\x12\
    \x03X\x04\r\n\x0c\n\x05\x05\x03\x02\x01\x02\x12\x03X\x10\x13\n\x0b\n\x04\
    \x05\x03\x02\x02\x12\x03Y\x04\x14\n\x0c\n\x05\x05\x03\x02\x02\x01\x12\
    \x03Y\x04\r\n\x0c\n\x05\x05\x03\x02\x02\x02\x12\x03Y\x10\x13\n\x0b\n\x04\
    \x05\x03\x02\x03\x12\x03Z\x04\x16\n\x0c\n\x05\x05\x03\x02\x03\x01\x12\
    \x03Z\x04\x0f\n\x0c\n\x05\x05\x03\x02\x03\x02\x12\x03Z\x12\x15\n\n\n\x02\
    \x05\x04\x12\x04]\0u\x01\n\n\n\x03\x05\x04\x01\x12\x03]\x05\x07\n\x0b\n\
    \x04\x05\x04\x02\0\x12\x03^\x04\x15\n\x0c\n\x05\x05\x04\x02\0\x01\x12\
    \x03^\x04\x0e\n\x0c\n\x05\x05\x04\x02\0\x02\x12\x03^\x11\x14\n\x0b\n\x04\
    \x05\x04\x02\x01\x12\x03_\x04\x15\n\x0c\n\x05\x05\x04\x02\x01\x01\x12\
    \x03_\x04\x0e\n\x0c\n\x05\x05\x04\x02\x01\x02\x12\x03_\x11\x14\n\x0b\n\
    \x04\x05\x04\x02\x02\x12\x03`\x04\x11\n\x0c\n\x05\x05\x04\x02\x02\x01\
    \x12\x03`\x04\n\n\x0c\n\x05\x05\x04\x02\x02\x02\x12\x03`\r\x10\n\x0b\n\
    \x04\x05\x04\x02\x03\x12\x03a\x04\x14\n\x0c\n\x05\x05\x04\x02\x03\x01\
    \x12\x03a\x04\r\n\x0c\n\x05\x05\x04\x02\x03\x02\x12\x03a\x10\x13\n\x0b\n\
    \x04\x05\x04\x02\x04\x12\x03b\x04\x11\n\x0c\n\x05\x05\x04\x02\x04\x01\
    \x12\x03b\x04\n\n\x0c\n\x05\x05\x04\x02\x04\x02\x12\x03b\r\x10\n\x0b\n\
    \x04\x05\x04\x02\x05\x12\x03c\x04\x13\n\x0c\n\x05\x05\x04\x02\x05\x01\
    \x12\x03c\x04\x0c\n\x0c\n\x05\x05\x04\x02\x05\x02\x12\x03c\x0f\x12\n\x0b\
    \n\x04\x05\x04\x02\x06\x12\x03d\x04\x18\n\x0c\n\x05\x05\x04\x02\x06\x01\
    \x12\x03d\x04\x11\n\x0c\n\x05\x05\x04\x02\x06\x02\x12\x03d\x14\x17\n\x0b\
    \n\x04\x05\x04\x02\x07\x12\x03e\x04\x15\n\x0c\n\x05\x05\x04\x02\x07\x01\
    \x12\x03e\x04\x0e\n\x0c\n\x05\x05\x04\x02\x07\x02\x12\x03e\x11\x14\n\x0b\
    \n\x04\x05\x04\x02\x08\x12\x03f\x04\x12\n\x0c\n\x05\x05\x04\x02\x08\x01\
    \x12\x03f\x04\x0b\n\x0c\n\x05\x05\x04\x02\x08\x02\x12\x03f\x0e\x11\n\x0b\
    \n\x04\x05\x04\x02\t\x12\x03g\x04\x15\n\x0c\n\x05\x05\x04\x02\t\x01\x12\
    \x03g\x04\x0e\n\x0c\n\x05\x05\x04\x02\t\x02\x12\x03g\x11\x14\n\x0b\n\x04\
    \x05\x04\x02\n\x12\x03h\x04\x18\n\x0c\n\x05\x05\x04\x02\n\x01\x12\x03h\
    \x04\x11\n\x0c\n\x05\x05\x04\x02\n\x02\x12\x03h\x14\x17\n\x0b\n\x04\x05\
    \x04\x02\x0b\x12\x03i\x04\x13\n\x0c\n\x05\x05\x04\x02\x0b\x01\x12\x03i\
    \x04\x0c\n\x0c\n\x05\x05\x04\x02\x0b\x02\x12\x03i\x0f\x12\n\x0b\n\x04\
    \x05\x04\x02\x0c\x12\x03j\x04\x16\n\x0c\n\x05\x05\x04\x02\x0c\x01\x12\
    \x03j\x04\x0f\n\x0c\n\x05\x05\x04\x02\x0c\x02\x12\x03j\x12\x15\n\x0b\n\
    \x04\x05\x04\x02\r\x12\x03k\x04\x11\n\x0c\n\x05\x05\x04\x02\r\x01\x12\
    \x03k\x04\n\n\x0c\n\x05\x05\x04\x02\r\x02\x12\x03k\r\x10\n\x0b\n\x04\x05\
    \x04\x02\x0e\x12\x03l\x04\x13\n\x0c\n\x05\x05\x04\x02\x0e\x01\x12\x03l\
    \x04\x0c\n\x0c\n\x05\x05\x04\x02\x0e\x02\x12\x03l\x0f\x12\n\x0b\n\x04\
    \x05\x04\x02\x0f\x12\x03m\x04\x15\n\x0c\n\x05\x05\x04\x02\x0f\x01\x12\
    \x03m\x04\x0e\n\x0c\n\x05\x05\x04\x02\x0f\x02\x12\x03m\x11\x14\n\x0b\n\
    \x04\x05\x04\x02\x10\x12\x03n\x04\x11\n\x0c\n\x05\x05\x04\x02\x10\x01\
    \x12\x03n\x04\t\n\x0c\n\x05\x05\x04\x02\x10\x02\x12\x03n\x0c\x10\n\x0b\n\
    \x04\x05\x04\x02\x11\x12\x03o\x04\x14\n\x0c\n\x05\x05\x04\x02\x11\x01\
    \x12\x03o\x04\x0c\n\x0c\n\x05\x05\x04\x02\x11\x02\x12\x03o\x0f\x13\n\x0b\
    \n\x04\x05\x04\x02\x12\x12\x03p\x04\x13\n\x0c\n\x05\x05\x04\x02\x12\x01\
    \x12\x03p\x04\x0b\n\x0c\n\x05\x05\x04\x02\x12\x02\x12\x03p\x0e\x12\n\x0b\
    \n\x04\x05\x04\x02\x13\x12\x03q\x04\x13\n\x0c\n\x05\x05\x04\x02\x13\x01\
    \x12\x03q\x04\x0b\n\x0c\n\x05\x05\x04\x02\x13\x02\x12\x03q\x0e\x12\n\x0b\
    \n\x04\x05\x04\x02\x14\x12\x03r\x04\x14\n\x0c\n\x05\x05\x04\x02\x14\x01\
    \x12\x03r\x04\x0c\n\x0c\n\x05\x05\x04\x02\x14\x02\x12\x03r\x0f\x13\n\x0b\
    \n\x04\x05\x04\x02\x15\x12\x03s\x04\x15\n\x0c\n\x05\x05\x04\x02\x15\x01\
    \x12\x03s\x04\r\n\x0c\n\x05\x05\x04\x02\x15\x02\x12\x03s\x10\x14\n\x0b\n\
    \x04\x05\x04\x02\x16\x12\x03t\x04\x12\n\x0c\n\x05\x05\x04\x02\x16\x01\
    \x12\x03t\x04\n\n\x0c\n\x05\x05\x04\x02\x16\x02\x12\x03t\r\x11\n\n\n\x02\
    \x04\t\x12\x04w\0}\x01\n\n\n\x03\x04\t\x01\x12\x03w\x08\x18\n\x0b\n\x04\
    \x04\t\x02\0\x12\x03x\x04\"\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03x\x04\x0c\
    \n\x0c\n\x05\x04\t\x02\0\x05\x12\x03x\r\x13\n\x0c\n\x05\x04\t\x02\0\x01\
    \x12\x03x\x14\x1b\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03x\x1e!\n\x0b\n\x04\
    \x04\t\x02\x01\x12\x03y\x04\x20\n\x0c\n\x05\x04\t\x02\x01\x04\x12\x03y\
    \x04\x0c\n\x0c\n\x05\x04\t\x02\x01\x05\x12\x03y\r\x12\n\x0c\n\x05\x04\t\
    \x02\x01\x01\x12\x03y\x13\x19\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03y\x1c\
    \x1f\n\x0b\n\x04\x04\t\x02\x02\x12\x03z\x04#\n\x0c\n\x05\x04\t\x02\x02\
    \x04\x12\x03z\x04\x0c\n\x0c\n\x05\x04\t\x02\x02\x05\x12\x03z\r\x12\n\x0c\
    \n\x05\x04\t\x02\x02\x01\x12\x03z\x13\x1c\n\x0c\n\x05\x04\t\x02\x02\x03\
    \x12\x03z\x1f\"\n\x0b\n\x04\x04\t\x02\x03\x12\x03{\x04$\n\x0c\n\x05\x04\
    \t\x02\x03\x04\x12\x03{\x04\x0c\n\x0c\n\x05\x04\t\x02\x03\x05\x12\x03{\r\
    \x13\n\x0c\n\x05\x04\t\x02\x03\x01\x12\x03{\x14\x1d\n\x0c\n\x05\x04\t\
    \x02\x03\x03\x12\x03{\x20#\n\x0b\n\x04\x04\t\x02\x04\x12\x03|\x04'\n\x0c\
    \n\x05\x04\t\x02\x04\x04\x12\x03|\x04\x0c\n\x0c\n\x05\x04\t\x02\x04\x05\
    \x12\x03|\r\x12\n\x0c\n\x05\x04\t\x02\x04\x01\x12\x03|\x13\x20\n\x0c\n\
    \x05\x04\t\x02\x04\x03\x12\x03|#&\n\x0b\n\x02\x04\n\x12\x05\x7f\0\x83\
    \x01\x01\n\n\n\x03\x04\n\x01\x12\x03\x7f\x08\x12\n\x0c\n\x04\x04\n\x02\0\
    \x12\x04\x80\x01\x04\x20\n\r\n\x05\x04\n\x02\0\x04\x12\x04\x80\x01\x04\
    \x0c\n\r\n\x05\x04\n\x02\0\x05\x12\x04\x80\x01\r\x11\n\r\n\x05\x04\n\x02\
    \0\x01\x12\x04\x80\x01\x12\x19\n\r\n\x05\x04\n\x02\0\x03\x12\x04\x80\x01\
    \x1c\x1f\n\x0c\n\x04\x04\n\x02\x01\x12\x04\x81\x01\x04)\n\r\n\x05\x04\n\
    \x02\x01\x04\x12\x04\x81\x01\x04\x0c\n\r\n\x05\x04\n\x02\x01\x06\x12\x04\
    \x81\x01\r\x1f\n\r\n\x05\x04\n\x02\x01\x01\x12\x04\x81\x01\x20\"\n\r\n\
    \x05\x04\n\x02\x01\x03\x12\x04\x81\x01%(\n\x0c\n\x04\x04\n\x02\x02\x12\
    \x04\x82\x01\x04#\n\r\n\x05\x04\n\x02\x02\x04\x12\x04\x82\x01\x04\x0c\n\
    \r\n\x05\x04\n\x02\x02\x05\x12\x04\x82\x01\r\x13\n\r\n\x05\x04\n\x02\x02\
    \x01\x12\x04\x82\x01\x14\x1c\n\r\n\x05\x04\n\x02\x02\x03\x12\x04\x82\x01\
    \x1f\"\n\x0c\n\x02\x04\x0b\x12\x06\x85\x01\0\x87\x01\x01\n\x0b\n\x03\x04\
    \x0b\x01\x12\x04\x85\x01\x08\x1a\n\x0c\n\x04\x04\x0b\x02\0\x12\x04\x86\
    \x01\x04%\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04\x86\x01\x04\x0c\n\r\n\x05\
    \x04\x0b\x02\0\x05\x12\x04\x86\x01\r\x13\n\r\n\x05\x04\x0b\x02\0\x01\x12\
    \x04\x86\x01\x14\x1e\n\r\n\x05\x04\x0b\x02\0\x03\x12\x04\x86\x01!$\n\x0c\
    \n\x02\x04\x0c\x12\x06\x89\x01\0\x92\x01\x01\n\x0b\n\x03\x04\x0c\x01\x12\
    \x04\x89\x01\x08\x11\n\x0c\n\x04\x04\x0c\x02\0\x12\x04\x8a\x01\x04-\n\r\
    \n\x05\x04\x0c\x02\0\x04\x12\x04\x8a\x01\x04\x0c\n\r\n\x05\x04\x0c\x02\0\
    \x05\x12\x04\x8a\x01\r\x13\n\r\n\x05\x04\x0c\x02\0\x01\x12\x04\x8a\x01\
    \x14&\n\r\n\x05\x04\x0c\x02\0\x03\x12\x04\x8a\x01),\n\x0c\n\x04\x04\x0c\
    \x02\x01\x12\x04\x8b\x01\x047\n\r\n\x05\x04\x0c\x02\x01\x04\x12\x04\x8b\
    \x01\x04\x0c\n\r\n\x05\x04\x0c\x02\x01\x06\x12\x04\x8b\x01\r\x18\n\r\n\
    \x05\x04\x0c\x02\x01\x01\x12\x04\x8b\x01\x19/\n\r\n\x05\x04\x0c\x02\x01\
    \x03\x12\x04\x8b\x0126\n\x0c\n\x04\x04\x0c\x02\x02\x12\x04\x8c\x01\x04;\
    \n\r\n\x05\x04\x0c\x02\x02\x04\x12\x04\x8c\x01\x04\x0c\n\r\n\x05\x04\x0c\
    \x02\x02\x06\x12\x04\x8c\x01\r\x18\n\r\n\x05\x04\x0c\x02\x02\x01\x12\x04\
    \x8c\x01\x193\n\r\n\x05\x04\x0c\x02\x02\x03\x12\x04\x8c\x016:\n\x0c\n\
    \x04\x04\x0c\x02\x03\x12\x04\x8d\x01\x04F\n\r\n\x05\x04\x0c\x02\x03\x04\
    \x12\x04\x8d\x01\x04\x0c\n\r\n\x05\x04\x0c\x02\x03\x06\x12\x04\x8d\x01\r\
    \x1f\n\r\n\x05\x04\x0c\x02\x03\x01\x12\x04\x8d\x01\x20>\n\r\n\x05\x04\
    \x0c\x02\x03\x03\x12\x04\x8d\x01AE\n\x0c\n\x04\x04\x0c\x02\x04\x12\x04\
    \x8e\x01\x044\n\r\n\x05\x04\x0c\x02\x04\x04\x12\x04\x8e\x01\x04\x0c\n\r\
    \n\x05\x04\x0c\x02\x04\x05\x12\x04\x8e\x01\r\x12\n\r\n\x05\x04\x0c\x02\
    \x04\x01\x12\x04\x8e\x01\x13,\n\r\n\x05\x04\x0c\x02\x04\x03\x12\x04\x8e\
    \x01/3\n\x0c\n\x04\x04\x0c\x02\x05\x12\x04\x8f\x01\x04%\n\r\n\x05\x04\
    \x0c\x02\x05\x04\x12\x04\x8f\x01\x04\x0c\n\r\n\x05\x04\x0c\x02\x05\x05\
    \x12\x04\x8f\x01\r\x12\n\r\n\x05\x04\x0c\x02\x05\x01\x12\x04\x8f\x01\x13\
    \x1d\n\r\n\x05\x04\x0c\x02\x05\x03\x12\x04\x8f\x01\x20$\n\x0c\n\x04\x04\
    \x0c\x02\x06\x12\x04\x90\x01\x04-\n\r\n\x05\x04\x0c\x02\x06\x04\x12\x04\
    \x90\x01\x04\x0c\n\r\n\x05\x04\x0c\x02\x06\x06\x12\x04\x90\x01\r\x18\n\r\
    \n\x05\x04\x0c\x02\x06\x01\x12\x04\x90\x01\x19%\n\r\n\x05\x04\x0c\x02\
    \x06\x03\x12\x04\x90\x01(,\n\x0c\n\x04\x04\x0c\x02\x07\x12\x04\x91\x01\
    \x04+\n\r\n\x05\x04\x0c\x02\x07\x04\x12\x04\x91\x01\x04\x0c\n\r\n\x05\
    \x04\x0c\x02\x07\x06\x12\x04\x91\x01\r\x20\n\r\n\x05\x04\x0c\x02\x07\x01\
    \x12\x04\x91\x01!#\n\r\n\x05\x04\x0c\x02\x07\x03\x12\x04\x91\x01&*\n\x0c\
    \n\x02\x05\x05\x12\x06\x94\x01\0\x97\x01\x01\n\x0b\n\x03\x05\x05\x01\x12\
    \x04\x94\x01\x05\x10\n\x0c\n\x04\x05\x05\x02\0\x12\x04\x95\x01\x04\x12\n\
    \r\n\x05\x05\x05\x02\0\x01\x12\x04\x95\x01\x04\x0b\n\r\n\x05\x05\x05\x02\
    \0\x02\x12\x04\x95\x01\x0e\x11\n\x0c\n\x04\x05\x05\x02\x01\x12\x04\x96\
    \x01\x04\x13\n\r\n\x05\x05\x05\x02\x01\x01\x12\x04\x96\x01\x04\x0c\n\r\n\
    \x05\x05\x05\x02\x01\x02\x12\x04\x96\x01\x0f\x12\n\x0c\n\x02\x04\r\x12\
    \x06\x99\x01\0\x9c\x01\x01\n\x0b\n\x03\x04\r\x01\x12\x04\x99\x01\x08\x13\
    \n\x0c\n\x04\x04\r\x02\0\x12\x04\x9a\x01\x04.\n\r\n\x05\x04\r\x02\0\x04\
    \x12\x04\x9a\x01\x04\x0c\n\r\n\x05\x04\r\x02\0\x06\x12\x04\x9a\x01\r\x1f\
    \n\r\n\x05\x04\r\x02\0\x01\x12\x04\x9a\x01\x20'\n\r\n\x05\x04\r\x02\0\
    \x03\x12\x04\x9a\x01*-\n\x0c\n\x04\x04\r\x02\x01\x12\x04\x9b\x01\x040\n\
    \r\n\x05\x04\r\x02\x01\x04\x12\x04\x9b\x01\x04\x0c\n\r\n\x05\x04\r\x02\
    \x01\x06\x12\x04\x9b\x01\r\x20\n\r\n\x05\x04\r\x02\x01\x01\x12\x04\x9b\
    \x01!)\n\r\n\x05\x04\r\x02\x01\x03\x12\x04\x9b\x01,/\n\x0c\n\x02\x04\x0e\
    \x12\x06\x9e\x01\0\x9f\x01\x01\n\x0b\n\x03\x04\x0e\x01\x12\x04\x9e\x01\
    \x08\x1a\n\x0c\n\x02\x04\x0f\x12\x06\xa1\x01\0\xa4\x01\x01\n\x0b\n\x03\
    \x04\x0f\x01\x12\x04\xa1\x01\x08\x1b\n\x0c\n\x04\x04\x0f\x02\0\x12\x04\
    \xa2\x01\x04'\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04\xa2\x01\x04\x0c\n\r\n\
    \x05\x04\x0f\x02\0\x05\x12\x04\xa2\x01\r\x13\n\r\n\x05\x04\x0f\x02\0\x01\
    \x12\x04\xa2\x01\x14\x20\n\r\n\x05\x04\x0f\x02\0\x03\x12\x04\xa2\x01#&\n\
    \x0c\n\x04\x04\x0f\x02\x01\x12\x04\xa3\x01\x04%\n\r\n\x05\x04\x0f\x02\
    \x01\x04\x12\x04\xa3\x01\x04\x0c\n\r\n\x05\x04\x0f\x02\x01\x05\x12\x04\
    \xa3\x01\r\x13\n\r\n\x05\x04\x0f\x02\x01\x01\x12\x04\xa3\x01\x14\x1e\n\r\
    \n\x05\x04\x0f\x02\x01\x03\x12\x04\xa3\x01!$\
";

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
