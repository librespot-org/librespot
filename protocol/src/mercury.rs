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
pub struct MercuryMultiGetRequest {
    // message fields
    request: ::protobuf::RepeatedField<MercuryRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MercuryMultiGetRequest {}

impl MercuryMultiGetRequest {
    pub fn new() -> MercuryMultiGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MercuryMultiGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<MercuryMultiGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MercuryMultiGetRequest,
        };
        unsafe {
            instance.get(MercuryMultiGetRequest::new)
        }
    }

    // repeated .MercuryRequest request = 1;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: ::protobuf::RepeatedField<MercuryRequest>) {
        self.request = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request(&mut self) -> &mut ::protobuf::RepeatedField<MercuryRequest> {
        &mut self.request
    }

    // Take field
    pub fn take_request(&mut self) -> ::protobuf::RepeatedField<MercuryRequest> {
        ::std::mem::replace(&mut self.request, ::protobuf::RepeatedField::new())
    }

    pub fn get_request(&self) -> &[MercuryRequest] {
        &self.request
    }

    fn get_request_for_reflect(&self) -> &::protobuf::RepeatedField<MercuryRequest> {
        &self.request
    }

    fn mut_request_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MercuryRequest> {
        &mut self.request
    }
}

impl ::protobuf::Message for MercuryMultiGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.request)?;
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
        for value in &self.request {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.request {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for MercuryMultiGetRequest {
    fn new() -> MercuryMultiGetRequest {
        MercuryMultiGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MercuryMultiGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MercuryRequest>>(
                    "request",
                    MercuryMultiGetRequest::get_request_for_reflect,
                    MercuryMultiGetRequest::mut_request_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MercuryMultiGetRequest>(
                    "MercuryMultiGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MercuryMultiGetRequest {
    fn clear(&mut self) {
        self.clear_request();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MercuryMultiGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MercuryMultiGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MercuryMultiGetReply {
    // message fields
    reply: ::protobuf::RepeatedField<MercuryReply>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MercuryMultiGetReply {}

impl MercuryMultiGetReply {
    pub fn new() -> MercuryMultiGetReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MercuryMultiGetReply {
        static mut instance: ::protobuf::lazy::Lazy<MercuryMultiGetReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MercuryMultiGetReply,
        };
        unsafe {
            instance.get(MercuryMultiGetReply::new)
        }
    }

    // repeated .MercuryReply reply = 1;

    pub fn clear_reply(&mut self) {
        self.reply.clear();
    }

    // Param is passed by value, moved
    pub fn set_reply(&mut self, v: ::protobuf::RepeatedField<MercuryReply>) {
        self.reply = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reply(&mut self) -> &mut ::protobuf::RepeatedField<MercuryReply> {
        &mut self.reply
    }

    // Take field
    pub fn take_reply(&mut self) -> ::protobuf::RepeatedField<MercuryReply> {
        ::std::mem::replace(&mut self.reply, ::protobuf::RepeatedField::new())
    }

    pub fn get_reply(&self) -> &[MercuryReply] {
        &self.reply
    }

    fn get_reply_for_reflect(&self) -> &::protobuf::RepeatedField<MercuryReply> {
        &self.reply
    }

    fn mut_reply_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MercuryReply> {
        &mut self.reply
    }
}

impl ::protobuf::Message for MercuryMultiGetReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.reply)?;
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
        for value in &self.reply {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.reply {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for MercuryMultiGetReply {
    fn new() -> MercuryMultiGetReply {
        MercuryMultiGetReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<MercuryMultiGetReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MercuryReply>>(
                    "reply",
                    MercuryMultiGetReply::get_reply_for_reflect,
                    MercuryMultiGetReply::mut_reply_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MercuryMultiGetReply>(
                    "MercuryMultiGetReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MercuryMultiGetReply {
    fn clear(&mut self) {
        self.clear_reply();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MercuryMultiGetReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MercuryMultiGetReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MercuryRequest {
    // message fields
    uri: ::protobuf::SingularField<::std::string::String>,
    content_type: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    etag: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MercuryRequest {}

impl MercuryRequest {
    pub fn new() -> MercuryRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MercuryRequest {
        static mut instance: ::protobuf::lazy::Lazy<MercuryRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MercuryRequest,
        };
        unsafe {
            instance.get(MercuryRequest::new)
        }
    }

    // optional string uri = 1;

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

    // optional string content_type = 2;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    pub fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::string::String) {
        self.content_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::string::String {
        if self.content_type.is_none() {
            self.content_type.set_default();
        };
        self.content_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::string::String {
        self.content_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_content_type(&self) -> &str {
        match self.content_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_content_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.content_type
    }

    fn mut_content_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.content_type
    }

    // optional bytes body = 3;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body.set_default();
        };
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.body
    }

    // optional bytes etag = 4;

    pub fn clear_etag(&mut self) {
        self.etag.clear();
    }

    pub fn has_etag(&self) -> bool {
        self.etag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_etag(&mut self, v: ::std::vec::Vec<u8>) {
        self.etag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_etag(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.etag.is_none() {
            self.etag.set_default();
        };
        self.etag.as_mut().unwrap()
    }

    // Take field
    pub fn take_etag(&mut self) -> ::std::vec::Vec<u8> {
        self.etag.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_etag(&self) -> &[u8] {
        match self.etag.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_etag_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.etag
    }

    fn mut_etag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.etag
    }
}

impl ::protobuf::Message for MercuryRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uri)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.content_type)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.etag)?;
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
        if let Some(v) = self.uri.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.content_type.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.etag.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uri.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.content_type.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.body.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.etag.as_ref() {
            os.write_bytes(4, &v)?;
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

impl ::protobuf::MessageStatic for MercuryRequest {
    fn new() -> MercuryRequest {
        MercuryRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MercuryRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uri",
                    MercuryRequest::get_uri_for_reflect,
                    MercuryRequest::mut_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content_type",
                    MercuryRequest::get_content_type_for_reflect,
                    MercuryRequest::mut_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    MercuryRequest::get_body_for_reflect,
                    MercuryRequest::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "etag",
                    MercuryRequest::get_etag_for_reflect,
                    MercuryRequest::mut_etag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MercuryRequest>(
                    "MercuryRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MercuryRequest {
    fn clear(&mut self) {
        self.clear_uri();
        self.clear_content_type();
        self.clear_body();
        self.clear_etag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MercuryRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MercuryRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MercuryReply {
    // message fields
    status_code: ::std::option::Option<i32>,
    status_message: ::protobuf::SingularField<::std::string::String>,
    cache_policy: ::std::option::Option<MercuryReply_CachePolicy>,
    ttl: ::std::option::Option<i32>,
    etag: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content_type: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MercuryReply {}

impl MercuryReply {
    pub fn new() -> MercuryReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MercuryReply {
        static mut instance: ::protobuf::lazy::Lazy<MercuryReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MercuryReply,
        };
        unsafe {
            instance.get(MercuryReply::new)
        }
    }

    // optional sint32 status_code = 1;

    pub fn clear_status_code(&mut self) {
        self.status_code = ::std::option::Option::None;
    }

    pub fn has_status_code(&self) -> bool {
        self.status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: i32) {
        self.status_code = ::std::option::Option::Some(v);
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code.unwrap_or(0)
    }

    fn get_status_code_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.status_code
    }

    fn mut_status_code_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.status_code
    }

    // optional string status_message = 2;

    pub fn clear_status_message(&mut self) {
        self.status_message.clear();
    }

    pub fn has_status_message(&self) -> bool {
        self.status_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_message(&mut self, v: ::std::string::String) {
        self.status_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status_message(&mut self) -> &mut ::std::string::String {
        if self.status_message.is_none() {
            self.status_message.set_default();
        };
        self.status_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_status_message(&mut self) -> ::std::string::String {
        self.status_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_status_message(&self) -> &str {
        match self.status_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_status_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.status_message
    }

    fn mut_status_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.status_message
    }

    // optional .MercuryReply.CachePolicy cache_policy = 3;

    pub fn clear_cache_policy(&mut self) {
        self.cache_policy = ::std::option::Option::None;
    }

    pub fn has_cache_policy(&self) -> bool {
        self.cache_policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cache_policy(&mut self, v: MercuryReply_CachePolicy) {
        self.cache_policy = ::std::option::Option::Some(v);
    }

    pub fn get_cache_policy(&self) -> MercuryReply_CachePolicy {
        self.cache_policy.unwrap_or(MercuryReply_CachePolicy::CACHE_NO)
    }

    fn get_cache_policy_for_reflect(&self) -> &::std::option::Option<MercuryReply_CachePolicy> {
        &self.cache_policy
    }

    fn mut_cache_policy_for_reflect(&mut self) -> &mut ::std::option::Option<MercuryReply_CachePolicy> {
        &mut self.cache_policy
    }

    // optional sint32 ttl = 4;

    pub fn clear_ttl(&mut self) {
        self.ttl = ::std::option::Option::None;
    }

    pub fn has_ttl(&self) -> bool {
        self.ttl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ttl(&mut self, v: i32) {
        self.ttl = ::std::option::Option::Some(v);
    }

    pub fn get_ttl(&self) -> i32 {
        self.ttl.unwrap_or(0)
    }

    fn get_ttl_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ttl
    }

    fn mut_ttl_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ttl
    }

    // optional bytes etag = 5;

    pub fn clear_etag(&mut self) {
        self.etag.clear();
    }

    pub fn has_etag(&self) -> bool {
        self.etag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_etag(&mut self, v: ::std::vec::Vec<u8>) {
        self.etag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_etag(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.etag.is_none() {
            self.etag.set_default();
        };
        self.etag.as_mut().unwrap()
    }

    // Take field
    pub fn take_etag(&mut self) -> ::std::vec::Vec<u8> {
        self.etag.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_etag(&self) -> &[u8] {
        match self.etag.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_etag_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.etag
    }

    fn mut_etag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.etag
    }

    // optional string content_type = 6;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    pub fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::string::String) {
        self.content_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::string::String {
        if self.content_type.is_none() {
            self.content_type.set_default();
        };
        self.content_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::string::String {
        self.content_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_content_type(&self) -> &str {
        match self.content_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_content_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.content_type
    }

    fn mut_content_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.content_type
    }

    // optional bytes body = 7;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body.set_default();
        };
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.body
    }
}

impl ::protobuf::Message for MercuryReply {
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
                    let tmp = is.read_sint32()?;
                    self.status_code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.status_message)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.cache_policy = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint32()?;
                    self.ttl = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.etag)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.content_type)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
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
        if let Some(v) = self.status_code {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        };
        if let Some(v) = self.status_message.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.cache_policy {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        if let Some(v) = self.ttl {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, v);
        };
        if let Some(v) = self.etag.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        if let Some(v) = self.content_type.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status_code {
            os.write_sint32(1, v)?;
        };
        if let Some(v) = self.status_message.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.cache_policy {
            os.write_enum(3, v.value())?;
        };
        if let Some(v) = self.ttl {
            os.write_sint32(4, v)?;
        };
        if let Some(v) = self.etag.as_ref() {
            os.write_bytes(5, &v)?;
        };
        if let Some(v) = self.content_type.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.body.as_ref() {
            os.write_bytes(7, &v)?;
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

impl ::protobuf::MessageStatic for MercuryReply {
    fn new() -> MercuryReply {
        MercuryReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<MercuryReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "status_code",
                    MercuryReply::get_status_code_for_reflect,
                    MercuryReply::mut_status_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "status_message",
                    MercuryReply::get_status_message_for_reflect,
                    MercuryReply::mut_status_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MercuryReply_CachePolicy>>(
                    "cache_policy",
                    MercuryReply::get_cache_policy_for_reflect,
                    MercuryReply::mut_cache_policy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "ttl",
                    MercuryReply::get_ttl_for_reflect,
                    MercuryReply::mut_ttl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "etag",
                    MercuryReply::get_etag_for_reflect,
                    MercuryReply::mut_etag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content_type",
                    MercuryReply::get_content_type_for_reflect,
                    MercuryReply::mut_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    MercuryReply::get_body_for_reflect,
                    MercuryReply::mut_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MercuryReply>(
                    "MercuryReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MercuryReply {
    fn clear(&mut self) {
        self.clear_status_code();
        self.clear_status_message();
        self.clear_cache_policy();
        self.clear_ttl();
        self.clear_etag();
        self.clear_content_type();
        self.clear_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MercuryReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MercuryReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MercuryReply_CachePolicy {
    CACHE_NO = 1,
    CACHE_PRIVATE = 2,
    CACHE_PUBLIC = 3,
}

impl ::protobuf::ProtobufEnum for MercuryReply_CachePolicy {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MercuryReply_CachePolicy> {
        match value {
            1 => ::std::option::Option::Some(MercuryReply_CachePolicy::CACHE_NO),
            2 => ::std::option::Option::Some(MercuryReply_CachePolicy::CACHE_PRIVATE),
            3 => ::std::option::Option::Some(MercuryReply_CachePolicy::CACHE_PUBLIC),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MercuryReply_CachePolicy] = &[
            MercuryReply_CachePolicy::CACHE_NO,
            MercuryReply_CachePolicy::CACHE_PRIVATE,
            MercuryReply_CachePolicy::CACHE_PUBLIC,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MercuryReply_CachePolicy>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MercuryReply_CachePolicy", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MercuryReply_CachePolicy {
}

impl ::protobuf::reflect::ProtobufValue for MercuryReply_CachePolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Header {
    // message fields
    uri: ::protobuf::SingularField<::std::string::String>,
    content_type: ::protobuf::SingularField<::std::string::String>,
    method: ::protobuf::SingularField<::std::string::String>,
    status_code: ::std::option::Option<i32>,
    user_fields: ::protobuf::RepeatedField<UserField>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Header {}

impl Header {
    pub fn new() -> Header {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Header {
        static mut instance: ::protobuf::lazy::Lazy<Header> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Header,
        };
        unsafe {
            instance.get(Header::new)
        }
    }

    // optional string uri = 1;

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

    // optional string content_type = 2;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    pub fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::string::String) {
        self.content_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::string::String {
        if self.content_type.is_none() {
            self.content_type.set_default();
        };
        self.content_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::string::String {
        self.content_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_content_type(&self) -> &str {
        match self.content_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_content_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.content_type
    }

    fn mut_content_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.content_type
    }

    // optional string method = 3;

    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    pub fn has_method(&self) -> bool {
        self.method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: ::std::string::String) {
        self.method = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_method(&mut self) -> &mut ::std::string::String {
        if self.method.is_none() {
            self.method.set_default();
        };
        self.method.as_mut().unwrap()
    }

    // Take field
    pub fn take_method(&mut self) -> ::std::string::String {
        self.method.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_method(&self) -> &str {
        match self.method.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_method_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.method
    }

    fn mut_method_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.method
    }

    // optional sint32 status_code = 4;

    pub fn clear_status_code(&mut self) {
        self.status_code = ::std::option::Option::None;
    }

    pub fn has_status_code(&self) -> bool {
        self.status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: i32) {
        self.status_code = ::std::option::Option::Some(v);
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code.unwrap_or(0)
    }

    fn get_status_code_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.status_code
    }

    fn mut_status_code_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.status_code
    }

    // repeated .UserField user_fields = 6;

    pub fn clear_user_fields(&mut self) {
        self.user_fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_fields(&mut self, v: ::protobuf::RepeatedField<UserField>) {
        self.user_fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_user_fields(&mut self) -> &mut ::protobuf::RepeatedField<UserField> {
        &mut self.user_fields
    }

    // Take field
    pub fn take_user_fields(&mut self) -> ::protobuf::RepeatedField<UserField> {
        ::std::mem::replace(&mut self.user_fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_user_fields(&self) -> &[UserField] {
        &self.user_fields
    }

    fn get_user_fields_for_reflect(&self) -> &::protobuf::RepeatedField<UserField> {
        &self.user_fields
    }

    fn mut_user_fields_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UserField> {
        &mut self.user_fields
    }
}

impl ::protobuf::Message for Header {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uri)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.content_type)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.method)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint32()?;
                    self.status_code = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.user_fields)?;
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
        if let Some(v) = self.uri.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.content_type.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.method.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.status_code {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, v);
        };
        for value in &self.user_fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uri.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.content_type.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.method.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.status_code {
            os.write_sint32(4, v)?;
        };
        for v in &self.user_fields {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Header {
    fn new() -> Header {
        Header::new()
    }

    fn descriptor_static(_: ::std::option::Option<Header>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uri",
                    Header::get_uri_for_reflect,
                    Header::mut_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content_type",
                    Header::get_content_type_for_reflect,
                    Header::mut_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "method",
                    Header::get_method_for_reflect,
                    Header::mut_method_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "status_code",
                    Header::get_status_code_for_reflect,
                    Header::mut_status_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UserField>>(
                    "user_fields",
                    Header::get_user_fields_for_reflect,
                    Header::mut_user_fields_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Header>(
                    "Header",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Header {
    fn clear(&mut self) {
        self.clear_uri();
        self.clear_content_type();
        self.clear_method();
        self.clear_status_code();
        self.clear_user_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Header {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserField {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserField {}

impl UserField {
    pub fn new() -> UserField {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserField {
        static mut instance: ::protobuf::lazy::Lazy<UserField> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserField,
        };
        unsafe {
            instance.get(UserField::new)
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.key
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for UserField {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for UserField {
    fn new() -> UserField {
        UserField::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserField>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    UserField::get_key_for_reflect,
                    UserField::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    UserField::get_value_for_reflect,
                    UserField::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserField>(
                    "UserField",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserField {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserField {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x6d, 0x65, 0x72, 0x63, 0x75, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x43, 0x0a, 0x16, 0x4d, 0x65, 0x72, 0x63, 0x75, 0x72, 0x79, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x47,
    0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x29, 0x0a, 0x07, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x4d, 0x65, 0x72,
    0x63, 0x75, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x07, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x22, 0x3b, 0x0a, 0x14, 0x4d, 0x65, 0x72, 0x63, 0x75, 0x72, 0x79, 0x4d,
    0x75, 0x6c, 0x74, 0x69, 0x47, 0x65, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x23, 0x0a, 0x05,
    0x72, 0x65, 0x70, 0x6c, 0x79, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x4d, 0x65,
    0x72, 0x63, 0x75, 0x72, 0x79, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x52, 0x05, 0x72, 0x65, 0x70, 0x6c,
    0x79, 0x22, 0x6d, 0x0a, 0x0e, 0x4d, 0x65, 0x72, 0x63, 0x75, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x69, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x03, 0x75, 0x72, 0x69, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x62, 0x6f, 0x64, 0x79,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x62, 0x6f, 0x64, 0x79, 0x12, 0x12, 0x0a, 0x04,
    0x65, 0x74, 0x61, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x65, 0x74, 0x61, 0x67,
    0x22, 0xb3, 0x02, 0x0a, 0x0c, 0x4d, 0x65, 0x72, 0x63, 0x75, 0x72, 0x79, 0x52, 0x65, 0x70, 0x6c,
    0x79, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x11, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f,
    0x64, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3c, 0x0a, 0x0c, 0x63, 0x61, 0x63,
    0x68, 0x65, 0x5f, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x19, 0x2e, 0x4d, 0x65, 0x72, 0x63, 0x75, 0x72, 0x79, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x2e, 0x43,
    0x61, 0x63, 0x68, 0x65, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x52, 0x0b, 0x63, 0x61, 0x63, 0x68,
    0x65, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x74, 0x74, 0x6c, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x11, 0x52, 0x03, 0x74, 0x74, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x65, 0x74, 0x61,
    0x67, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x65, 0x74, 0x61, 0x67, 0x12, 0x21, 0x0a,
    0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x12, 0x0a, 0x04, 0x62, 0x6f, 0x64, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04,
    0x62, 0x6f, 0x64, 0x79, 0x22, 0x40, 0x0a, 0x0b, 0x43, 0x61, 0x63, 0x68, 0x65, 0x50, 0x6f, 0x6c,
    0x69, 0x63, 0x79, 0x12, 0x0c, 0x0a, 0x08, 0x43, 0x41, 0x43, 0x48, 0x45, 0x5f, 0x4e, 0x4f, 0x10,
    0x01, 0x12, 0x11, 0x0a, 0x0d, 0x43, 0x41, 0x43, 0x48, 0x45, 0x5f, 0x50, 0x52, 0x49, 0x56, 0x41,
    0x54, 0x45, 0x10, 0x02, 0x12, 0x10, 0x0a, 0x0c, 0x43, 0x41, 0x43, 0x48, 0x45, 0x5f, 0x50, 0x55,
    0x42, 0x4c, 0x49, 0x43, 0x10, 0x03, 0x22, 0xa3, 0x01, 0x0a, 0x06, 0x48, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x69, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03,
    0x75, 0x72, 0x69, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x12, 0x1f,
    0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x11, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f, 0x64, 0x65, 0x12,
    0x2b, 0x0a, 0x0b, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x06,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x55, 0x73, 0x65, 0x72, 0x46, 0x69, 0x65, 0x6c, 0x64,
    0x52, 0x0a, 0x75, 0x73, 0x65, 0x72, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x22, 0x33, 0x0a, 0x09,
    0x55, 0x73, 0x65, 0x72, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x4a, 0xaf, 0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2c, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00,
    0x04, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x03, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x03, 0x1c, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x03, 0x26, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x06, 0x00, 0x08, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x06, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x07, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x07, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07,
    0x1a, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x22, 0x25,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0a, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x0b, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x14, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x0c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0c, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c,
    0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x04, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x0d, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x0e, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0e,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0e, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x13, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x1a, 0x1d, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x11, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x11, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x12, 0x04,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x12, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x01, 0x12, 0x03, 0x13, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x13, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x13,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x14, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x25, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x14, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x14, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x14, 0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x14, 0x28, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x15, 0x04, 0x19,
    0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x15, 0x09, 0x14, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x16, 0x08, 0x17, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x08, 0x10, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x16, 0x13, 0x16, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x17, 0x08, 0x1c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x08, 0x15, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x17, 0x18, 0x1b, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x18, 0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x18, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x1a, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x1a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x1a, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1a, 0x1a,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x1b, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x1b, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03,
    0x1c, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x03, 0x1c, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x03, 0x1c, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1c, 0x14, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1c, 0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x06, 0x12, 0x03, 0x1d, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x1d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1d,
    0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1d, 0x1a, 0x1d,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x21, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x04, 0x01, 0x12, 0x03, 0x21, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x22, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x14, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x23, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x23, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x23, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23,
    0x23, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x24, 0x04, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x24, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x24, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x24, 0x1d, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12,
    0x03, 0x25, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x25,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x25, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x25, 0x14, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x25, 0x22, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x26, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x26, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x26, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x26, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x26, 0x25,
    0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x29, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x29, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x2a, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x2a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2a,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x14, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x1a, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x2b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2b, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2b, 0x1b, 0x1f,
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
