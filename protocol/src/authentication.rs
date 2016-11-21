// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
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
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                ClientResponseEncrypted {
                    login_credentials: ::protobuf::SingularPtrField::none(),
                    account_creation: ::std::option::Option::None,
                    fingerprint_response: ::protobuf::SingularPtrField::none(),
                    peer_ticket: ::protobuf::SingularPtrField::none(),
                    system_info: ::protobuf::SingularPtrField::none(),
                    platform_model: ::protobuf::SingularField::none(),
                    version_string: ::protobuf::SingularField::none(),
                    appkey: ::protobuf::SingularPtrField::none(),
                    client_info: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
        self.login_credentials.as_mut().unwrap()
    }

    // Take field
    pub fn take_login_credentials(&mut self) -> LoginCredentials {
        self.login_credentials.take().unwrap_or_else(|| LoginCredentials::new())
    }

    pub fn get_login_credentials(&self) -> &LoginCredentials {
        self.login_credentials.as_ref().unwrap_or_else(|| LoginCredentials::default_instance())
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
        };
        self.fingerprint_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_fingerprint_response(&mut self) -> FingerprintResponseUnion {
        self.fingerprint_response.take().unwrap_or_else(|| FingerprintResponseUnion::new())
    }

    pub fn get_fingerprint_response(&self) -> &FingerprintResponseUnion {
        self.fingerprint_response.as_ref().unwrap_or_else(|| FingerprintResponseUnion::default_instance())
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
        };
        self.peer_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer_ticket(&mut self) -> PeerTicketUnion {
        self.peer_ticket.take().unwrap_or_else(|| PeerTicketUnion::new())
    }

    pub fn get_peer_ticket(&self) -> &PeerTicketUnion {
        self.peer_ticket.as_ref().unwrap_or_else(|| PeerTicketUnion::default_instance())
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
        };
        self.system_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_system_info(&mut self) -> SystemInfo {
        self.system_info.take().unwrap_or_else(|| SystemInfo::new())
    }

    pub fn get_system_info(&self) -> &SystemInfo {
        self.system_info.as_ref().unwrap_or_else(|| SystemInfo::default_instance())
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
        };
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
        };
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
        };
        self.appkey.as_mut().unwrap()
    }

    // Take field
    pub fn take_appkey(&mut self) -> LibspotifyAppKey {
        self.appkey.take().unwrap_or_else(|| LibspotifyAppKey::new())
    }

    pub fn get_appkey(&self) -> &LibspotifyAppKey {
        self.appkey.as_ref().unwrap_or_else(|| LibspotifyAppKey::default_instance())
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
        };
        self.client_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_info(&mut self) -> ClientInfo {
        self.client_info.take().unwrap_or_else(|| ClientInfo::new())
    }

    pub fn get_client_info(&self) -> &ClientInfo {
        self.client_info.as_ref().unwrap_or_else(|| ClientInfo::default_instance())
    }
}

impl ::protobuf::Message for ClientResponseEncrypted {
    fn is_initialized(&self) -> bool {
        if self.login_credentials.is_none() {
            return false;
        };
        if self.system_info.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login_credentials));
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.account_creation = ::std::option::Option::Some(tmp);
                },
                30 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fingerprint_response));
                },
                40 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer_ticket));
                },
                50 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.system_info));
                },
                60 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.platform_model));
                },
                70 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version_string));
                },
                80 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.appkey));
                },
                90 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.client_info));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.login_credentials {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.account_creation {
            my_size += ::protobuf::rt::enum_size(20, *value);
        };
        for value in &self.fingerprint_response {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.peer_ticket {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.system_info {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.platform_model {
            my_size += ::protobuf::rt::string_size(60, &value);
        };
        for value in &self.version_string {
            my_size += ::protobuf::rt::string_size(70, &value);
        };
        for value in &self.appkey {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.client_info {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.login_credentials.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.account_creation {
            try!(os.write_enum(20, v.value()));
        };
        if let Some(v) = self.fingerprint_response.as_ref() {
            try!(os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.peer_ticket.as_ref() {
            try!(os.write_tag(40, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.system_info.as_ref() {
            try!(os.write_tag(50, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.platform_model.as_ref() {
            try!(os.write_string(60, &v));
        };
        if let Some(v) = self.version_string.as_ref() {
            try!(os.write_string(70, &v));
        };
        if let Some(v) = self.appkey.as_ref() {
            try!(os.write_tag(80, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.client_info.as_ref() {
            try!(os.write_tag(90, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ClientResponseEncrypted>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "login_credentials",
                    ClientResponseEncrypted::has_login_credentials,
                    ClientResponseEncrypted::get_login_credentials,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "account_creation",
                    ClientResponseEncrypted::has_account_creation,
                    ClientResponseEncrypted::get_account_creation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fingerprint_response",
                    ClientResponseEncrypted::has_fingerprint_response,
                    ClientResponseEncrypted::get_fingerprint_response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "peer_ticket",
                    ClientResponseEncrypted::has_peer_ticket,
                    ClientResponseEncrypted::get_peer_ticket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "system_info",
                    ClientResponseEncrypted::has_system_info,
                    ClientResponseEncrypted::get_system_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "platform_model",
                    ClientResponseEncrypted::has_platform_model,
                    ClientResponseEncrypted::get_platform_model,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "version_string",
                    ClientResponseEncrypted::has_version_string,
                    ClientResponseEncrypted::get_version_string,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "appkey",
                    ClientResponseEncrypted::has_appkey,
                    ClientResponseEncrypted::get_appkey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "client_info",
                    ClientResponseEncrypted::has_client_info,
                    ClientResponseEncrypted::get_client_info,
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

impl ::std::cmp::PartialEq for ClientResponseEncrypted {
    fn eq(&self, other: &ClientResponseEncrypted) -> bool {
        self.login_credentials == other.login_credentials &&
        self.account_creation == other.account_creation &&
        self.fingerprint_response == other.fingerprint_response &&
        self.peer_ticket == other.peer_ticket &&
        self.system_info == other.system_info &&
        self.platform_model == other.platform_model &&
        self.version_string == other.version_string &&
        self.appkey == other.appkey &&
        self.client_info == other.client_info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientResponseEncrypted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LoginCredentials {
    // message fields
    username: ::protobuf::SingularField<::std::string::String>,
    typ: ::std::option::Option<AuthenticationType>,
    auth_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                LoginCredentials {
                    username: ::protobuf::SingularField::none(),
                    typ: ::std::option::Option::None,
                    auth_data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
}

impl ::protobuf::Message for LoginCredentials {
    fn is_initialized(&self) -> bool {
        if self.typ.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.username));
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.typ = ::std::option::Option::Some(tmp);
                },
                30 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.auth_data));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.username {
            my_size += ::protobuf::rt::string_size(10, &value);
        };
        for value in &self.typ {
            my_size += ::protobuf::rt::enum_size(20, *value);
        };
        for value in &self.auth_data {
            my_size += ::protobuf::rt::bytes_size(30, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.username.as_ref() {
            try!(os.write_string(10, &v));
        };
        if let Some(v) = self.typ {
            try!(os.write_enum(20, v.value()));
        };
        if let Some(v) = self.auth_data.as_ref() {
            try!(os.write_bytes(30, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<LoginCredentials>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "username",
                    LoginCredentials::has_username,
                    LoginCredentials::get_username,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "typ",
                    LoginCredentials::has_typ,
                    LoginCredentials::get_typ,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "auth_data",
                    LoginCredentials::has_auth_data,
                    LoginCredentials::get_auth_data,
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

impl ::std::cmp::PartialEq for LoginCredentials {
    fn eq(&self, other: &LoginCredentials) -> bool {
        self.username == other.username &&
        self.typ == other.typ &&
        self.auth_data == other.auth_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LoginCredentials {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FingerprintResponseUnion {
    // message fields
    grain: ::protobuf::SingularPtrField<FingerprintGrainResponse>,
    hmac_ripemd: ::protobuf::SingularPtrField<FingerprintHmacRipemdResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                FingerprintResponseUnion {
                    grain: ::protobuf::SingularPtrField::none(),
                    hmac_ripemd: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
        self.grain.as_mut().unwrap()
    }

    // Take field
    pub fn take_grain(&mut self) -> FingerprintGrainResponse {
        self.grain.take().unwrap_or_else(|| FingerprintGrainResponse::new())
    }

    pub fn get_grain(&self) -> &FingerprintGrainResponse {
        self.grain.as_ref().unwrap_or_else(|| FingerprintGrainResponse::default_instance())
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
        };
        self.hmac_ripemd.as_mut().unwrap()
    }

    // Take field
    pub fn take_hmac_ripemd(&mut self) -> FingerprintHmacRipemdResponse {
        self.hmac_ripemd.take().unwrap_or_else(|| FingerprintHmacRipemdResponse::new())
    }

    pub fn get_hmac_ripemd(&self) -> &FingerprintHmacRipemdResponse {
        self.hmac_ripemd.as_ref().unwrap_or_else(|| FingerprintHmacRipemdResponse::default_instance())
    }
}

impl ::protobuf::Message for FingerprintResponseUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.grain));
                },
                20 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hmac_ripemd));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.grain {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.hmac_ripemd {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.grain.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.hmac_ripemd.as_ref() {
            try!(os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FingerprintResponseUnion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "grain",
                    FingerprintResponseUnion::has_grain,
                    FingerprintResponseUnion::get_grain,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "hmac_ripemd",
                    FingerprintResponseUnion::has_hmac_ripemd,
                    FingerprintResponseUnion::get_hmac_ripemd,
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

impl ::std::cmp::PartialEq for FingerprintResponseUnion {
    fn eq(&self, other: &FingerprintResponseUnion) -> bool {
        self.grain == other.grain &&
        self.hmac_ripemd == other.hmac_ripemd &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FingerprintResponseUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FingerprintGrainResponse {
    // message fields
    encrypted_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                FingerprintGrainResponse {
                    encrypted_key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
}

impl ::protobuf::Message for FingerprintGrainResponse {
    fn is_initialized(&self) -> bool {
        if self.encrypted_key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encrypted_key));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.encrypted_key {
            my_size += ::protobuf::rt::bytes_size(10, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.encrypted_key.as_ref() {
            try!(os.write_bytes(10, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FingerprintGrainResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "encrypted_key",
                    FingerprintGrainResponse::has_encrypted_key,
                    FingerprintGrainResponse::get_encrypted_key,
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

impl ::std::cmp::PartialEq for FingerprintGrainResponse {
    fn eq(&self, other: &FingerprintGrainResponse) -> bool {
        self.encrypted_key == other.encrypted_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FingerprintGrainResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FingerprintHmacRipemdResponse {
    // message fields
    hmac: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                FingerprintHmacRipemdResponse {
                    hmac: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
}

impl ::protobuf::Message for FingerprintHmacRipemdResponse {
    fn is_initialized(&self) -> bool {
        if self.hmac.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hmac));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.hmac {
            my_size += ::protobuf::rt::bytes_size(10, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hmac.as_ref() {
            try!(os.write_bytes(10, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FingerprintHmacRipemdResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "hmac",
                    FingerprintHmacRipemdResponse::has_hmac,
                    FingerprintHmacRipemdResponse::get_hmac,
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

impl ::std::cmp::PartialEq for FingerprintHmacRipemdResponse {
    fn eq(&self, other: &FingerprintHmacRipemdResponse) -> bool {
        self.hmac == other.hmac &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FingerprintHmacRipemdResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PeerTicketUnion {
    // message fields
    public_key: ::protobuf::SingularPtrField<PeerTicketPublicKey>,
    old_ticket: ::protobuf::SingularPtrField<PeerTicketOld>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                PeerTicketUnion {
                    public_key: ::protobuf::SingularPtrField::none(),
                    old_ticket: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> PeerTicketPublicKey {
        self.public_key.take().unwrap_or_else(|| PeerTicketPublicKey::new())
    }

    pub fn get_public_key(&self) -> &PeerTicketPublicKey {
        self.public_key.as_ref().unwrap_or_else(|| PeerTicketPublicKey::default_instance())
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
        };
        self.old_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_old_ticket(&mut self) -> PeerTicketOld {
        self.old_ticket.take().unwrap_or_else(|| PeerTicketOld::new())
    }

    pub fn get_old_ticket(&self) -> &PeerTicketOld {
        self.old_ticket.as_ref().unwrap_or_else(|| PeerTicketOld::default_instance())
    }
}

impl ::protobuf::Message for PeerTicketUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.public_key));
                },
                20 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.old_ticket));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.public_key {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.old_ticket {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.public_key.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.old_ticket.as_ref() {
            try!(os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PeerTicketUnion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "public_key",
                    PeerTicketUnion::has_public_key,
                    PeerTicketUnion::get_public_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "old_ticket",
                    PeerTicketUnion::has_old_ticket,
                    PeerTicketUnion::get_old_ticket,
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

impl ::std::cmp::PartialEq for PeerTicketUnion {
    fn eq(&self, other: &PeerTicketUnion) -> bool {
        self.public_key == other.public_key &&
        self.old_ticket == other.old_ticket &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PeerTicketUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PeerTicketPublicKey {
    // message fields
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                PeerTicketPublicKey {
                    public_key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
}

impl ::protobuf::Message for PeerTicketPublicKey {
    fn is_initialized(&self) -> bool {
        if self.public_key.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.public_key));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.public_key {
            my_size += ::protobuf::rt::bytes_size(10, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.public_key.as_ref() {
            try!(os.write_bytes(10, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PeerTicketPublicKey>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "public_key",
                    PeerTicketPublicKey::has_public_key,
                    PeerTicketPublicKey::get_public_key,
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

impl ::std::cmp::PartialEq for PeerTicketPublicKey {
    fn eq(&self, other: &PeerTicketPublicKey) -> bool {
        self.public_key == other.public_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PeerTicketPublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PeerTicketOld {
    // message fields
    peer_ticket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    peer_ticket_signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                PeerTicketOld {
                    peer_ticket: ::protobuf::SingularField::none(),
                    peer_ticket_signature: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
}

impl ::protobuf::Message for PeerTicketOld {
    fn is_initialized(&self) -> bool {
        if self.peer_ticket.is_none() {
            return false;
        };
        if self.peer_ticket_signature.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.peer_ticket));
                },
                20 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.peer_ticket_signature));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.peer_ticket {
            my_size += ::protobuf::rt::bytes_size(10, &value);
        };
        for value in &self.peer_ticket_signature {
            my_size += ::protobuf::rt::bytes_size(20, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.peer_ticket.as_ref() {
            try!(os.write_bytes(10, &v));
        };
        if let Some(v) = self.peer_ticket_signature.as_ref() {
            try!(os.write_bytes(20, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PeerTicketOld>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "peer_ticket",
                    PeerTicketOld::has_peer_ticket,
                    PeerTicketOld::get_peer_ticket,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "peer_ticket_signature",
                    PeerTicketOld::has_peer_ticket_signature,
                    PeerTicketOld::get_peer_ticket_signature,
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

impl ::std::cmp::PartialEq for PeerTicketOld {
    fn eq(&self, other: &PeerTicketOld) -> bool {
        self.peer_ticket == other.peer_ticket &&
        self.peer_ticket_signature == other.peer_ticket_signature &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PeerTicketOld {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
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
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                SystemInfo {
                    cpu_family: ::std::option::Option::None,
                    cpu_subtype: ::std::option::Option::None,
                    cpu_ext: ::std::option::Option::None,
                    brand: ::std::option::Option::None,
                    brand_flags: ::std::option::Option::None,
                    os: ::std::option::Option::None,
                    os_version: ::std::option::Option::None,
                    os_ext: ::std::option::Option::None,
                    system_information_string: ::protobuf::SingularField::none(),
                    device_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
}

impl ::protobuf::Message for SystemInfo {
    fn is_initialized(&self) -> bool {
        if self.cpu_family.is_none() {
            return false;
        };
        if self.os.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.cpu_family = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.cpu_subtype = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.cpu_ext = ::std::option::Option::Some(tmp);
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.brand = ::std::option::Option::Some(tmp);
                },
                50 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.brand_flags = ::std::option::Option::Some(tmp);
                },
                60 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.os = ::std::option::Option::Some(tmp);
                },
                70 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.os_version = ::std::option::Option::Some(tmp);
                },
                80 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.os_ext = ::std::option::Option::Some(tmp);
                },
                90 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.system_information_string));
                },
                100 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.device_id));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.cpu_family {
            my_size += ::protobuf::rt::enum_size(10, *value);
        };
        for value in &self.cpu_subtype {
            my_size += ::protobuf::rt::value_size(20, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.cpu_ext {
            my_size += ::protobuf::rt::value_size(30, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.brand {
            my_size += ::protobuf::rt::enum_size(40, *value);
        };
        for value in &self.brand_flags {
            my_size += ::protobuf::rt::value_size(50, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.os {
            my_size += ::protobuf::rt::enum_size(60, *value);
        };
        for value in &self.os_version {
            my_size += ::protobuf::rt::value_size(70, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.os_ext {
            my_size += ::protobuf::rt::value_size(80, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.system_information_string {
            my_size += ::protobuf::rt::string_size(90, &value);
        };
        for value in &self.device_id {
            my_size += ::protobuf::rt::string_size(100, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cpu_family {
            try!(os.write_enum(10, v.value()));
        };
        if let Some(v) = self.cpu_subtype {
            try!(os.write_uint32(20, v));
        };
        if let Some(v) = self.cpu_ext {
            try!(os.write_uint32(30, v));
        };
        if let Some(v) = self.brand {
            try!(os.write_enum(40, v.value()));
        };
        if let Some(v) = self.brand_flags {
            try!(os.write_uint32(50, v));
        };
        if let Some(v) = self.os {
            try!(os.write_enum(60, v.value()));
        };
        if let Some(v) = self.os_version {
            try!(os.write_uint32(70, v));
        };
        if let Some(v) = self.os_ext {
            try!(os.write_uint32(80, v));
        };
        if let Some(v) = self.system_information_string.as_ref() {
            try!(os.write_string(90, &v));
        };
        if let Some(v) = self.device_id.as_ref() {
            try!(os.write_string(100, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SystemInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "cpu_family",
                    SystemInfo::has_cpu_family,
                    SystemInfo::get_cpu_family,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "cpu_subtype",
                    SystemInfo::has_cpu_subtype,
                    SystemInfo::get_cpu_subtype,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "cpu_ext",
                    SystemInfo::has_cpu_ext,
                    SystemInfo::get_cpu_ext,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "brand",
                    SystemInfo::has_brand,
                    SystemInfo::get_brand,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "brand_flags",
                    SystemInfo::has_brand_flags,
                    SystemInfo::get_brand_flags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "os",
                    SystemInfo::has_os,
                    SystemInfo::get_os,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "os_version",
                    SystemInfo::has_os_version,
                    SystemInfo::get_os_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "os_ext",
                    SystemInfo::has_os_ext,
                    SystemInfo::get_os_ext,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "system_information_string",
                    SystemInfo::has_system_information_string,
                    SystemInfo::get_system_information_string,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "device_id",
                    SystemInfo::has_device_id,
                    SystemInfo::get_device_id,
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

impl ::std::cmp::PartialEq for SystemInfo {
    fn eq(&self, other: &SystemInfo) -> bool {
        self.cpu_family == other.cpu_family &&
        self.cpu_subtype == other.cpu_subtype &&
        self.cpu_ext == other.cpu_ext &&
        self.brand == other.brand &&
        self.brand_flags == other.brand_flags &&
        self.os == other.os &&
        self.os_version == other.os_version &&
        self.os_ext == other.os_ext &&
        self.system_information_string == other.system_information_string &&
        self.device_id == other.device_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SystemInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LibspotifyAppKey {
    // message fields
    version: ::std::option::Option<u32>,
    devkey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    useragent: ::protobuf::SingularField<::std::string::String>,
    callback_hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                LibspotifyAppKey {
                    version: ::std::option::Option::None,
                    devkey: ::protobuf::SingularField::none(),
                    signature: ::protobuf::SingularField::none(),
                    useragent: ::protobuf::SingularField::none(),
                    callback_hash: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
        };
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
        };
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
}

impl ::protobuf::Message for LibspotifyAppKey {
    fn is_initialized(&self) -> bool {
        if self.version.is_none() {
            return false;
        };
        if self.devkey.is_none() {
            return false;
        };
        if self.signature.is_none() {
            return false;
        };
        if self.useragent.is_none() {
            return false;
        };
        if self.callback_hash.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.devkey));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.useragent));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.callback_hash));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.version {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.devkey {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.signature {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in &self.useragent {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.callback_hash {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.devkey.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.signature.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.useragent.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.callback_hash.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<LibspotifyAppKey>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "version",
                    LibspotifyAppKey::has_version,
                    LibspotifyAppKey::get_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "devkey",
                    LibspotifyAppKey::has_devkey,
                    LibspotifyAppKey::get_devkey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "signature",
                    LibspotifyAppKey::has_signature,
                    LibspotifyAppKey::get_signature,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "useragent",
                    LibspotifyAppKey::has_useragent,
                    LibspotifyAppKey::get_useragent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "callback_hash",
                    LibspotifyAppKey::has_callback_hash,
                    LibspotifyAppKey::get_callback_hash,
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

impl ::std::cmp::PartialEq for LibspotifyAppKey {
    fn eq(&self, other: &LibspotifyAppKey) -> bool {
        self.version == other.version &&
        self.devkey == other.devkey &&
        self.signature == other.signature &&
        self.useragent == other.useragent &&
        self.callback_hash == other.callback_hash &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LibspotifyAppKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ClientInfo {
    // message fields
    limited: ::std::option::Option<bool>,
    fb: ::protobuf::SingularPtrField<ClientInfoFacebook>,
    language: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                ClientInfo {
                    limited: ::std::option::Option::None,
                    fb: ::protobuf::SingularPtrField::none(),
                    language: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
        self.fb.as_mut().unwrap()
    }

    // Take field
    pub fn take_fb(&mut self) -> ClientInfoFacebook {
        self.fb.take().unwrap_or_else(|| ClientInfoFacebook::new())
    }

    pub fn get_fb(&self) -> &ClientInfoFacebook {
        self.fb.as_ref().unwrap_or_else(|| ClientInfoFacebook::default_instance())
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
        };
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
}

impl ::protobuf::Message for ClientInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.limited = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fb));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.language));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.limited.is_some() {
            my_size += 2;
        };
        for value in &self.fb {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.language {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.limited {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.fb.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.language.as_ref() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ClientInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "limited",
                    ClientInfo::has_limited,
                    ClientInfo::get_limited,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fb",
                    ClientInfo::has_fb,
                    ClientInfo::get_fb,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "language",
                    ClientInfo::has_language,
                    ClientInfo::get_language,
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

impl ::std::cmp::PartialEq for ClientInfo {
    fn eq(&self, other: &ClientInfo) -> bool {
        self.limited == other.limited &&
        self.fb == other.fb &&
        self.language == other.language &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ClientInfoFacebook {
    // message fields
    machine_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                ClientInfoFacebook {
                    machine_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
}

impl ::protobuf::Message for ClientInfoFacebook {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.machine_id));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.machine_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.machine_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ClientInfoFacebook>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "machine_id",
                    ClientInfoFacebook::has_machine_id,
                    ClientInfoFacebook::get_machine_id,
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

impl ::std::cmp::PartialEq for ClientInfoFacebook {
    fn eq(&self, other: &ClientInfoFacebook) -> bool {
        self.machine_id == other.machine_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientInfoFacebook {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
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
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                APWelcome {
                    canonical_username: ::protobuf::SingularField::none(),
                    account_type_logged_in: ::std::option::Option::None,
                    credentials_type_logged_in: ::std::option::Option::None,
                    reusable_auth_credentials_type: ::std::option::Option::None,
                    reusable_auth_credentials: ::protobuf::SingularField::none(),
                    lfs_secret: ::protobuf::SingularField::none(),
                    account_info: ::protobuf::SingularPtrField::none(),
                    fb: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
        };
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
        };
        self.account_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_info(&mut self) -> AccountInfo {
        self.account_info.take().unwrap_or_else(|| AccountInfo::new())
    }

    pub fn get_account_info(&self) -> &AccountInfo {
        self.account_info.as_ref().unwrap_or_else(|| AccountInfo::default_instance())
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
        };
        self.fb.as_mut().unwrap()
    }

    // Take field
    pub fn take_fb(&mut self) -> AccountInfoFacebook {
        self.fb.take().unwrap_or_else(|| AccountInfoFacebook::new())
    }

    pub fn get_fb(&self) -> &AccountInfoFacebook {
        self.fb.as_ref().unwrap_or_else(|| AccountInfoFacebook::default_instance())
    }
}

impl ::protobuf::Message for APWelcome {
    fn is_initialized(&self) -> bool {
        if self.canonical_username.is_none() {
            return false;
        };
        if self.account_type_logged_in.is_none() {
            return false;
        };
        if self.credentials_type_logged_in.is_none() {
            return false;
        };
        if self.reusable_auth_credentials_type.is_none() {
            return false;
        };
        if self.reusable_auth_credentials.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.canonical_username));
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.account_type_logged_in = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.credentials_type_logged_in = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.reusable_auth_credentials_type = ::std::option::Option::Some(tmp);
                },
                40 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.reusable_auth_credentials));
                },
                50 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.lfs_secret));
                },
                60 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.account_info));
                },
                70 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fb));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.canonical_username {
            my_size += ::protobuf::rt::string_size(10, &value);
        };
        for value in &self.account_type_logged_in {
            my_size += ::protobuf::rt::enum_size(20, *value);
        };
        for value in &self.credentials_type_logged_in {
            my_size += ::protobuf::rt::enum_size(25, *value);
        };
        for value in &self.reusable_auth_credentials_type {
            my_size += ::protobuf::rt::enum_size(30, *value);
        };
        for value in &self.reusable_auth_credentials {
            my_size += ::protobuf::rt::bytes_size(40, &value);
        };
        for value in &self.lfs_secret {
            my_size += ::protobuf::rt::bytes_size(50, &value);
        };
        for value in &self.account_info {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.fb {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.canonical_username.as_ref() {
            try!(os.write_string(10, &v));
        };
        if let Some(v) = self.account_type_logged_in {
            try!(os.write_enum(20, v.value()));
        };
        if let Some(v) = self.credentials_type_logged_in {
            try!(os.write_enum(25, v.value()));
        };
        if let Some(v) = self.reusable_auth_credentials_type {
            try!(os.write_enum(30, v.value()));
        };
        if let Some(v) = self.reusable_auth_credentials.as_ref() {
            try!(os.write_bytes(40, &v));
        };
        if let Some(v) = self.lfs_secret.as_ref() {
            try!(os.write_bytes(50, &v));
        };
        if let Some(v) = self.account_info.as_ref() {
            try!(os.write_tag(60, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.fb.as_ref() {
            try!(os.write_tag(70, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<APWelcome>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "canonical_username",
                    APWelcome::has_canonical_username,
                    APWelcome::get_canonical_username,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "account_type_logged_in",
                    APWelcome::has_account_type_logged_in,
                    APWelcome::get_account_type_logged_in,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "credentials_type_logged_in",
                    APWelcome::has_credentials_type_logged_in,
                    APWelcome::get_credentials_type_logged_in,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "reusable_auth_credentials_type",
                    APWelcome::has_reusable_auth_credentials_type,
                    APWelcome::get_reusable_auth_credentials_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "reusable_auth_credentials",
                    APWelcome::has_reusable_auth_credentials,
                    APWelcome::get_reusable_auth_credentials,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "lfs_secret",
                    APWelcome::has_lfs_secret,
                    APWelcome::get_lfs_secret,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "account_info",
                    APWelcome::has_account_info,
                    APWelcome::get_account_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fb",
                    APWelcome::has_fb,
                    APWelcome::get_fb,
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

impl ::std::cmp::PartialEq for APWelcome {
    fn eq(&self, other: &APWelcome) -> bool {
        self.canonical_username == other.canonical_username &&
        self.account_type_logged_in == other.account_type_logged_in &&
        self.credentials_type_logged_in == other.credentials_type_logged_in &&
        self.reusable_auth_credentials_type == other.reusable_auth_credentials_type &&
        self.reusable_auth_credentials == other.reusable_auth_credentials &&
        self.lfs_secret == other.lfs_secret &&
        self.account_info == other.account_info &&
        self.fb == other.fb &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for APWelcome {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AccountInfo {
    // message fields
    spotify: ::protobuf::SingularPtrField<AccountInfoSpotify>,
    facebook: ::protobuf::SingularPtrField<AccountInfoFacebook>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                AccountInfo {
                    spotify: ::protobuf::SingularPtrField::none(),
                    facebook: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
        self.spotify.as_mut().unwrap()
    }

    // Take field
    pub fn take_spotify(&mut self) -> AccountInfoSpotify {
        self.spotify.take().unwrap_or_else(|| AccountInfoSpotify::new())
    }

    pub fn get_spotify(&self) -> &AccountInfoSpotify {
        self.spotify.as_ref().unwrap_or_else(|| AccountInfoSpotify::default_instance())
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
        };
        self.facebook.as_mut().unwrap()
    }

    // Take field
    pub fn take_facebook(&mut self) -> AccountInfoFacebook {
        self.facebook.take().unwrap_or_else(|| AccountInfoFacebook::new())
    }

    pub fn get_facebook(&self) -> &AccountInfoFacebook {
        self.facebook.as_ref().unwrap_or_else(|| AccountInfoFacebook::default_instance())
    }
}

impl ::protobuf::Message for AccountInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.spotify));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.facebook));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.spotify {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.facebook {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.spotify.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.facebook.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AccountInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "spotify",
                    AccountInfo::has_spotify,
                    AccountInfo::get_spotify,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "facebook",
                    AccountInfo::has_facebook,
                    AccountInfo::get_facebook,
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

impl ::std::cmp::PartialEq for AccountInfo {
    fn eq(&self, other: &AccountInfo) -> bool {
        self.spotify == other.spotify &&
        self.facebook == other.facebook &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AccountInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AccountInfoSpotify {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                AccountInfoSpotify {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for AccountInfoSpotify {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AccountInfoSpotify>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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

impl ::std::cmp::PartialEq for AccountInfoSpotify {
    fn eq(&self, other: &AccountInfoSpotify) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AccountInfoSpotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AccountInfoFacebook {
    // message fields
    access_token: ::protobuf::SingularField<::std::string::String>,
    machine_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                AccountInfoFacebook {
                    access_token: ::protobuf::SingularField::none(),
                    machine_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
        };
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
        };
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
}

impl ::protobuf::Message for AccountInfoFacebook {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.access_token));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.machine_id));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.access_token {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.machine_id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.access_token.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.machine_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AccountInfoFacebook>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "access_token",
                    AccountInfoFacebook::has_access_token,
                    AccountInfoFacebook::get_access_token,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "machine_id",
                    AccountInfoFacebook::has_machine_id,
                    AccountInfoFacebook::get_machine_id,
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

impl ::std::cmp::PartialEq for AccountInfoFacebook {
    fn eq(&self, other: &AccountInfoFacebook) -> bool {
        self.access_token == other.access_token &&
        self.machine_id == other.machine_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AccountInfoFacebook {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
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

    fn enum_descriptor_static(_: Option<AuthenticationType>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

    fn enum_descriptor_static(_: Option<AccountCreation>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

    fn enum_descriptor_static(_: Option<CpuFamily>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

    fn enum_descriptor_static(_: Option<Brand>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

    fn enum_descriptor_static(_: Option<Os>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

    fn enum_descriptor_static(_: Option<AccountType>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x61, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xec, 0x03, 0x0a, 0x17, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74,
    0x65, 0x64, 0x12, 0x3e, 0x0a, 0x11, 0x6c, 0x6f, 0x67, 0x69, 0x6e, 0x5f, 0x63, 0x72, 0x65, 0x64,
    0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73,
    0x52, 0x10, 0x6c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61,
    0x6c, 0x73, 0x12, 0x3b, 0x0a, 0x10, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x63, 0x72,
    0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x10, 0x2e, 0x41,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x43, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0f,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x43, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x4c, 0x0a, 0x14, 0x66, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x5f, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x1e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e,
    0x46, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x52, 0x13, 0x66, 0x69, 0x6e, 0x67, 0x65, 0x72,
    0x70, 0x72, 0x69, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x31, 0x0a,
    0x0b, 0x70, 0x65, 0x65, 0x72, 0x5f, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x28, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x55,
    0x6e, 0x69, 0x6f, 0x6e, 0x52, 0x0a, 0x70, 0x65, 0x65, 0x72, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74,
    0x12, 0x2c, 0x0a, 0x0b, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18,
    0x32, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x49, 0x6e,
    0x66, 0x6f, 0x52, 0x0a, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x25,
    0x0a, 0x0e, 0x70, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x6c,
    0x18, 0x3c, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x70, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d,
    0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x12, 0x25, 0x0a, 0x0e, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x46, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x12, 0x29, 0x0a, 0x06,
    0x61, 0x70, 0x70, 0x6b, 0x65, 0x79, 0x18, 0x50, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x4c,
    0x69, 0x62, 0x73, 0x70, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x41, 0x70, 0x70, 0x4b, 0x65, 0x79, 0x52,
    0x06, 0x61, 0x70, 0x70, 0x6b, 0x65, 0x79, 0x12, 0x2c, 0x0a, 0x0b, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x5a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x0a, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x72, 0x0a, 0x10, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72,
    0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65,
    0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65,
    0x72, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x25, 0x0a, 0x03, 0x74, 0x79, 0x70, 0x18, 0x14, 0x20, 0x02,
    0x28, 0x0e, 0x32, 0x13, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x03, 0x74, 0x79, 0x70, 0x12, 0x1b, 0x0a, 0x09,
    0x61, 0x75, 0x74, 0x68, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x1e, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x08, 0x61, 0x75, 0x74, 0x68, 0x44, 0x61, 0x74, 0x61, 0x22, 0x8c, 0x01, 0x0a, 0x18, 0x46, 0x69,
    0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x2f, 0x0a, 0x05, 0x67, 0x72, 0x61, 0x69, 0x6e, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x46, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72,
    0x69, 0x6e, 0x74, 0x47, 0x72, 0x61, 0x69, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x52, 0x05, 0x67, 0x72, 0x61, 0x69, 0x6e, 0x12, 0x3f, 0x0a, 0x0b, 0x68, 0x6d, 0x61, 0x63, 0x5f,
    0x72, 0x69, 0x70, 0x65, 0x6d, 0x64, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x46,
    0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x48, 0x6d, 0x61, 0x63, 0x52, 0x69,
    0x70, 0x65, 0x6d, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0a, 0x68, 0x6d,
    0x61, 0x63, 0x52, 0x69, 0x70, 0x65, 0x6d, 0x64, 0x22, 0x3f, 0x0a, 0x18, 0x46, 0x69, 0x6e, 0x67,
    0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x47, 0x72, 0x61, 0x69, 0x6e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65,
    0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x0c, 0x65, 0x6e, 0x63,
    0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x4b, 0x65, 0x79, 0x22, 0x33, 0x0a, 0x1d, 0x46, 0x69, 0x6e,
    0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x48, 0x6d, 0x61, 0x63, 0x52, 0x69, 0x70, 0x65,
    0x6d, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x6d,
    0x61, 0x63, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x04, 0x68, 0x6d, 0x61, 0x63, 0x22, 0x75,
    0x0a, 0x0f, 0x50, 0x65, 0x65, 0x72, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x55, 0x6e, 0x69, 0x6f,
    0x6e, 0x12, 0x33, 0x0a, 0x0a, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x54, 0x69, 0x63, 0x6b,
    0x65, 0x74, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x52, 0x09, 0x70, 0x75, 0x62,
    0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x12, 0x2d, 0x0a, 0x0a, 0x6f, 0x6c, 0x64, 0x5f, 0x74, 0x69,
    0x63, 0x6b, 0x65, 0x74, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x50, 0x65, 0x65,
    0x72, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x4f, 0x6c, 0x64, 0x52, 0x09, 0x6f, 0x6c, 0x64, 0x54,
    0x69, 0x63, 0x6b, 0x65, 0x74, 0x22, 0x34, 0x0a, 0x13, 0x50, 0x65, 0x65, 0x72, 0x54, 0x69, 0x63,
    0x6b, 0x65, 0x74, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x12, 0x1d, 0x0a, 0x0a,
    0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0c,
    0x52, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x22, 0x64, 0x0a, 0x0d, 0x50,
    0x65, 0x65, 0x72, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x4f, 0x6c, 0x64, 0x12, 0x1f, 0x0a, 0x0b,
    0x70, 0x65, 0x65, 0x72, 0x5f, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x0a, 0x20, 0x02, 0x28,
    0x0c, 0x52, 0x0a, 0x70, 0x65, 0x65, 0x72, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x32, 0x0a,
    0x15, 0x70, 0x65, 0x65, 0x72, 0x5f, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x73, 0x69, 0x67,
    0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x14, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x13, 0x70, 0x65,
    0x65, 0x72, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72,
    0x65, 0x22, 0xd4, 0x02, 0x0a, 0x0a, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x49, 0x6e, 0x66, 0x6f,
    0x12, 0x29, 0x0a, 0x0a, 0x63, 0x70, 0x75, 0x5f, 0x66, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x18, 0x0a,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x0a, 0x2e, 0x43, 0x70, 0x75, 0x46, 0x61, 0x6d, 0x69, 0x6c, 0x79,
    0x52, 0x09, 0x63, 0x70, 0x75, 0x46, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x12, 0x1f, 0x0a, 0x0b, 0x63,
    0x70, 0x75, 0x5f, 0x73, 0x75, 0x62, 0x74, 0x79, 0x70, 0x65, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0a, 0x63, 0x70, 0x75, 0x53, 0x75, 0x62, 0x74, 0x79, 0x70, 0x65, 0x12, 0x17, 0x0a, 0x07,
    0x63, 0x70, 0x75, 0x5f, 0x65, 0x78, 0x74, 0x18, 0x1e, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x63,
    0x70, 0x75, 0x45, 0x78, 0x74, 0x12, 0x1c, 0x0a, 0x05, 0x62, 0x72, 0x61, 0x6e, 0x64, 0x18, 0x28,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x06, 0x2e, 0x42, 0x72, 0x61, 0x6e, 0x64, 0x52, 0x05, 0x62, 0x72,
    0x61, 0x6e, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x72, 0x61, 0x6e, 0x64, 0x5f, 0x66, 0x6c, 0x61,
    0x67, 0x73, 0x18, 0x32, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x62, 0x72, 0x61, 0x6e, 0x64, 0x46,
    0x6c, 0x61, 0x67, 0x73, 0x12, 0x13, 0x0a, 0x02, 0x6f, 0x73, 0x18, 0x3c, 0x20, 0x02, 0x28, 0x0e,
    0x32, 0x03, 0x2e, 0x4f, 0x73, 0x52, 0x02, 0x6f, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x6f, 0x73, 0x5f,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x46, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x6f,
    0x73, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x15, 0x0a, 0x06, 0x6f, 0x73, 0x5f, 0x65,
    0x78, 0x74, 0x18, 0x50, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x6f, 0x73, 0x45, 0x78, 0x74, 0x12,
    0x3a, 0x0a, 0x19, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x5a, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x17, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x12, 0x1b, 0x0a, 0x09, 0x64,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x64, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x49, 0x64, 0x22, 0xa5, 0x01, 0x0a, 0x10, 0x4c, 0x69, 0x62,
    0x73, 0x70, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x41, 0x70, 0x70, 0x4b, 0x65, 0x79, 0x12, 0x18, 0x0a,
    0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x52, 0x07,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x65, 0x76, 0x6b, 0x65,
    0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x06, 0x64, 0x65, 0x76, 0x6b, 0x65, 0x79, 0x12,
    0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x03, 0x20, 0x02,
    0x28, 0x0c, 0x52, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12, 0x1c, 0x0a,
    0x09, 0x75, 0x73, 0x65, 0x72, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x02, 0x28, 0x09,
    0x52, 0x09, 0x75, 0x73, 0x65, 0x72, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x63,
    0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x05, 0x20, 0x02,
    0x28, 0x0c, 0x52, 0x0c, 0x63, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x48, 0x61, 0x73, 0x68,
    0x22, 0x67, 0x0a, 0x0a, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x18,
    0x0a, 0x07, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x07, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x12, 0x23, 0x0a, 0x02, 0x66, 0x62, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66,
    0x6f, 0x46, 0x61, 0x63, 0x65, 0x62, 0x6f, 0x6f, 0x6b, 0x52, 0x02, 0x66, 0x62, 0x12, 0x1a, 0x0a,
    0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x22, 0x33, 0x0a, 0x12, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x46, 0x61, 0x63, 0x65, 0x62, 0x6f, 0x6f, 0x6b, 0x12,
    0x1d, 0x0a, 0x0a, 0x6d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x09, 0x6d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65, 0x49, 0x64, 0x22, 0xd4,
    0x03, 0x0a, 0x09, 0x41, 0x50, 0x57, 0x65, 0x6c, 0x63, 0x6f, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x12,
    0x63, 0x61, 0x6e, 0x6f, 0x6e, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x09, 0x52, 0x11, 0x63, 0x61, 0x6e, 0x6f, 0x6e, 0x69,
    0x63, 0x61, 0x6c, 0x55, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x41, 0x0a, 0x16, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x67,
    0x65, 0x64, 0x5f, 0x69, 0x6e, 0x18, 0x14, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x0c, 0x2e, 0x41, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x52, 0x13, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x4c, 0x6f, 0x67, 0x67, 0x65, 0x64, 0x49, 0x6e, 0x12, 0x49,
    0x0a, 0x1a, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x67, 0x65, 0x64, 0x5f, 0x69, 0x6e, 0x18, 0x19, 0x20, 0x02,
    0x28, 0x0e, 0x32, 0x0c, 0x2e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65,
    0x52, 0x17, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x54, 0x79, 0x70,
    0x65, 0x4c, 0x6f, 0x67, 0x67, 0x65, 0x64, 0x49, 0x6e, 0x12, 0x58, 0x0a, 0x1e, 0x72, 0x65, 0x75,
    0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x63, 0x72, 0x65, 0x64, 0x65,
    0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x1e, 0x20, 0x02, 0x28,
    0x0e, 0x32, 0x13, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x1b, 0x72, 0x65, 0x75, 0x73, 0x61, 0x62, 0x6c, 0x65,
    0x41, 0x75, 0x74, 0x68, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x3a, 0x0a, 0x19, 0x72, 0x65, 0x75, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x5f,
    0x61, 0x75, 0x74, 0x68, 0x5f, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73,
    0x18, 0x28, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x17, 0x72, 0x65, 0x75, 0x73, 0x61, 0x62, 0x6c, 0x65,
    0x41, 0x75, 0x74, 0x68, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x12,
    0x1d, 0x0a, 0x0a, 0x6c, 0x66, 0x73, 0x5f, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x18, 0x32, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x09, 0x6c, 0x66, 0x73, 0x53, 0x65, 0x63, 0x72, 0x65, 0x74, 0x12, 0x2f,
    0x0a, 0x0c, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x3c,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x6e,
    0x66, 0x6f, 0x52, 0x0b, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x12,
    0x24, 0x0a, 0x02, 0x66, 0x62, 0x18, 0x46, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x41, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x46, 0x61, 0x63, 0x65, 0x62, 0x6f, 0x6f,
    0x6b, 0x52, 0x02, 0x66, 0x62, 0x22, 0x6e, 0x0a, 0x0b, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x2d, 0x0a, 0x07, 0x73, 0x70, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49,
    0x6e, 0x66, 0x6f, 0x53, 0x70, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x52, 0x07, 0x73, 0x70, 0x6f, 0x74,
    0x69, 0x66, 0x79, 0x12, 0x30, 0x0a, 0x08, 0x66, 0x61, 0x63, 0x65, 0x62, 0x6f, 0x6f, 0x6b, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49,
    0x6e, 0x66, 0x6f, 0x46, 0x61, 0x63, 0x65, 0x62, 0x6f, 0x6f, 0x6b, 0x52, 0x08, 0x66, 0x61, 0x63,
    0x65, 0x62, 0x6f, 0x6f, 0x6b, 0x22, 0x14, 0x0a, 0x12, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x49, 0x6e, 0x66, 0x6f, 0x53, 0x70, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x22, 0x57, 0x0a, 0x13, 0x41,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x46, 0x61, 0x63, 0x65, 0x62, 0x6f,
    0x6f, 0x6b, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x5f, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73,
    0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6d, 0x61, 0x63, 0x68, 0x69,
    0x6e, 0x65, 0x49, 0x64, 0x2a, 0xd6, 0x01, 0x0a, 0x12, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74,
    0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1c, 0x0a, 0x18, 0x41,
    0x55, 0x54, 0x48, 0x45, 0x4e, 0x54, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x53,
    0x45, 0x52, 0x5f, 0x50, 0x41, 0x53, 0x53, 0x10, 0x00, 0x12, 0x2d, 0x0a, 0x29, 0x41, 0x55, 0x54,
    0x48, 0x45, 0x4e, 0x54, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x53, 0x54, 0x4f, 0x52,
    0x45, 0x44, 0x5f, 0x53, 0x50, 0x4f, 0x54, 0x49, 0x46, 0x59, 0x5f, 0x43, 0x52, 0x45, 0x44, 0x45,
    0x4e, 0x54, 0x49, 0x41, 0x4c, 0x53, 0x10, 0x01, 0x12, 0x2e, 0x0a, 0x2a, 0x41, 0x55, 0x54, 0x48,
    0x45, 0x4e, 0x54, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x53, 0x54, 0x4f, 0x52, 0x45,
    0x44, 0x5f, 0x46, 0x41, 0x43, 0x45, 0x42, 0x4f, 0x4f, 0x4b, 0x5f, 0x43, 0x52, 0x45, 0x44, 0x45,
    0x4e, 0x54, 0x49, 0x41, 0x4c, 0x53, 0x10, 0x02, 0x12, 0x20, 0x0a, 0x1c, 0x41, 0x55, 0x54, 0x48,
    0x45, 0x4e, 0x54, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x53, 0x50, 0x4f, 0x54, 0x49,
    0x46, 0x59, 0x5f, 0x54, 0x4f, 0x4b, 0x45, 0x4e, 0x10, 0x03, 0x12, 0x21, 0x0a, 0x1d, 0x41, 0x55,
    0x54, 0x48, 0x45, 0x4e, 0x54, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x46, 0x41, 0x43,
    0x45, 0x42, 0x4f, 0x4f, 0x4b, 0x5f, 0x54, 0x4f, 0x4b, 0x45, 0x4e, 0x10, 0x04, 0x2a, 0x59, 0x0a,
    0x0f, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x43, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x12, 0x22, 0x0a, 0x1e, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x5f, 0x43, 0x52, 0x45, 0x41,
    0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x41, 0x4c, 0x57, 0x41, 0x59, 0x53, 0x5f, 0x50, 0x52, 0x4f, 0x4d,
    0x50, 0x54, 0x10, 0x01, 0x12, 0x22, 0x0a, 0x1e, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x5f,
    0x43, 0x52, 0x45, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x41, 0x4c, 0x57, 0x41, 0x59, 0x53, 0x5f,
    0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x10, 0x03, 0x2a, 0x9d, 0x01, 0x0a, 0x09, 0x43, 0x70, 0x75,
    0x46, 0x61, 0x6d, 0x69, 0x6c, 0x79, 0x12, 0x0f, 0x0a, 0x0b, 0x43, 0x50, 0x55, 0x5f, 0x55, 0x4e,
    0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x50, 0x55, 0x5f, 0x58,
    0x38, 0x36, 0x10, 0x01, 0x12, 0x0e, 0x0a, 0x0a, 0x43, 0x50, 0x55, 0x5f, 0x58, 0x38, 0x36, 0x5f,
    0x36, 0x34, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x50, 0x55, 0x5f, 0x50, 0x50, 0x43, 0x10,
    0x03, 0x12, 0x0e, 0x0a, 0x0a, 0x43, 0x50, 0x55, 0x5f, 0x50, 0x50, 0x43, 0x5f, 0x36, 0x34, 0x10,
    0x04, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x50, 0x55, 0x5f, 0x41, 0x52, 0x4d, 0x10, 0x05, 0x12, 0x0c,
    0x0a, 0x08, 0x43, 0x50, 0x55, 0x5f, 0x49, 0x41, 0x36, 0x34, 0x10, 0x06, 0x12, 0x0a, 0x0a, 0x06,
    0x43, 0x50, 0x55, 0x5f, 0x53, 0x48, 0x10, 0x07, 0x12, 0x0c, 0x0a, 0x08, 0x43, 0x50, 0x55, 0x5f,
    0x4d, 0x49, 0x50, 0x53, 0x10, 0x08, 0x12, 0x10, 0x0a, 0x0c, 0x43, 0x50, 0x55, 0x5f, 0x42, 0x4c,
    0x41, 0x43, 0x4b, 0x46, 0x49, 0x4e, 0x10, 0x09, 0x2a, 0x4b, 0x0a, 0x05, 0x42, 0x72, 0x61, 0x6e,
    0x64, 0x12, 0x13, 0x0a, 0x0f, 0x42, 0x52, 0x41, 0x4e, 0x44, 0x5f, 0x55, 0x4e, 0x42, 0x52, 0x41,
    0x4e, 0x44, 0x45, 0x44, 0x10, 0x00, 0x12, 0x0d, 0x0a, 0x09, 0x42, 0x52, 0x41, 0x4e, 0x44, 0x5f,
    0x49, 0x4e, 0x51, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x42, 0x52, 0x41, 0x4e, 0x44, 0x5f, 0x48,
    0x54, 0x43, 0x10, 0x02, 0x12, 0x0f, 0x0a, 0x0b, 0x42, 0x52, 0x41, 0x4e, 0x44, 0x5f, 0x4e, 0x4f,
    0x4b, 0x49, 0x41, 0x10, 0x03, 0x2a, 0xd1, 0x02, 0x0a, 0x02, 0x4f, 0x73, 0x12, 0x0e, 0x0a, 0x0a,
    0x4f, 0x53, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a,
    0x4f, 0x53, 0x5f, 0x57, 0x49, 0x4e, 0x44, 0x4f, 0x57, 0x53, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06,
    0x4f, 0x53, 0x5f, 0x4f, 0x53, 0x58, 0x10, 0x02, 0x12, 0x0d, 0x0a, 0x09, 0x4f, 0x53, 0x5f, 0x49,
    0x50, 0x48, 0x4f, 0x4e, 0x45, 0x10, 0x03, 0x12, 0x0a, 0x0a, 0x06, 0x4f, 0x53, 0x5f, 0x53, 0x36,
    0x30, 0x10, 0x04, 0x12, 0x0c, 0x0a, 0x08, 0x4f, 0x53, 0x5f, 0x4c, 0x49, 0x4e, 0x55, 0x58, 0x10,
    0x05, 0x12, 0x11, 0x0a, 0x0d, 0x4f, 0x53, 0x5f, 0x57, 0x49, 0x4e, 0x44, 0x4f, 0x57, 0x53, 0x5f,
    0x43, 0x45, 0x10, 0x06, 0x12, 0x0e, 0x0a, 0x0a, 0x4f, 0x53, 0x5f, 0x41, 0x4e, 0x44, 0x52, 0x4f,
    0x49, 0x44, 0x10, 0x07, 0x12, 0x0b, 0x0a, 0x07, 0x4f, 0x53, 0x5f, 0x50, 0x41, 0x4c, 0x4d, 0x10,
    0x08, 0x12, 0x0e, 0x0a, 0x0a, 0x4f, 0x53, 0x5f, 0x46, 0x52, 0x45, 0x45, 0x42, 0x53, 0x44, 0x10,
    0x09, 0x12, 0x11, 0x0a, 0x0d, 0x4f, 0x53, 0x5f, 0x42, 0x4c, 0x41, 0x43, 0x4b, 0x42, 0x45, 0x52,
    0x52, 0x59, 0x10, 0x0a, 0x12, 0x0c, 0x0a, 0x08, 0x4f, 0x53, 0x5f, 0x53, 0x4f, 0x4e, 0x4f, 0x53,
    0x10, 0x0b, 0x12, 0x0f, 0x0a, 0x0b, 0x4f, 0x53, 0x5f, 0x4c, 0x4f, 0x47, 0x49, 0x54, 0x45, 0x43,
    0x48, 0x10, 0x0c, 0x12, 0x0a, 0x0a, 0x06, 0x4f, 0x53, 0x5f, 0x57, 0x50, 0x37, 0x10, 0x0d, 0x12,
    0x0c, 0x0a, 0x08, 0x4f, 0x53, 0x5f, 0x4f, 0x4e, 0x4b, 0x59, 0x4f, 0x10, 0x0e, 0x12, 0x0e, 0x0a,
    0x0a, 0x4f, 0x53, 0x5f, 0x50, 0x48, 0x49, 0x4c, 0x49, 0x50, 0x53, 0x10, 0x0f, 0x12, 0x09, 0x0a,
    0x05, 0x4f, 0x53, 0x5f, 0x57, 0x44, 0x10, 0x10, 0x12, 0x0c, 0x0a, 0x08, 0x4f, 0x53, 0x5f, 0x56,
    0x4f, 0x4c, 0x56, 0x4f, 0x10, 0x11, 0x12, 0x0b, 0x0a, 0x07, 0x4f, 0x53, 0x5f, 0x54, 0x49, 0x56,
    0x4f, 0x10, 0x12, 0x12, 0x0b, 0x0a, 0x07, 0x4f, 0x53, 0x5f, 0x41, 0x57, 0x4f, 0x58, 0x10, 0x13,
    0x12, 0x0c, 0x0a, 0x08, 0x4f, 0x53, 0x5f, 0x4d, 0x45, 0x45, 0x47, 0x4f, 0x10, 0x14, 0x12, 0x0d,
    0x0a, 0x09, 0x4f, 0x53, 0x5f, 0x51, 0x4e, 0x58, 0x4e, 0x54, 0x4f, 0x10, 0x15, 0x12, 0x0a, 0x0a,
    0x06, 0x4f, 0x53, 0x5f, 0x42, 0x43, 0x4f, 0x10, 0x16, 0x2a, 0x28, 0x0a, 0x0b, 0x41, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x70, 0x6f, 0x74,
    0x69, 0x66, 0x79, 0x10, 0x00, 0x12, 0x0c, 0x0a, 0x08, 0x46, 0x61, 0x63, 0x65, 0x62, 0x6f, 0x6f,
    0x6b, 0x10, 0x01, 0x4a, 0xee, 0x2f, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xa4, 0x01, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x02, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x36, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x03, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x1e, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x03, 0x32, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x04, 0x04, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x04, 0x0d, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x1d, 0x2d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x30, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x04, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x05, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x05, 0x0d, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05,
    0x26, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x3d, 0x41,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x06, 0x04, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x06, 0x0d, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x06, 0x1d, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x06, 0x2b, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07,
    0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x07, 0x0d, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x07, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x07, 0x26, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x05, 0x12, 0x03, 0x08, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x08, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x08, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x08, 0x14,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x08, 0x25, 0x29, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x09, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x05, 0x12, 0x03, 0x09, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x09, 0x14, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12,
    0x03, 0x09, 0x25, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0a, 0x04,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x0a, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x0a, 0x1e, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x0a, 0x27, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x08, 0x12, 0x03, 0x0b, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12,
    0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x0b,
    0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0b, 0x18, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x0b, 0x26, 0x2a, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0e, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x0e, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x0f, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x14, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x10, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x10, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10,
    0x20, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x26, 0x2a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x11, 0x04, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x11, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x11, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x11, 0x1f, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x14, 0x00, 0x1a,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x14, 0x05, 0x17, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x15, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x16, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x04,
    0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x16, 0x30, 0x33, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x17, 0x04, 0x35, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x17, 0x31, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x18, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x18, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x18, 0x23,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x19, 0x04, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x19, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x19, 0x24, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01,
    0x12, 0x04, 0x1c, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x1c,
    0x05, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x04, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x04, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1d, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x1e, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x1e, 0x25, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x21, 0x00, 0x24, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x21, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x22, 0x0d, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x22, 0x26, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x2e,
    0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x04, 0x3e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x23, 0x0d, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x2b, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x23, 0x39, 0x3d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x26, 0x00,
    0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x26, 0x08, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x27, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x27, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x27, 0x13, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x27, 0x23, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2a, 0x00, 0x2c, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x2b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b,
    0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x1a, 0x1d,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x2e, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x2f, 0x04, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2f, 0x0d,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x21, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x2e, 0x31, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x30, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x30, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x30, 0x1b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30,
    0x28, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x33, 0x00, 0x35, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x33, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x34, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x34, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x34, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34, 0x13,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x34, 0x20, 0x23, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x37, 0x00, 0x3a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x07, 0x01, 0x12, 0x03, 0x37, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12,
    0x03, 0x38, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x38,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x38, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x13, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x38, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x39, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x39, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x39, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x39, 0x13, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x39, 0x2b,
    0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x3c, 0x00, 0x47, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x00, 0x12, 0x03, 0x3d, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3d,
    0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x17, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x24, 0x27, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x3e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x3e, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x3e, 0x22, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x3f, 0x04, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3f, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x3f, 0x1e, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03,
    0x12, 0x03, 0x40, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x06, 0x12, 0x03, 0x40, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x40, 0x13, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x40, 0x1b, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03, 0x41, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x41, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x41, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x41,
    0x22, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x05, 0x12, 0x03, 0x42, 0x04, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x04, 0x12, 0x03, 0x42, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x05, 0x06, 0x12, 0x03, 0x42, 0x0d, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x05, 0x01, 0x12, 0x03, 0x42, 0x10, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x42, 0x15, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x06, 0x12,
    0x03, 0x43, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x04, 0x12, 0x03, 0x43,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x05, 0x12, 0x03, 0x43, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x01, 0x12, 0x03, 0x43, 0x14, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x03, 0x12, 0x03, 0x43, 0x21, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x07, 0x12, 0x03, 0x44, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x07, 0x04, 0x12, 0x03, 0x44, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x05,
    0x12, 0x03, 0x44, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x44, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x03, 0x12, 0x03, 0x44, 0x1d,
    0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x08, 0x12, 0x03, 0x45, 0x04, 0x35, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x08, 0x04, 0x12, 0x03, 0x45, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x08, 0x05, 0x12, 0x03, 0x45, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x45, 0x14, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x08,
    0x03, 0x12, 0x03, 0x45, 0x30, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x09, 0x12, 0x03,
    0x46, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x09, 0x04, 0x12, 0x03, 0x46, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x09, 0x05, 0x12, 0x03, 0x46, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x09, 0x01, 0x12, 0x03, 0x46, 0x14, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x09, 0x03, 0x12, 0x03, 0x46, 0x20, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x05,
    0x02, 0x12, 0x04, 0x49, 0x00, 0x54, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03,
    0x49, 0x05, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x04, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4a, 0x04, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x4a, 0x12, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4b, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x4b, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x4b, 0x0e, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x4c,
    0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x04, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x03, 0x4c, 0x11, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4d, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4d, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x4d, 0x0e, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x4e, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4e,
    0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04, 0x02, 0x12, 0x03, 0x4e, 0x11, 0x14,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4f, 0x04, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4f, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x05, 0x02, 0x12, 0x03, 0x4f, 0x0e, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x50, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x06, 0x02, 0x12, 0x03, 0x50,
    0x0f, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x07, 0x12, 0x03, 0x51, 0x04, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x51, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x07, 0x02, 0x12, 0x03, 0x51, 0x0d, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x08, 0x12, 0x03, 0x52, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x08,
    0x01, 0x12, 0x03, 0x52, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x08, 0x02, 0x12,
    0x03, 0x52, 0x0f, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x09, 0x12, 0x03, 0x53, 0x04,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x09, 0x01, 0x12, 0x03, 0x53, 0x04, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x09, 0x02, 0x12, 0x03, 0x53, 0x13, 0x16, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x03, 0x12, 0x04, 0x56, 0x00, 0x5b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x03, 0x01,
    0x12, 0x03, 0x56, 0x05, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x00, 0x12, 0x03, 0x57,
    0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x04, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x02, 0x12, 0x03, 0x57, 0x16, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x03, 0x02, 0x01, 0x12, 0x03, 0x58, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x58, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x58, 0x10, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x02, 0x12,
    0x03, 0x59, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x59,
    0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x02, 0x02, 0x12, 0x03, 0x59, 0x10, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x03, 0x12, 0x03, 0x5a, 0x04, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x5a, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x03, 0x02, 0x03, 0x02, 0x12, 0x03, 0x5a, 0x12, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x04, 0x12,
    0x04, 0x5d, 0x00, 0x75, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x04, 0x01, 0x12, 0x03, 0x5d, 0x05,
    0x07, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x00, 0x12, 0x03, 0x5e, 0x04, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5e, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x04, 0x02, 0x00, 0x02, 0x12, 0x03, 0x5e, 0x11, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x5f, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x5f, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x5f, 0x11, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x02, 0x12, 0x03, 0x60, 0x04, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x60, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x60, 0x0d, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x04, 0x02, 0x03, 0x12, 0x03, 0x61, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x61, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x61, 0x10, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x04, 0x12, 0x03, 0x62,
    0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x62, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x04, 0x02, 0x12, 0x03, 0x62, 0x0d, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x04, 0x02, 0x05, 0x12, 0x03, 0x63, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x63, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02,
    0x05, 0x02, 0x12, 0x03, 0x63, 0x0f, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x06, 0x12,
    0x03, 0x64, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x64,
    0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x06, 0x02, 0x12, 0x03, 0x64, 0x14, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x07, 0x12, 0x03, 0x65, 0x04, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x04, 0x02, 0x07, 0x01, 0x12, 0x03, 0x65, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x04, 0x02, 0x07, 0x02, 0x12, 0x03, 0x65, 0x11, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02,
    0x08, 0x12, 0x03, 0x66, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x66, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x08, 0x02, 0x12, 0x03, 0x66,
    0x0e, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x09, 0x12, 0x03, 0x67, 0x04, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x09, 0x01, 0x12, 0x03, 0x67, 0x04, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x04, 0x02, 0x09, 0x02, 0x12, 0x03, 0x67, 0x11, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x04, 0x02, 0x0a, 0x12, 0x03, 0x68, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x68, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x0a, 0x02, 0x12,
    0x03, 0x68, 0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x0b, 0x12, 0x03, 0x69, 0x04,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x69, 0x0f, 0x12, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x04, 0x02, 0x0c, 0x12, 0x03, 0x6a, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04,
    0x02, 0x0c, 0x01, 0x12, 0x03, 0x6a, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x0c,
    0x02, 0x12, 0x03, 0x6a, 0x12, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x0d, 0x12, 0x03,
    0x6b, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x6b, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x6b, 0x0d, 0x10, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x0e, 0x12, 0x03, 0x6c, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x04, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x6c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04,
    0x02, 0x0e, 0x02, 0x12, 0x03, 0x6c, 0x0f, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x0f,
    0x12, 0x03, 0x6d, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x0f, 0x01, 0x12, 0x03,
    0x6d, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x6d, 0x11,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x10, 0x12, 0x03, 0x6e, 0x04, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x04, 0x02, 0x10, 0x01, 0x12, 0x03, 0x6e, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x04, 0x02, 0x10, 0x02, 0x12, 0x03, 0x6e, 0x0c, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04,
    0x02, 0x11, 0x12, 0x03, 0x6f, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x11, 0x01,
    0x12, 0x03, 0x6f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x11, 0x02, 0x12, 0x03,
    0x6f, 0x0f, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x12, 0x12, 0x03, 0x70, 0x04, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x12, 0x01, 0x12, 0x03, 0x70, 0x04, 0x0b, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x04, 0x02, 0x12, 0x02, 0x12, 0x03, 0x70, 0x0e, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x04, 0x02, 0x13, 0x12, 0x03, 0x71, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02,
    0x13, 0x01, 0x12, 0x03, 0x71, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x13, 0x02,
    0x12, 0x03, 0x71, 0x0e, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x14, 0x12, 0x03, 0x72,
    0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x14, 0x01, 0x12, 0x03, 0x72, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x14, 0x02, 0x12, 0x03, 0x72, 0x0f, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x04, 0x02, 0x15, 0x12, 0x03, 0x73, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x04, 0x02, 0x15, 0x01, 0x12, 0x03, 0x73, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02,
    0x15, 0x02, 0x12, 0x03, 0x73, 0x10, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x16, 0x12,
    0x03, 0x74, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x16, 0x01, 0x12, 0x03, 0x74,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x16, 0x02, 0x12, 0x03, 0x74, 0x0d, 0x11,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x77, 0x00, 0x7d, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x09, 0x01, 0x12, 0x03, 0x77, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x03, 0x78, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x78, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x78, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x78, 0x14, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x78, 0x1e, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x79, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x79, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x79, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x79, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x79,
    0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x7a, 0x04, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03, 0x7a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7a, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x7a, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12,
    0x03, 0x7b, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7b,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x05, 0x12, 0x03, 0x7b, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7b, 0x14, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7b, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x04, 0x12, 0x03, 0x7c, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x7c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x7c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x7c, 0x13, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x03, 0x12, 0x03, 0x7c, 0x23,
    0x26, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x05, 0x7f, 0x00, 0x83, 0x01, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x7f, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x00, 0x12, 0x04, 0x80, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x80, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x80, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x80, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x80, 0x01, 0x1c, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x04, 0x81, 0x01,
    0x04, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0x81, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x04, 0x81, 0x01, 0x0d, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x04, 0x81, 0x01, 0x20, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x81, 0x01, 0x25, 0x28, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x04, 0x82, 0x01, 0x04, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x02, 0x04, 0x12, 0x04, 0x82, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x02, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x82, 0x01, 0x14, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x82, 0x01, 0x1f, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06,
    0x85, 0x01, 0x00, 0x87, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0x85,
    0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01, 0x04,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0d, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x14, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x21, 0x24, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x0c, 0x12, 0x06, 0x89, 0x01, 0x00, 0x92, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x0c, 0x01, 0x12, 0x04, 0x89, 0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00,
    0x12, 0x04, 0x8a, 0x01, 0x04, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x8a, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x04,
    0x8a, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8a,
    0x01, 0x14, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8a, 0x01,
    0x29, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x37,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x06, 0x12, 0x04, 0x8b, 0x01, 0x0d, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x19, 0x2f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x32, 0x36, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x02, 0x06, 0x12, 0x04, 0x8c, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x8c, 0x01, 0x19, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x8c, 0x01, 0x36, 0x3a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x04,
    0x8d, 0x01, 0x04, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8d,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x06, 0x12, 0x04, 0x8d, 0x01,
    0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x20,
    0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x41, 0x45,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x34, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x04, 0x05, 0x12, 0x04, 0x8e, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x13, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x04, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x2f, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x05,
    0x12, 0x04, 0x8f, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x01, 0x12,
    0x04, 0x8f, 0x01, 0x13, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x03, 0x12, 0x04,
    0x8f, 0x01, 0x20, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x06, 0x12, 0x04, 0x90, 0x01,
    0x04, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x04, 0x12, 0x04, 0x90, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x06, 0x12, 0x04, 0x90, 0x01, 0x0d, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x01, 0x12, 0x04, 0x90, 0x01, 0x19, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x03, 0x12, 0x04, 0x90, 0x01, 0x28, 0x2c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x07, 0x12, 0x04, 0x91, 0x01, 0x04, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x07, 0x04, 0x12, 0x04, 0x91, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x07, 0x06, 0x12, 0x04, 0x91, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x07, 0x01, 0x12, 0x04, 0x91, 0x01, 0x21, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x07, 0x03, 0x12, 0x04, 0x91, 0x01, 0x26, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x05, 0x12, 0x06,
    0x94, 0x01, 0x00, 0x97, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x05, 0x01, 0x12, 0x04, 0x94,
    0x01, 0x05, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x00, 0x12, 0x04, 0x95, 0x01, 0x04,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x01, 0x04, 0x0b,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x00, 0x02, 0x12, 0x04, 0x95, 0x01, 0x0e, 0x11, 0x0a,
    0x0c, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x04, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x05, 0x02, 0x01, 0x01, 0x12, 0x04, 0x96, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x05, 0x02, 0x01, 0x02, 0x12, 0x04, 0x96, 0x01, 0x0f, 0x12, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x0d, 0x12, 0x06, 0x99, 0x01, 0x00, 0x9c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01,
    0x12, 0x04, 0x99, 0x01, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04,
    0x9a, 0x01, 0x04, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9a,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9a, 0x01,
    0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x20,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x2a, 0x2d,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x30, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9b, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x21, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x2c, 0x2f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e,
    0x12, 0x06, 0x9e, 0x01, 0x00, 0x9f, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12,
    0x04, 0x9e, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xa1, 0x01, 0x00,
    0xa4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x1b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0xa2, 0x01, 0x04, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa2, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x23, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xa3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xa3, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xa3, 0x01, 0x14, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xa3, 0x01, 0x21, 0x24,
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
