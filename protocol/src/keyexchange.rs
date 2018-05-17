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
pub struct ClientHello {
    // message fields
    build_info: ::protobuf::SingularPtrField<BuildInfo>,
    fingerprints_supported: ::std::vec::Vec<Fingerprint>,
    cryptosuites_supported: ::std::vec::Vec<Cryptosuite>,
    powschemes_supported: ::std::vec::Vec<Powscheme>,
    login_crypto_hello: ::protobuf::SingularPtrField<LoginCryptoHelloUnion>,
    client_nonce: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    padding: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    feature_set: ::protobuf::SingularPtrField<FeatureSet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientHello {}

impl ClientHello {
    pub fn new() -> ClientHello {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientHello {
        static mut instance: ::protobuf::lazy::Lazy<ClientHello> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientHello,
        };
        unsafe {
            instance.get(ClientHello::new)
        }
    }

    // required .BuildInfo build_info = 10;

    pub fn clear_build_info(&mut self) {
        self.build_info.clear();
    }

    pub fn has_build_info(&self) -> bool {
        self.build_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_build_info(&mut self, v: BuildInfo) {
        self.build_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_build_info(&mut self) -> &mut BuildInfo {
        if self.build_info.is_none() {
            self.build_info.set_default();
        };
        self.build_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_build_info(&mut self) -> BuildInfo {
        self.build_info.take().unwrap_or_else(|| BuildInfo::new())
    }

    pub fn get_build_info(&self) -> &BuildInfo {
        self.build_info.as_ref().unwrap_or_else(|| BuildInfo::default_instance())
    }

    fn get_build_info_for_reflect(&self) -> &::protobuf::SingularPtrField<BuildInfo> {
        &self.build_info
    }

    fn mut_build_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BuildInfo> {
        &mut self.build_info
    }

    // repeated .Fingerprint fingerprints_supported = 20;

    pub fn clear_fingerprints_supported(&mut self) {
        self.fingerprints_supported.clear();
    }

    // Param is passed by value, moved
    pub fn set_fingerprints_supported(&mut self, v: ::std::vec::Vec<Fingerprint>) {
        self.fingerprints_supported = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fingerprints_supported(&mut self) -> &mut ::std::vec::Vec<Fingerprint> {
        &mut self.fingerprints_supported
    }

    // Take field
    pub fn take_fingerprints_supported(&mut self) -> ::std::vec::Vec<Fingerprint> {
        ::std::mem::replace(&mut self.fingerprints_supported, ::std::vec::Vec::new())
    }

    pub fn get_fingerprints_supported(&self) -> &[Fingerprint] {
        &self.fingerprints_supported
    }

    fn get_fingerprints_supported_for_reflect(&self) -> &::std::vec::Vec<Fingerprint> {
        &self.fingerprints_supported
    }

    fn mut_fingerprints_supported_for_reflect(&mut self) -> &mut ::std::vec::Vec<Fingerprint> {
        &mut self.fingerprints_supported
    }

    // repeated .Cryptosuite cryptosuites_supported = 30;

    pub fn clear_cryptosuites_supported(&mut self) {
        self.cryptosuites_supported.clear();
    }

    // Param is passed by value, moved
    pub fn set_cryptosuites_supported(&mut self, v: ::std::vec::Vec<Cryptosuite>) {
        self.cryptosuites_supported = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cryptosuites_supported(&mut self) -> &mut ::std::vec::Vec<Cryptosuite> {
        &mut self.cryptosuites_supported
    }

    // Take field
    pub fn take_cryptosuites_supported(&mut self) -> ::std::vec::Vec<Cryptosuite> {
        ::std::mem::replace(&mut self.cryptosuites_supported, ::std::vec::Vec::new())
    }

    pub fn get_cryptosuites_supported(&self) -> &[Cryptosuite] {
        &self.cryptosuites_supported
    }

    fn get_cryptosuites_supported_for_reflect(&self) -> &::std::vec::Vec<Cryptosuite> {
        &self.cryptosuites_supported
    }

    fn mut_cryptosuites_supported_for_reflect(&mut self) -> &mut ::std::vec::Vec<Cryptosuite> {
        &mut self.cryptosuites_supported
    }

    // repeated .Powscheme powschemes_supported = 40;

    pub fn clear_powschemes_supported(&mut self) {
        self.powschemes_supported.clear();
    }

    // Param is passed by value, moved
    pub fn set_powschemes_supported(&mut self, v: ::std::vec::Vec<Powscheme>) {
        self.powschemes_supported = v;
    }

    // Mutable pointer to the field.
    pub fn mut_powschemes_supported(&mut self) -> &mut ::std::vec::Vec<Powscheme> {
        &mut self.powschemes_supported
    }

    // Take field
    pub fn take_powschemes_supported(&mut self) -> ::std::vec::Vec<Powscheme> {
        ::std::mem::replace(&mut self.powschemes_supported, ::std::vec::Vec::new())
    }

    pub fn get_powschemes_supported(&self) -> &[Powscheme] {
        &self.powschemes_supported
    }

    fn get_powschemes_supported_for_reflect(&self) -> &::std::vec::Vec<Powscheme> {
        &self.powschemes_supported
    }

    fn mut_powschemes_supported_for_reflect(&mut self) -> &mut ::std::vec::Vec<Powscheme> {
        &mut self.powschemes_supported
    }

    // required .LoginCryptoHelloUnion login_crypto_hello = 50;

    pub fn clear_login_crypto_hello(&mut self) {
        self.login_crypto_hello.clear();
    }

    pub fn has_login_crypto_hello(&self) -> bool {
        self.login_crypto_hello.is_some()
    }

    // Param is passed by value, moved
    pub fn set_login_crypto_hello(&mut self, v: LoginCryptoHelloUnion) {
        self.login_crypto_hello = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_login_crypto_hello(&mut self) -> &mut LoginCryptoHelloUnion {
        if self.login_crypto_hello.is_none() {
            self.login_crypto_hello.set_default();
        };
        self.login_crypto_hello.as_mut().unwrap()
    }

    // Take field
    pub fn take_login_crypto_hello(&mut self) -> LoginCryptoHelloUnion {
        self.login_crypto_hello.take().unwrap_or_else(|| LoginCryptoHelloUnion::new())
    }

    pub fn get_login_crypto_hello(&self) -> &LoginCryptoHelloUnion {
        self.login_crypto_hello.as_ref().unwrap_or_else(|| LoginCryptoHelloUnion::default_instance())
    }

    fn get_login_crypto_hello_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoHelloUnion> {
        &self.login_crypto_hello
    }

    fn mut_login_crypto_hello_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoHelloUnion> {
        &mut self.login_crypto_hello
    }

    // required bytes client_nonce = 60;

    pub fn clear_client_nonce(&mut self) {
        self.client_nonce.clear();
    }

    pub fn has_client_nonce(&self) -> bool {
        self.client_nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_nonce(&mut self, v: ::std::vec::Vec<u8>) {
        self.client_nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_nonce(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.client_nonce.is_none() {
            self.client_nonce.set_default();
        };
        self.client_nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_nonce(&mut self) -> ::std::vec::Vec<u8> {
        self.client_nonce.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_client_nonce(&self) -> &[u8] {
        match self.client_nonce.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_client_nonce_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.client_nonce
    }

    fn mut_client_nonce_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.client_nonce
    }

    // optional bytes padding = 70;

    pub fn clear_padding(&mut self) {
        self.padding.clear();
    }

    pub fn has_padding(&self) -> bool {
        self.padding.is_some()
    }

    // Param is passed by value, moved
    pub fn set_padding(&mut self, v: ::std::vec::Vec<u8>) {
        self.padding = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_padding(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.padding.is_none() {
            self.padding.set_default();
        };
        self.padding.as_mut().unwrap()
    }

    // Take field
    pub fn take_padding(&mut self) -> ::std::vec::Vec<u8> {
        self.padding.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_padding(&self) -> &[u8] {
        match self.padding.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_padding_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.padding
    }

    fn mut_padding_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.padding
    }

    // optional .FeatureSet feature_set = 80;

    pub fn clear_feature_set(&mut self) {
        self.feature_set.clear();
    }

    pub fn has_feature_set(&self) -> bool {
        self.feature_set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_feature_set(&mut self, v: FeatureSet) {
        self.feature_set = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_feature_set(&mut self) -> &mut FeatureSet {
        if self.feature_set.is_none() {
            self.feature_set.set_default();
        };
        self.feature_set.as_mut().unwrap()
    }

    // Take field
    pub fn take_feature_set(&mut self) -> FeatureSet {
        self.feature_set.take().unwrap_or_else(|| FeatureSet::new())
    }

    pub fn get_feature_set(&self) -> &FeatureSet {
        self.feature_set.as_ref().unwrap_or_else(|| FeatureSet::default_instance())
    }

    fn get_feature_set_for_reflect(&self) -> &::protobuf::SingularPtrField<FeatureSet> {
        &self.feature_set
    }

    fn mut_feature_set_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FeatureSet> {
        &mut self.feature_set
    }
}

impl ::protobuf::Message for ClientHello {
    fn is_initialized(&self) -> bool {
        if self.build_info.is_none() {
            return false;
        };
        if self.login_crypto_hello.is_none() {
            return false;
        };
        if self.client_nonce.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.build_info)?;
                },
                20 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.fingerprints_supported)?;
                },
                30 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.cryptosuites_supported)?;
                },
                40 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.powschemes_supported)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login_crypto_hello)?;
                },
                60 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.client_nonce)?;
                },
                70 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.padding)?;
                },
                80 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.feature_set)?;
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
        if let Some(v) = self.build_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.fingerprints_supported {
            my_size += ::protobuf::rt::enum_size(20, *value);
        };
        for value in &self.cryptosuites_supported {
            my_size += ::protobuf::rt::enum_size(30, *value);
        };
        for value in &self.powschemes_supported {
            my_size += ::protobuf::rt::enum_size(40, *value);
        };
        if let Some(v) = self.login_crypto_hello.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.client_nonce.as_ref() {
            my_size += ::protobuf::rt::bytes_size(60, &v);
        };
        if let Some(v) = self.padding.as_ref() {
            my_size += ::protobuf::rt::bytes_size(70, &v);
        };
        if let Some(v) = self.feature_set.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.build_info.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.fingerprints_supported {
            os.write_enum(20, v.value())?;
        };
        for v in &self.cryptosuites_supported {
            os.write_enum(30, v.value())?;
        };
        for v in &self.powschemes_supported {
            os.write_enum(40, v.value())?;
        };
        if let Some(v) = self.login_crypto_hello.as_ref() {
            os.write_tag(50, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.client_nonce.as_ref() {
            os.write_bytes(60, &v)?;
        };
        if let Some(v) = self.padding.as_ref() {
            os.write_bytes(70, &v)?;
        };
        if let Some(v) = self.feature_set.as_ref() {
            os.write_tag(80, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ClientHello {
    fn new() -> ClientHello {
        ClientHello::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientHello>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BuildInfo>>(
                    "build_info",
                    ClientHello::get_build_info_for_reflect,
                    ClientHello::mut_build_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Fingerprint>>(
                    "fingerprints_supported",
                    ClientHello::get_fingerprints_supported_for_reflect,
                    ClientHello::mut_fingerprints_supported_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Cryptosuite>>(
                    "cryptosuites_supported",
                    ClientHello::get_cryptosuites_supported_for_reflect,
                    ClientHello::mut_cryptosuites_supported_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Powscheme>>(
                    "powschemes_supported",
                    ClientHello::get_powschemes_supported_for_reflect,
                    ClientHello::mut_powschemes_supported_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoHelloUnion>>(
                    "login_crypto_hello",
                    ClientHello::get_login_crypto_hello_for_reflect,
                    ClientHello::mut_login_crypto_hello_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "client_nonce",
                    ClientHello::get_client_nonce_for_reflect,
                    ClientHello::mut_client_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "padding",
                    ClientHello::get_padding_for_reflect,
                    ClientHello::mut_padding_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FeatureSet>>(
                    "feature_set",
                    ClientHello::get_feature_set_for_reflect,
                    ClientHello::mut_feature_set_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientHello>(
                    "ClientHello",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientHello {
    fn clear(&mut self) {
        self.clear_build_info();
        self.clear_fingerprints_supported();
        self.clear_cryptosuites_supported();
        self.clear_powschemes_supported();
        self.clear_login_crypto_hello();
        self.clear_client_nonce();
        self.clear_padding();
        self.clear_feature_set();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientHello {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientHello {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BuildInfo {
    // message fields
    product: ::std::option::Option<Product>,
    product_flags: ::std::vec::Vec<ProductFlags>,
    platform: ::std::option::Option<Platform>,
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BuildInfo {}

impl BuildInfo {
    pub fn new() -> BuildInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BuildInfo {
        static mut instance: ::protobuf::lazy::Lazy<BuildInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BuildInfo,
        };
        unsafe {
            instance.get(BuildInfo::new)
        }
    }

    // required .Product product = 10;

    pub fn clear_product(&mut self) {
        self.product = ::std::option::Option::None;
    }

    pub fn has_product(&self) -> bool {
        self.product.is_some()
    }

    // Param is passed by value, moved
    pub fn set_product(&mut self, v: Product) {
        self.product = ::std::option::Option::Some(v);
    }

    pub fn get_product(&self) -> Product {
        self.product.unwrap_or(Product::PRODUCT_CLIENT)
    }

    fn get_product_for_reflect(&self) -> &::std::option::Option<Product> {
        &self.product
    }

    fn mut_product_for_reflect(&mut self) -> &mut ::std::option::Option<Product> {
        &mut self.product
    }

    // repeated .ProductFlags product_flags = 20;

    pub fn clear_product_flags(&mut self) {
        self.product_flags.clear();
    }

    // Param is passed by value, moved
    pub fn set_product_flags(&mut self, v: ::std::vec::Vec<ProductFlags>) {
        self.product_flags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_product_flags(&mut self) -> &mut ::std::vec::Vec<ProductFlags> {
        &mut self.product_flags
    }

    // Take field
    pub fn take_product_flags(&mut self) -> ::std::vec::Vec<ProductFlags> {
        ::std::mem::replace(&mut self.product_flags, ::std::vec::Vec::new())
    }

    pub fn get_product_flags(&self) -> &[ProductFlags] {
        &self.product_flags
    }

    fn get_product_flags_for_reflect(&self) -> &::std::vec::Vec<ProductFlags> {
        &self.product_flags
    }

    fn mut_product_flags_for_reflect(&mut self) -> &mut ::std::vec::Vec<ProductFlags> {
        &mut self.product_flags
    }

    // required .Platform platform = 30;

    pub fn clear_platform(&mut self) {
        self.platform = ::std::option::Option::None;
    }

    pub fn has_platform(&self) -> bool {
        self.platform.is_some()
    }

    // Param is passed by value, moved
    pub fn set_platform(&mut self, v: Platform) {
        self.platform = ::std::option::Option::Some(v);
    }

    pub fn get_platform(&self) -> Platform {
        self.platform.unwrap_or(Platform::PLATFORM_WIN32_X86)
    }

    fn get_platform_for_reflect(&self) -> &::std::option::Option<Platform> {
        &self.platform
    }

    fn mut_platform_for_reflect(&mut self) -> &mut ::std::option::Option<Platform> {
        &mut self.platform
    }

    // required uint64 version = 40;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }
}

impl ::protobuf::Message for BuildInfo {
    fn is_initialized(&self) -> bool {
        if self.product.is_none() {
            return false;
        };
        if self.platform.is_none() {
            return false;
        };
        if self.version.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.product = ::std::option::Option::Some(tmp);
                },
                20 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.product_flags)?;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.platform = ::std::option::Option::Some(tmp);
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.product {
            my_size += ::protobuf::rt::enum_size(10, v);
        };
        for value in &self.product_flags {
            my_size += ::protobuf::rt::enum_size(20, *value);
        };
        if let Some(v) = self.platform {
            my_size += ::protobuf::rt::enum_size(30, v);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(40, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.product {
            os.write_enum(10, v.value())?;
        };
        for v in &self.product_flags {
            os.write_enum(20, v.value())?;
        };
        if let Some(v) = self.platform {
            os.write_enum(30, v.value())?;
        };
        if let Some(v) = self.version {
            os.write_uint64(40, v)?;
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

impl ::protobuf::MessageStatic for BuildInfo {
    fn new() -> BuildInfo {
        BuildInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<BuildInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Product>>(
                    "product",
                    BuildInfo::get_product_for_reflect,
                    BuildInfo::mut_product_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProductFlags>>(
                    "product_flags",
                    BuildInfo::get_product_flags_for_reflect,
                    BuildInfo::mut_product_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Platform>>(
                    "platform",
                    BuildInfo::get_platform_for_reflect,
                    BuildInfo::mut_platform_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    BuildInfo::get_version_for_reflect,
                    BuildInfo::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BuildInfo>(
                    "BuildInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BuildInfo {
    fn clear(&mut self) {
        self.clear_product();
        self.clear_product_flags();
        self.clear_platform();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BuildInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BuildInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoHelloUnion {
    // message fields
    diffie_hellman: ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanHello>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoHelloUnion {}

impl LoginCryptoHelloUnion {
    pub fn new() -> LoginCryptoHelloUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoHelloUnion {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoHelloUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoHelloUnion,
        };
        unsafe {
            instance.get(LoginCryptoHelloUnion::new)
        }
    }

    // optional .LoginCryptoDiffieHellmanHello diffie_hellman = 10;

    pub fn clear_diffie_hellman(&mut self) {
        self.diffie_hellman.clear();
    }

    pub fn has_diffie_hellman(&self) -> bool {
        self.diffie_hellman.is_some()
    }

    // Param is passed by value, moved
    pub fn set_diffie_hellman(&mut self, v: LoginCryptoDiffieHellmanHello) {
        self.diffie_hellman = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_diffie_hellman(&mut self) -> &mut LoginCryptoDiffieHellmanHello {
        if self.diffie_hellman.is_none() {
            self.diffie_hellman.set_default();
        };
        self.diffie_hellman.as_mut().unwrap()
    }

    // Take field
    pub fn take_diffie_hellman(&mut self) -> LoginCryptoDiffieHellmanHello {
        self.diffie_hellman.take().unwrap_or_else(|| LoginCryptoDiffieHellmanHello::new())
    }

    pub fn get_diffie_hellman(&self) -> &LoginCryptoDiffieHellmanHello {
        self.diffie_hellman.as_ref().unwrap_or_else(|| LoginCryptoDiffieHellmanHello::default_instance())
    }

    fn get_diffie_hellman_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoDiffieHellmanHello> {
        &self.diffie_hellman
    }

    fn mut_diffie_hellman_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanHello> {
        &mut self.diffie_hellman
    }
}

impl ::protobuf::Message for LoginCryptoHelloUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.diffie_hellman)?;
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
        if let Some(v) = self.diffie_hellman.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.diffie_hellman.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LoginCryptoHelloUnion {
    fn new() -> LoginCryptoHelloUnion {
        LoginCryptoHelloUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoHelloUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoDiffieHellmanHello>>(
                    "diffie_hellman",
                    LoginCryptoHelloUnion::get_diffie_hellman_for_reflect,
                    LoginCryptoHelloUnion::mut_diffie_hellman_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoHelloUnion>(
                    "LoginCryptoHelloUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoHelloUnion {
    fn clear(&mut self) {
        self.clear_diffie_hellman();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoHelloUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoHelloUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoDiffieHellmanHello {
    // message fields
    gc: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    server_keys_known: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoDiffieHellmanHello {}

impl LoginCryptoDiffieHellmanHello {
    pub fn new() -> LoginCryptoDiffieHellmanHello {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoDiffieHellmanHello {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoDiffieHellmanHello> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoDiffieHellmanHello,
        };
        unsafe {
            instance.get(LoginCryptoDiffieHellmanHello::new)
        }
    }

    // required bytes gc = 10;

    pub fn clear_gc(&mut self) {
        self.gc.clear();
    }

    pub fn has_gc(&self) -> bool {
        self.gc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc(&mut self, v: ::std::vec::Vec<u8>) {
        self.gc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gc(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gc.is_none() {
            self.gc.set_default();
        };
        self.gc.as_mut().unwrap()
    }

    // Take field
    pub fn take_gc(&mut self) -> ::std::vec::Vec<u8> {
        self.gc.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gc(&self) -> &[u8] {
        match self.gc.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gc_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gc
    }

    fn mut_gc_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gc
    }

    // required uint32 server_keys_known = 20;

    pub fn clear_server_keys_known(&mut self) {
        self.server_keys_known = ::std::option::Option::None;
    }

    pub fn has_server_keys_known(&self) -> bool {
        self.server_keys_known.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_keys_known(&mut self, v: u32) {
        self.server_keys_known = ::std::option::Option::Some(v);
    }

    pub fn get_server_keys_known(&self) -> u32 {
        self.server_keys_known.unwrap_or(0)
    }

    fn get_server_keys_known_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_keys_known
    }

    fn mut_server_keys_known_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_keys_known
    }
}

impl ::protobuf::Message for LoginCryptoDiffieHellmanHello {
    fn is_initialized(&self) -> bool {
        if self.gc.is_none() {
            return false;
        };
        if self.server_keys_known.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gc)?;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.server_keys_known = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.gc.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        if let Some(v) = self.server_keys_known {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gc.as_ref() {
            os.write_bytes(10, &v)?;
        };
        if let Some(v) = self.server_keys_known {
            os.write_uint32(20, v)?;
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

impl ::protobuf::MessageStatic for LoginCryptoDiffieHellmanHello {
    fn new() -> LoginCryptoDiffieHellmanHello {
        LoginCryptoDiffieHellmanHello::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoDiffieHellmanHello>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gc",
                    LoginCryptoDiffieHellmanHello::get_gc_for_reflect,
                    LoginCryptoDiffieHellmanHello::mut_gc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_keys_known",
                    LoginCryptoDiffieHellmanHello::get_server_keys_known_for_reflect,
                    LoginCryptoDiffieHellmanHello::mut_server_keys_known_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoDiffieHellmanHello>(
                    "LoginCryptoDiffieHellmanHello",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoDiffieHellmanHello {
    fn clear(&mut self) {
        self.clear_gc();
        self.clear_server_keys_known();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoDiffieHellmanHello {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoDiffieHellmanHello {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FeatureSet {
    // message fields
    autoupdate2: ::std::option::Option<bool>,
    current_location: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FeatureSet {}

impl FeatureSet {
    pub fn new() -> FeatureSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FeatureSet {
        static mut instance: ::protobuf::lazy::Lazy<FeatureSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FeatureSet,
        };
        unsafe {
            instance.get(FeatureSet::new)
        }
    }

    // optional bool autoupdate2 = 1;

    pub fn clear_autoupdate2(&mut self) {
        self.autoupdate2 = ::std::option::Option::None;
    }

    pub fn has_autoupdate2(&self) -> bool {
        self.autoupdate2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_autoupdate2(&mut self, v: bool) {
        self.autoupdate2 = ::std::option::Option::Some(v);
    }

    pub fn get_autoupdate2(&self) -> bool {
        self.autoupdate2.unwrap_or(false)
    }

    fn get_autoupdate2_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.autoupdate2
    }

    fn mut_autoupdate2_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.autoupdate2
    }

    // optional bool current_location = 2;

    pub fn clear_current_location(&mut self) {
        self.current_location = ::std::option::Option::None;
    }

    pub fn has_current_location(&self) -> bool {
        self.current_location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_location(&mut self, v: bool) {
        self.current_location = ::std::option::Option::Some(v);
    }

    pub fn get_current_location(&self) -> bool {
        self.current_location.unwrap_or(false)
    }

    fn get_current_location_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.current_location
    }

    fn mut_current_location_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.current_location
    }
}

impl ::protobuf::Message for FeatureSet {
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
                    let tmp = is.read_bool()?;
                    self.autoupdate2 = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.current_location = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.autoupdate2 {
            my_size += 2;
        };
        if let Some(v) = self.current_location {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.autoupdate2 {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.current_location {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for FeatureSet {
    fn new() -> FeatureSet {
        FeatureSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<FeatureSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "autoupdate2",
                    FeatureSet::get_autoupdate2_for_reflect,
                    FeatureSet::mut_autoupdate2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "current_location",
                    FeatureSet::get_current_location_for_reflect,
                    FeatureSet::mut_current_location_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FeatureSet>(
                    "FeatureSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FeatureSet {
    fn clear(&mut self) {
        self.clear_autoupdate2();
        self.clear_current_location();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FeatureSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeatureSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct APResponseMessage {
    // message fields
    challenge: ::protobuf::SingularPtrField<APChallenge>,
    upgrade: ::protobuf::SingularPtrField<UpgradeRequiredMessage>,
    login_failed: ::protobuf::SingularPtrField<APLoginFailed>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for APResponseMessage {}

impl APResponseMessage {
    pub fn new() -> APResponseMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static APResponseMessage {
        static mut instance: ::protobuf::lazy::Lazy<APResponseMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const APResponseMessage,
        };
        unsafe {
            instance.get(APResponseMessage::new)
        }
    }

    // optional .APChallenge challenge = 10;

    pub fn clear_challenge(&mut self) {
        self.challenge.clear();
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: APChallenge) {
        self.challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge(&mut self) -> &mut APChallenge {
        if self.challenge.is_none() {
            self.challenge.set_default();
        };
        self.challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenge(&mut self) -> APChallenge {
        self.challenge.take().unwrap_or_else(|| APChallenge::new())
    }

    pub fn get_challenge(&self) -> &APChallenge {
        self.challenge.as_ref().unwrap_or_else(|| APChallenge::default_instance())
    }

    fn get_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<APChallenge> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<APChallenge> {
        &mut self.challenge
    }

    // optional .UpgradeRequiredMessage upgrade = 20;

    pub fn clear_upgrade(&mut self) {
        self.upgrade.clear();
    }

    pub fn has_upgrade(&self) -> bool {
        self.upgrade.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrade(&mut self, v: UpgradeRequiredMessage) {
        self.upgrade = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upgrade(&mut self) -> &mut UpgradeRequiredMessage {
        if self.upgrade.is_none() {
            self.upgrade.set_default();
        };
        self.upgrade.as_mut().unwrap()
    }

    // Take field
    pub fn take_upgrade(&mut self) -> UpgradeRequiredMessage {
        self.upgrade.take().unwrap_or_else(|| UpgradeRequiredMessage::new())
    }

    pub fn get_upgrade(&self) -> &UpgradeRequiredMessage {
        self.upgrade.as_ref().unwrap_or_else(|| UpgradeRequiredMessage::default_instance())
    }

    fn get_upgrade_for_reflect(&self) -> &::protobuf::SingularPtrField<UpgradeRequiredMessage> {
        &self.upgrade
    }

    fn mut_upgrade_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UpgradeRequiredMessage> {
        &mut self.upgrade
    }

    // optional .APLoginFailed login_failed = 30;

    pub fn clear_login_failed(&mut self) {
        self.login_failed.clear();
    }

    pub fn has_login_failed(&self) -> bool {
        self.login_failed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_login_failed(&mut self, v: APLoginFailed) {
        self.login_failed = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_login_failed(&mut self) -> &mut APLoginFailed {
        if self.login_failed.is_none() {
            self.login_failed.set_default();
        };
        self.login_failed.as_mut().unwrap()
    }

    // Take field
    pub fn take_login_failed(&mut self) -> APLoginFailed {
        self.login_failed.take().unwrap_or_else(|| APLoginFailed::new())
    }

    pub fn get_login_failed(&self) -> &APLoginFailed {
        self.login_failed.as_ref().unwrap_or_else(|| APLoginFailed::default_instance())
    }

    fn get_login_failed_for_reflect(&self) -> &::protobuf::SingularPtrField<APLoginFailed> {
        &self.login_failed
    }

    fn mut_login_failed_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<APLoginFailed> {
        &mut self.login_failed
    }
}

impl ::protobuf::Message for APResponseMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.challenge)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.upgrade)?;
                },
                30 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login_failed)?;
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
        if let Some(v) = self.challenge.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.upgrade.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.login_failed.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.challenge.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.upgrade.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.login_failed.as_ref() {
            os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for APResponseMessage {
    fn new() -> APResponseMessage {
        APResponseMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<APResponseMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<APChallenge>>(
                    "challenge",
                    APResponseMessage::get_challenge_for_reflect,
                    APResponseMessage::mut_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UpgradeRequiredMessage>>(
                    "upgrade",
                    APResponseMessage::get_upgrade_for_reflect,
                    APResponseMessage::mut_upgrade_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<APLoginFailed>>(
                    "login_failed",
                    APResponseMessage::get_login_failed_for_reflect,
                    APResponseMessage::mut_login_failed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<APResponseMessage>(
                    "APResponseMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for APResponseMessage {
    fn clear(&mut self) {
        self.clear_challenge();
        self.clear_upgrade();
        self.clear_login_failed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for APResponseMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for APResponseMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct APChallenge {
    // message fields
    login_crypto_challenge: ::protobuf::SingularPtrField<LoginCryptoChallengeUnion>,
    fingerprint_challenge: ::protobuf::SingularPtrField<FingerprintChallengeUnion>,
    pow_challenge: ::protobuf::SingularPtrField<PoWChallengeUnion>,
    crypto_challenge: ::protobuf::SingularPtrField<CryptoChallengeUnion>,
    server_nonce: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    padding: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for APChallenge {}

impl APChallenge {
    pub fn new() -> APChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static APChallenge {
        static mut instance: ::protobuf::lazy::Lazy<APChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const APChallenge,
        };
        unsafe {
            instance.get(APChallenge::new)
        }
    }

    // required .LoginCryptoChallengeUnion login_crypto_challenge = 10;

    pub fn clear_login_crypto_challenge(&mut self) {
        self.login_crypto_challenge.clear();
    }

    pub fn has_login_crypto_challenge(&self) -> bool {
        self.login_crypto_challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_login_crypto_challenge(&mut self, v: LoginCryptoChallengeUnion) {
        self.login_crypto_challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_login_crypto_challenge(&mut self) -> &mut LoginCryptoChallengeUnion {
        if self.login_crypto_challenge.is_none() {
            self.login_crypto_challenge.set_default();
        };
        self.login_crypto_challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_login_crypto_challenge(&mut self) -> LoginCryptoChallengeUnion {
        self.login_crypto_challenge.take().unwrap_or_else(|| LoginCryptoChallengeUnion::new())
    }

    pub fn get_login_crypto_challenge(&self) -> &LoginCryptoChallengeUnion {
        self.login_crypto_challenge.as_ref().unwrap_or_else(|| LoginCryptoChallengeUnion::default_instance())
    }

    fn get_login_crypto_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoChallengeUnion> {
        &self.login_crypto_challenge
    }

    fn mut_login_crypto_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoChallengeUnion> {
        &mut self.login_crypto_challenge
    }

    // required .FingerprintChallengeUnion fingerprint_challenge = 20;

    pub fn clear_fingerprint_challenge(&mut self) {
        self.fingerprint_challenge.clear();
    }

    pub fn has_fingerprint_challenge(&self) -> bool {
        self.fingerprint_challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fingerprint_challenge(&mut self, v: FingerprintChallengeUnion) {
        self.fingerprint_challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fingerprint_challenge(&mut self) -> &mut FingerprintChallengeUnion {
        if self.fingerprint_challenge.is_none() {
            self.fingerprint_challenge.set_default();
        };
        self.fingerprint_challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_fingerprint_challenge(&mut self) -> FingerprintChallengeUnion {
        self.fingerprint_challenge.take().unwrap_or_else(|| FingerprintChallengeUnion::new())
    }

    pub fn get_fingerprint_challenge(&self) -> &FingerprintChallengeUnion {
        self.fingerprint_challenge.as_ref().unwrap_or_else(|| FingerprintChallengeUnion::default_instance())
    }

    fn get_fingerprint_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<FingerprintChallengeUnion> {
        &self.fingerprint_challenge
    }

    fn mut_fingerprint_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FingerprintChallengeUnion> {
        &mut self.fingerprint_challenge
    }

    // required .PoWChallengeUnion pow_challenge = 30;

    pub fn clear_pow_challenge(&mut self) {
        self.pow_challenge.clear();
    }

    pub fn has_pow_challenge(&self) -> bool {
        self.pow_challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pow_challenge(&mut self, v: PoWChallengeUnion) {
        self.pow_challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pow_challenge(&mut self) -> &mut PoWChallengeUnion {
        if self.pow_challenge.is_none() {
            self.pow_challenge.set_default();
        };
        self.pow_challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_pow_challenge(&mut self) -> PoWChallengeUnion {
        self.pow_challenge.take().unwrap_or_else(|| PoWChallengeUnion::new())
    }

    pub fn get_pow_challenge(&self) -> &PoWChallengeUnion {
        self.pow_challenge.as_ref().unwrap_or_else(|| PoWChallengeUnion::default_instance())
    }

    fn get_pow_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<PoWChallengeUnion> {
        &self.pow_challenge
    }

    fn mut_pow_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PoWChallengeUnion> {
        &mut self.pow_challenge
    }

    // required .CryptoChallengeUnion crypto_challenge = 40;

    pub fn clear_crypto_challenge(&mut self) {
        self.crypto_challenge.clear();
    }

    pub fn has_crypto_challenge(&self) -> bool {
        self.crypto_challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crypto_challenge(&mut self, v: CryptoChallengeUnion) {
        self.crypto_challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_crypto_challenge(&mut self) -> &mut CryptoChallengeUnion {
        if self.crypto_challenge.is_none() {
            self.crypto_challenge.set_default();
        };
        self.crypto_challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_crypto_challenge(&mut self) -> CryptoChallengeUnion {
        self.crypto_challenge.take().unwrap_or_else(|| CryptoChallengeUnion::new())
    }

    pub fn get_crypto_challenge(&self) -> &CryptoChallengeUnion {
        self.crypto_challenge.as_ref().unwrap_or_else(|| CryptoChallengeUnion::default_instance())
    }

    fn get_crypto_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoChallengeUnion> {
        &self.crypto_challenge
    }

    fn mut_crypto_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoChallengeUnion> {
        &mut self.crypto_challenge
    }

    // required bytes server_nonce = 50;

    pub fn clear_server_nonce(&mut self) {
        self.server_nonce.clear();
    }

    pub fn has_server_nonce(&self) -> bool {
        self.server_nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_nonce(&mut self, v: ::std::vec::Vec<u8>) {
        self.server_nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_nonce(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.server_nonce.is_none() {
            self.server_nonce.set_default();
        };
        self.server_nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_nonce(&mut self) -> ::std::vec::Vec<u8> {
        self.server_nonce.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_server_nonce(&self) -> &[u8] {
        match self.server_nonce.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_server_nonce_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.server_nonce
    }

    fn mut_server_nonce_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.server_nonce
    }

    // optional bytes padding = 60;

    pub fn clear_padding(&mut self) {
        self.padding.clear();
    }

    pub fn has_padding(&self) -> bool {
        self.padding.is_some()
    }

    // Param is passed by value, moved
    pub fn set_padding(&mut self, v: ::std::vec::Vec<u8>) {
        self.padding = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_padding(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.padding.is_none() {
            self.padding.set_default();
        };
        self.padding.as_mut().unwrap()
    }

    // Take field
    pub fn take_padding(&mut self) -> ::std::vec::Vec<u8> {
        self.padding.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_padding(&self) -> &[u8] {
        match self.padding.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_padding_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.padding
    }

    fn mut_padding_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.padding
    }
}

impl ::protobuf::Message for APChallenge {
    fn is_initialized(&self) -> bool {
        if self.login_crypto_challenge.is_none() {
            return false;
        };
        if self.fingerprint_challenge.is_none() {
            return false;
        };
        if self.pow_challenge.is_none() {
            return false;
        };
        if self.crypto_challenge.is_none() {
            return false;
        };
        if self.server_nonce.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login_crypto_challenge)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fingerprint_challenge)?;
                },
                30 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pow_challenge)?;
                },
                40 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.crypto_challenge)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.server_nonce)?;
                },
                60 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.padding)?;
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
        if let Some(v) = self.login_crypto_challenge.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.fingerprint_challenge.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.pow_challenge.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.crypto_challenge.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.server_nonce.as_ref() {
            my_size += ::protobuf::rt::bytes_size(50, &v);
        };
        if let Some(v) = self.padding.as_ref() {
            my_size += ::protobuf::rt::bytes_size(60, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.login_crypto_challenge.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.fingerprint_challenge.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.pow_challenge.as_ref() {
            os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.crypto_challenge.as_ref() {
            os.write_tag(40, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.server_nonce.as_ref() {
            os.write_bytes(50, &v)?;
        };
        if let Some(v) = self.padding.as_ref() {
            os.write_bytes(60, &v)?;
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

impl ::protobuf::MessageStatic for APChallenge {
    fn new() -> APChallenge {
        APChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<APChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoChallengeUnion>>(
                    "login_crypto_challenge",
                    APChallenge::get_login_crypto_challenge_for_reflect,
                    APChallenge::mut_login_crypto_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FingerprintChallengeUnion>>(
                    "fingerprint_challenge",
                    APChallenge::get_fingerprint_challenge_for_reflect,
                    APChallenge::mut_fingerprint_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PoWChallengeUnion>>(
                    "pow_challenge",
                    APChallenge::get_pow_challenge_for_reflect,
                    APChallenge::mut_pow_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoChallengeUnion>>(
                    "crypto_challenge",
                    APChallenge::get_crypto_challenge_for_reflect,
                    APChallenge::mut_crypto_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "server_nonce",
                    APChallenge::get_server_nonce_for_reflect,
                    APChallenge::mut_server_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "padding",
                    APChallenge::get_padding_for_reflect,
                    APChallenge::mut_padding_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<APChallenge>(
                    "APChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for APChallenge {
    fn clear(&mut self) {
        self.clear_login_crypto_challenge();
        self.clear_fingerprint_challenge();
        self.clear_pow_challenge();
        self.clear_crypto_challenge();
        self.clear_server_nonce();
        self.clear_padding();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for APChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for APChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoChallengeUnion {
    // message fields
    diffie_hellman: ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanChallenge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoChallengeUnion {}

impl LoginCryptoChallengeUnion {
    pub fn new() -> LoginCryptoChallengeUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoChallengeUnion {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoChallengeUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoChallengeUnion,
        };
        unsafe {
            instance.get(LoginCryptoChallengeUnion::new)
        }
    }

    // optional .LoginCryptoDiffieHellmanChallenge diffie_hellman = 10;

    pub fn clear_diffie_hellman(&mut self) {
        self.diffie_hellman.clear();
    }

    pub fn has_diffie_hellman(&self) -> bool {
        self.diffie_hellman.is_some()
    }

    // Param is passed by value, moved
    pub fn set_diffie_hellman(&mut self, v: LoginCryptoDiffieHellmanChallenge) {
        self.diffie_hellman = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_diffie_hellman(&mut self) -> &mut LoginCryptoDiffieHellmanChallenge {
        if self.diffie_hellman.is_none() {
            self.diffie_hellman.set_default();
        };
        self.diffie_hellman.as_mut().unwrap()
    }

    // Take field
    pub fn take_diffie_hellman(&mut self) -> LoginCryptoDiffieHellmanChallenge {
        self.diffie_hellman.take().unwrap_or_else(|| LoginCryptoDiffieHellmanChallenge::new())
    }

    pub fn get_diffie_hellman(&self) -> &LoginCryptoDiffieHellmanChallenge {
        self.diffie_hellman.as_ref().unwrap_or_else(|| LoginCryptoDiffieHellmanChallenge::default_instance())
    }

    fn get_diffie_hellman_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoDiffieHellmanChallenge> {
        &self.diffie_hellman
    }

    fn mut_diffie_hellman_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanChallenge> {
        &mut self.diffie_hellman
    }
}

impl ::protobuf::Message for LoginCryptoChallengeUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.diffie_hellman)?;
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
        if let Some(v) = self.diffie_hellman.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.diffie_hellman.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LoginCryptoChallengeUnion {
    fn new() -> LoginCryptoChallengeUnion {
        LoginCryptoChallengeUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoChallengeUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoDiffieHellmanChallenge>>(
                    "diffie_hellman",
                    LoginCryptoChallengeUnion::get_diffie_hellman_for_reflect,
                    LoginCryptoChallengeUnion::mut_diffie_hellman_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoChallengeUnion>(
                    "LoginCryptoChallengeUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoChallengeUnion {
    fn clear(&mut self) {
        self.clear_diffie_hellman();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoChallengeUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoChallengeUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoDiffieHellmanChallenge {
    // message fields
    gs: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    server_signature_key: ::std::option::Option<i32>,
    gs_signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoDiffieHellmanChallenge {}

impl LoginCryptoDiffieHellmanChallenge {
    pub fn new() -> LoginCryptoDiffieHellmanChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoDiffieHellmanChallenge {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoDiffieHellmanChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoDiffieHellmanChallenge,
        };
        unsafe {
            instance.get(LoginCryptoDiffieHellmanChallenge::new)
        }
    }

    // required bytes gs = 10;

    pub fn clear_gs(&mut self) {
        self.gs.clear();
    }

    pub fn has_gs(&self) -> bool {
        self.gs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gs(&mut self, v: ::std::vec::Vec<u8>) {
        self.gs = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gs(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gs.is_none() {
            self.gs.set_default();
        };
        self.gs.as_mut().unwrap()
    }

    // Take field
    pub fn take_gs(&mut self) -> ::std::vec::Vec<u8> {
        self.gs.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gs(&self) -> &[u8] {
        match self.gs.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gs_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gs
    }

    fn mut_gs_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gs
    }

    // required int32 server_signature_key = 20;

    pub fn clear_server_signature_key(&mut self) {
        self.server_signature_key = ::std::option::Option::None;
    }

    pub fn has_server_signature_key(&self) -> bool {
        self.server_signature_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_signature_key(&mut self, v: i32) {
        self.server_signature_key = ::std::option::Option::Some(v);
    }

    pub fn get_server_signature_key(&self) -> i32 {
        self.server_signature_key.unwrap_or(0)
    }

    fn get_server_signature_key_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.server_signature_key
    }

    fn mut_server_signature_key_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.server_signature_key
    }

    // required bytes gs_signature = 30;

    pub fn clear_gs_signature(&mut self) {
        self.gs_signature.clear();
    }

    pub fn has_gs_signature(&self) -> bool {
        self.gs_signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gs_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.gs_signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gs_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gs_signature.is_none() {
            self.gs_signature.set_default();
        };
        self.gs_signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_gs_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.gs_signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gs_signature(&self) -> &[u8] {
        match self.gs_signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gs_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gs_signature
    }

    fn mut_gs_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gs_signature
    }
}

impl ::protobuf::Message for LoginCryptoDiffieHellmanChallenge {
    fn is_initialized(&self) -> bool {
        if self.gs.is_none() {
            return false;
        };
        if self.server_signature_key.is_none() {
            return false;
        };
        if self.gs_signature.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gs)?;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.server_signature_key = ::std::option::Option::Some(tmp);
                },
                30 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gs_signature)?;
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
        if let Some(v) = self.gs.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        if let Some(v) = self.server_signature_key {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.gs_signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(30, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gs.as_ref() {
            os.write_bytes(10, &v)?;
        };
        if let Some(v) = self.server_signature_key {
            os.write_int32(20, v)?;
        };
        if let Some(v) = self.gs_signature.as_ref() {
            os.write_bytes(30, &v)?;
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

impl ::protobuf::MessageStatic for LoginCryptoDiffieHellmanChallenge {
    fn new() -> LoginCryptoDiffieHellmanChallenge {
        LoginCryptoDiffieHellmanChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoDiffieHellmanChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gs",
                    LoginCryptoDiffieHellmanChallenge::get_gs_for_reflect,
                    LoginCryptoDiffieHellmanChallenge::mut_gs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "server_signature_key",
                    LoginCryptoDiffieHellmanChallenge::get_server_signature_key_for_reflect,
                    LoginCryptoDiffieHellmanChallenge::mut_server_signature_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gs_signature",
                    LoginCryptoDiffieHellmanChallenge::get_gs_signature_for_reflect,
                    LoginCryptoDiffieHellmanChallenge::mut_gs_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoDiffieHellmanChallenge>(
                    "LoginCryptoDiffieHellmanChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoDiffieHellmanChallenge {
    fn clear(&mut self) {
        self.clear_gs();
        self.clear_server_signature_key();
        self.clear_gs_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoDiffieHellmanChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoDiffieHellmanChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FingerprintChallengeUnion {
    // message fields
    grain: ::protobuf::SingularPtrField<FingerprintGrainChallenge>,
    hmac_ripemd: ::protobuf::SingularPtrField<FingerprintHmacRipemdChallenge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FingerprintChallengeUnion {}

impl FingerprintChallengeUnion {
    pub fn new() -> FingerprintChallengeUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FingerprintChallengeUnion {
        static mut instance: ::protobuf::lazy::Lazy<FingerprintChallengeUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FingerprintChallengeUnion,
        };
        unsafe {
            instance.get(FingerprintChallengeUnion::new)
        }
    }

    // optional .FingerprintGrainChallenge grain = 10;

    pub fn clear_grain(&mut self) {
        self.grain.clear();
    }

    pub fn has_grain(&self) -> bool {
        self.grain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_grain(&mut self, v: FingerprintGrainChallenge) {
        self.grain = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_grain(&mut self) -> &mut FingerprintGrainChallenge {
        if self.grain.is_none() {
            self.grain.set_default();
        };
        self.grain.as_mut().unwrap()
    }

    // Take field
    pub fn take_grain(&mut self) -> FingerprintGrainChallenge {
        self.grain.take().unwrap_or_else(|| FingerprintGrainChallenge::new())
    }

    pub fn get_grain(&self) -> &FingerprintGrainChallenge {
        self.grain.as_ref().unwrap_or_else(|| FingerprintGrainChallenge::default_instance())
    }

    fn get_grain_for_reflect(&self) -> &::protobuf::SingularPtrField<FingerprintGrainChallenge> {
        &self.grain
    }

    fn mut_grain_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FingerprintGrainChallenge> {
        &mut self.grain
    }

    // optional .FingerprintHmacRipemdChallenge hmac_ripemd = 20;

    pub fn clear_hmac_ripemd(&mut self) {
        self.hmac_ripemd.clear();
    }

    pub fn has_hmac_ripemd(&self) -> bool {
        self.hmac_ripemd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hmac_ripemd(&mut self, v: FingerprintHmacRipemdChallenge) {
        self.hmac_ripemd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hmac_ripemd(&mut self) -> &mut FingerprintHmacRipemdChallenge {
        if self.hmac_ripemd.is_none() {
            self.hmac_ripemd.set_default();
        };
        self.hmac_ripemd.as_mut().unwrap()
    }

    // Take field
    pub fn take_hmac_ripemd(&mut self) -> FingerprintHmacRipemdChallenge {
        self.hmac_ripemd.take().unwrap_or_else(|| FingerprintHmacRipemdChallenge::new())
    }

    pub fn get_hmac_ripemd(&self) -> &FingerprintHmacRipemdChallenge {
        self.hmac_ripemd.as_ref().unwrap_or_else(|| FingerprintHmacRipemdChallenge::default_instance())
    }

    fn get_hmac_ripemd_for_reflect(&self) -> &::protobuf::SingularPtrField<FingerprintHmacRipemdChallenge> {
        &self.hmac_ripemd
    }

    fn mut_hmac_ripemd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FingerprintHmacRipemdChallenge> {
        &mut self.hmac_ripemd
    }
}

impl ::protobuf::Message for FingerprintChallengeUnion {
    fn is_initialized(&self) -> bool {
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
        if let Some(v) = self.grain.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hmac_ripemd.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.grain.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hmac_ripemd.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for FingerprintChallengeUnion {
    fn new() -> FingerprintChallengeUnion {
        FingerprintChallengeUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<FingerprintChallengeUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FingerprintGrainChallenge>>(
                    "grain",
                    FingerprintChallengeUnion::get_grain_for_reflect,
                    FingerprintChallengeUnion::mut_grain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FingerprintHmacRipemdChallenge>>(
                    "hmac_ripemd",
                    FingerprintChallengeUnion::get_hmac_ripemd_for_reflect,
                    FingerprintChallengeUnion::mut_hmac_ripemd_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FingerprintChallengeUnion>(
                    "FingerprintChallengeUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FingerprintChallengeUnion {
    fn clear(&mut self) {
        self.clear_grain();
        self.clear_hmac_ripemd();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FingerprintChallengeUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FingerprintChallengeUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FingerprintGrainChallenge {
    // message fields
    kek: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FingerprintGrainChallenge {}

impl FingerprintGrainChallenge {
    pub fn new() -> FingerprintGrainChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FingerprintGrainChallenge {
        static mut instance: ::protobuf::lazy::Lazy<FingerprintGrainChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FingerprintGrainChallenge,
        };
        unsafe {
            instance.get(FingerprintGrainChallenge::new)
        }
    }

    // required bytes kek = 10;

    pub fn clear_kek(&mut self) {
        self.kek.clear();
    }

    pub fn has_kek(&self) -> bool {
        self.kek.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kek(&mut self, v: ::std::vec::Vec<u8>) {
        self.kek = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kek(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.kek.is_none() {
            self.kek.set_default();
        };
        self.kek.as_mut().unwrap()
    }

    // Take field
    pub fn take_kek(&mut self) -> ::std::vec::Vec<u8> {
        self.kek.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_kek(&self) -> &[u8] {
        match self.kek.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_kek_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.kek
    }

    fn mut_kek_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.kek
    }
}

impl ::protobuf::Message for FingerprintGrainChallenge {
    fn is_initialized(&self) -> bool {
        if self.kek.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.kek)?;
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
        if let Some(v) = self.kek.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.kek.as_ref() {
            os.write_bytes(10, &v)?;
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

impl ::protobuf::MessageStatic for FingerprintGrainChallenge {
    fn new() -> FingerprintGrainChallenge {
        FingerprintGrainChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<FingerprintGrainChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "kek",
                    FingerprintGrainChallenge::get_kek_for_reflect,
                    FingerprintGrainChallenge::mut_kek_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FingerprintGrainChallenge>(
                    "FingerprintGrainChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FingerprintGrainChallenge {
    fn clear(&mut self) {
        self.clear_kek();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FingerprintGrainChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FingerprintGrainChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FingerprintHmacRipemdChallenge {
    // message fields
    challenge: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FingerprintHmacRipemdChallenge {}

impl FingerprintHmacRipemdChallenge {
    pub fn new() -> FingerprintHmacRipemdChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FingerprintHmacRipemdChallenge {
        static mut instance: ::protobuf::lazy::Lazy<FingerprintHmacRipemdChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FingerprintHmacRipemdChallenge,
        };
        unsafe {
            instance.get(FingerprintHmacRipemdChallenge::new)
        }
    }

    // required bytes challenge = 10;

    pub fn clear_challenge(&mut self) {
        self.challenge.clear();
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: ::std::vec::Vec<u8>) {
        self.challenge = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.challenge.is_none() {
            self.challenge.set_default();
        };
        self.challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenge(&mut self) -> ::std::vec::Vec<u8> {
        self.challenge.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_challenge(&self) -> &[u8] {
        match self.challenge.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_challenge_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.challenge
    }
}

impl ::protobuf::Message for FingerprintHmacRipemdChallenge {
    fn is_initialized(&self) -> bool {
        if self.challenge.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.challenge)?;
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
        if let Some(v) = self.challenge.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.challenge.as_ref() {
            os.write_bytes(10, &v)?;
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

impl ::protobuf::MessageStatic for FingerprintHmacRipemdChallenge {
    fn new() -> FingerprintHmacRipemdChallenge {
        FingerprintHmacRipemdChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<FingerprintHmacRipemdChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "challenge",
                    FingerprintHmacRipemdChallenge::get_challenge_for_reflect,
                    FingerprintHmacRipemdChallenge::mut_challenge_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FingerprintHmacRipemdChallenge>(
                    "FingerprintHmacRipemdChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FingerprintHmacRipemdChallenge {
    fn clear(&mut self) {
        self.clear_challenge();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FingerprintHmacRipemdChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FingerprintHmacRipemdChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoWChallengeUnion {
    // message fields
    hash_cash: ::protobuf::SingularPtrField<PoWHashCashChallenge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoWChallengeUnion {}

impl PoWChallengeUnion {
    pub fn new() -> PoWChallengeUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoWChallengeUnion {
        static mut instance: ::protobuf::lazy::Lazy<PoWChallengeUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoWChallengeUnion,
        };
        unsafe {
            instance.get(PoWChallengeUnion::new)
        }
    }

    // optional .PoWHashCashChallenge hash_cash = 10;

    pub fn clear_hash_cash(&mut self) {
        self.hash_cash.clear();
    }

    pub fn has_hash_cash(&self) -> bool {
        self.hash_cash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash_cash(&mut self, v: PoWHashCashChallenge) {
        self.hash_cash = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash_cash(&mut self) -> &mut PoWHashCashChallenge {
        if self.hash_cash.is_none() {
            self.hash_cash.set_default();
        };
        self.hash_cash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash_cash(&mut self) -> PoWHashCashChallenge {
        self.hash_cash.take().unwrap_or_else(|| PoWHashCashChallenge::new())
    }

    pub fn get_hash_cash(&self) -> &PoWHashCashChallenge {
        self.hash_cash.as_ref().unwrap_or_else(|| PoWHashCashChallenge::default_instance())
    }

    fn get_hash_cash_for_reflect(&self) -> &::protobuf::SingularPtrField<PoWHashCashChallenge> {
        &self.hash_cash
    }

    fn mut_hash_cash_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PoWHashCashChallenge> {
        &mut self.hash_cash
    }
}

impl ::protobuf::Message for PoWChallengeUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hash_cash)?;
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
        if let Some(v) = self.hash_cash.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash_cash.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for PoWChallengeUnion {
    fn new() -> PoWChallengeUnion {
        PoWChallengeUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoWChallengeUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PoWHashCashChallenge>>(
                    "hash_cash",
                    PoWChallengeUnion::get_hash_cash_for_reflect,
                    PoWChallengeUnion::mut_hash_cash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoWChallengeUnion>(
                    "PoWChallengeUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoWChallengeUnion {
    fn clear(&mut self) {
        self.clear_hash_cash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoWChallengeUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoWChallengeUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoWHashCashChallenge {
    // message fields
    prefix: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    length: ::std::option::Option<i32>,
    target: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoWHashCashChallenge {}

impl PoWHashCashChallenge {
    pub fn new() -> PoWHashCashChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoWHashCashChallenge {
        static mut instance: ::protobuf::lazy::Lazy<PoWHashCashChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoWHashCashChallenge,
        };
        unsafe {
            instance.get(PoWHashCashChallenge::new)
        }
    }

    // optional bytes prefix = 10;

    pub fn clear_prefix(&mut self) {
        self.prefix.clear();
    }

    pub fn has_prefix(&self) -> bool {
        self.prefix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prefix(&mut self, v: ::std::vec::Vec<u8>) {
        self.prefix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prefix(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.prefix.is_none() {
            self.prefix.set_default();
        };
        self.prefix.as_mut().unwrap()
    }

    // Take field
    pub fn take_prefix(&mut self) -> ::std::vec::Vec<u8> {
        self.prefix.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_prefix(&self) -> &[u8] {
        match self.prefix.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_prefix_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.prefix
    }

    fn mut_prefix_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.prefix
    }

    // optional int32 length = 20;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: i32) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> i32 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.length
    }

    // optional int32 target = 30;

    pub fn clear_target(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: i32) {
        self.target = ::std::option::Option::Some(v);
    }

    pub fn get_target(&self) -> i32 {
        self.target.unwrap_or(0)
    }

    fn get_target_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.target
    }
}

impl ::protobuf::Message for PoWHashCashChallenge {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.prefix)?;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.target = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.prefix.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.target {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.prefix.as_ref() {
            os.write_bytes(10, &v)?;
        };
        if let Some(v) = self.length {
            os.write_int32(20, v)?;
        };
        if let Some(v) = self.target {
            os.write_int32(30, v)?;
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

impl ::protobuf::MessageStatic for PoWHashCashChallenge {
    fn new() -> PoWHashCashChallenge {
        PoWHashCashChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoWHashCashChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "prefix",
                    PoWHashCashChallenge::get_prefix_for_reflect,
                    PoWHashCashChallenge::mut_prefix_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "length",
                    PoWHashCashChallenge::get_length_for_reflect,
                    PoWHashCashChallenge::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "target",
                    PoWHashCashChallenge::get_target_for_reflect,
                    PoWHashCashChallenge::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoWHashCashChallenge>(
                    "PoWHashCashChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoWHashCashChallenge {
    fn clear(&mut self) {
        self.clear_prefix();
        self.clear_length();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoWHashCashChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoWHashCashChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoChallengeUnion {
    // message fields
    shannon: ::protobuf::SingularPtrField<CryptoShannonChallenge>,
    rc4_sha1_hmac: ::protobuf::SingularPtrField<CryptoRc4Sha1HmacChallenge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoChallengeUnion {}

impl CryptoChallengeUnion {
    pub fn new() -> CryptoChallengeUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoChallengeUnion {
        static mut instance: ::protobuf::lazy::Lazy<CryptoChallengeUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoChallengeUnion,
        };
        unsafe {
            instance.get(CryptoChallengeUnion::new)
        }
    }

    // optional .CryptoShannonChallenge shannon = 10;

    pub fn clear_shannon(&mut self) {
        self.shannon.clear();
    }

    pub fn has_shannon(&self) -> bool {
        self.shannon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shannon(&mut self, v: CryptoShannonChallenge) {
        self.shannon = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shannon(&mut self) -> &mut CryptoShannonChallenge {
        if self.shannon.is_none() {
            self.shannon.set_default();
        };
        self.shannon.as_mut().unwrap()
    }

    // Take field
    pub fn take_shannon(&mut self) -> CryptoShannonChallenge {
        self.shannon.take().unwrap_or_else(|| CryptoShannonChallenge::new())
    }

    pub fn get_shannon(&self) -> &CryptoShannonChallenge {
        self.shannon.as_ref().unwrap_or_else(|| CryptoShannonChallenge::default_instance())
    }

    fn get_shannon_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoShannonChallenge> {
        &self.shannon
    }

    fn mut_shannon_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoShannonChallenge> {
        &mut self.shannon
    }

    // optional .CryptoRc4Sha1HmacChallenge rc4_sha1_hmac = 20;

    pub fn clear_rc4_sha1_hmac(&mut self) {
        self.rc4_sha1_hmac.clear();
    }

    pub fn has_rc4_sha1_hmac(&self) -> bool {
        self.rc4_sha1_hmac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rc4_sha1_hmac(&mut self, v: CryptoRc4Sha1HmacChallenge) {
        self.rc4_sha1_hmac = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rc4_sha1_hmac(&mut self) -> &mut CryptoRc4Sha1HmacChallenge {
        if self.rc4_sha1_hmac.is_none() {
            self.rc4_sha1_hmac.set_default();
        };
        self.rc4_sha1_hmac.as_mut().unwrap()
    }

    // Take field
    pub fn take_rc4_sha1_hmac(&mut self) -> CryptoRc4Sha1HmacChallenge {
        self.rc4_sha1_hmac.take().unwrap_or_else(|| CryptoRc4Sha1HmacChallenge::new())
    }

    pub fn get_rc4_sha1_hmac(&self) -> &CryptoRc4Sha1HmacChallenge {
        self.rc4_sha1_hmac.as_ref().unwrap_or_else(|| CryptoRc4Sha1HmacChallenge::default_instance())
    }

    fn get_rc4_sha1_hmac_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoRc4Sha1HmacChallenge> {
        &self.rc4_sha1_hmac
    }

    fn mut_rc4_sha1_hmac_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoRc4Sha1HmacChallenge> {
        &mut self.rc4_sha1_hmac
    }
}

impl ::protobuf::Message for CryptoChallengeUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shannon)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rc4_sha1_hmac)?;
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
        if let Some(v) = self.shannon.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.rc4_sha1_hmac.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.shannon.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.rc4_sha1_hmac.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CryptoChallengeUnion {
    fn new() -> CryptoChallengeUnion {
        CryptoChallengeUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoChallengeUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoShannonChallenge>>(
                    "shannon",
                    CryptoChallengeUnion::get_shannon_for_reflect,
                    CryptoChallengeUnion::mut_shannon_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoRc4Sha1HmacChallenge>>(
                    "rc4_sha1_hmac",
                    CryptoChallengeUnion::get_rc4_sha1_hmac_for_reflect,
                    CryptoChallengeUnion::mut_rc4_sha1_hmac_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CryptoChallengeUnion>(
                    "CryptoChallengeUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoChallengeUnion {
    fn clear(&mut self) {
        self.clear_shannon();
        self.clear_rc4_sha1_hmac();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoChallengeUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoChallengeUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoShannonChallenge {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoShannonChallenge {}

impl CryptoShannonChallenge {
    pub fn new() -> CryptoShannonChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoShannonChallenge {
        static mut instance: ::protobuf::lazy::Lazy<CryptoShannonChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoShannonChallenge,
        };
        unsafe {
            instance.get(CryptoShannonChallenge::new)
        }
    }
}

impl ::protobuf::Message for CryptoShannonChallenge {
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

impl ::protobuf::MessageStatic for CryptoShannonChallenge {
    fn new() -> CryptoShannonChallenge {
        CryptoShannonChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoShannonChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CryptoShannonChallenge>(
                    "CryptoShannonChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoShannonChallenge {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoShannonChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoShannonChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoRc4Sha1HmacChallenge {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoRc4Sha1HmacChallenge {}

impl CryptoRc4Sha1HmacChallenge {
    pub fn new() -> CryptoRc4Sha1HmacChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoRc4Sha1HmacChallenge {
        static mut instance: ::protobuf::lazy::Lazy<CryptoRc4Sha1HmacChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoRc4Sha1HmacChallenge,
        };
        unsafe {
            instance.get(CryptoRc4Sha1HmacChallenge::new)
        }
    }
}

impl ::protobuf::Message for CryptoRc4Sha1HmacChallenge {
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

impl ::protobuf::MessageStatic for CryptoRc4Sha1HmacChallenge {
    fn new() -> CryptoRc4Sha1HmacChallenge {
        CryptoRc4Sha1HmacChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoRc4Sha1HmacChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CryptoRc4Sha1HmacChallenge>(
                    "CryptoRc4Sha1HmacChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoRc4Sha1HmacChallenge {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoRc4Sha1HmacChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoRc4Sha1HmacChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpgradeRequiredMessage {
    // message fields
    upgrade_signed_part: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    http_suffix: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpgradeRequiredMessage {}

impl UpgradeRequiredMessage {
    pub fn new() -> UpgradeRequiredMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpgradeRequiredMessage {
        static mut instance: ::protobuf::lazy::Lazy<UpgradeRequiredMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpgradeRequiredMessage,
        };
        unsafe {
            instance.get(UpgradeRequiredMessage::new)
        }
    }

    // required bytes upgrade_signed_part = 10;

    pub fn clear_upgrade_signed_part(&mut self) {
        self.upgrade_signed_part.clear();
    }

    pub fn has_upgrade_signed_part(&self) -> bool {
        self.upgrade_signed_part.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrade_signed_part(&mut self, v: ::std::vec::Vec<u8>) {
        self.upgrade_signed_part = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upgrade_signed_part(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.upgrade_signed_part.is_none() {
            self.upgrade_signed_part.set_default();
        };
        self.upgrade_signed_part.as_mut().unwrap()
    }

    // Take field
    pub fn take_upgrade_signed_part(&mut self) -> ::std::vec::Vec<u8> {
        self.upgrade_signed_part.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_upgrade_signed_part(&self) -> &[u8] {
        match self.upgrade_signed_part.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_upgrade_signed_part_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.upgrade_signed_part
    }

    fn mut_upgrade_signed_part_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.upgrade_signed_part
    }

    // required bytes signature = 20;

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

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }

    // optional string http_suffix = 30;

    pub fn clear_http_suffix(&mut self) {
        self.http_suffix.clear();
    }

    pub fn has_http_suffix(&self) -> bool {
        self.http_suffix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http_suffix(&mut self, v: ::std::string::String) {
        self.http_suffix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_suffix(&mut self) -> &mut ::std::string::String {
        if self.http_suffix.is_none() {
            self.http_suffix.set_default();
        };
        self.http_suffix.as_mut().unwrap()
    }

    // Take field
    pub fn take_http_suffix(&mut self) -> ::std::string::String {
        self.http_suffix.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_http_suffix(&self) -> &str {
        match self.http_suffix.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_http_suffix_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.http_suffix
    }

    fn mut_http_suffix_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.http_suffix
    }
}

impl ::protobuf::Message for UpgradeRequiredMessage {
    fn is_initialized(&self) -> bool {
        if self.upgrade_signed_part.is_none() {
            return false;
        };
        if self.signature.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.upgrade_signed_part)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
                },
                30 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.http_suffix)?;
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
        if let Some(v) = self.upgrade_signed_part.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        if let Some(v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(20, &v);
        };
        if let Some(v) = self.http_suffix.as_ref() {
            my_size += ::protobuf::rt::string_size(30, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.upgrade_signed_part.as_ref() {
            os.write_bytes(10, &v)?;
        };
        if let Some(v) = self.signature.as_ref() {
            os.write_bytes(20, &v)?;
        };
        if let Some(v) = self.http_suffix.as_ref() {
            os.write_string(30, &v)?;
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

impl ::protobuf::MessageStatic for UpgradeRequiredMessage {
    fn new() -> UpgradeRequiredMessage {
        UpgradeRequiredMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpgradeRequiredMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "upgrade_signed_part",
                    UpgradeRequiredMessage::get_upgrade_signed_part_for_reflect,
                    UpgradeRequiredMessage::mut_upgrade_signed_part_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    UpgradeRequiredMessage::get_signature_for_reflect,
                    UpgradeRequiredMessage::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "http_suffix",
                    UpgradeRequiredMessage::get_http_suffix_for_reflect,
                    UpgradeRequiredMessage::mut_http_suffix_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpgradeRequiredMessage>(
                    "UpgradeRequiredMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpgradeRequiredMessage {
    fn clear(&mut self) {
        self.clear_upgrade_signed_part();
        self.clear_signature();
        self.clear_http_suffix();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpgradeRequiredMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpgradeRequiredMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct APLoginFailed {
    // message fields
    error_code: ::std::option::Option<ErrorCode>,
    retry_delay: ::std::option::Option<i32>,
    expiry: ::std::option::Option<i32>,
    error_description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for APLoginFailed {}

impl APLoginFailed {
    pub fn new() -> APLoginFailed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static APLoginFailed {
        static mut instance: ::protobuf::lazy::Lazy<APLoginFailed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const APLoginFailed,
        };
        unsafe {
            instance.get(APLoginFailed::new)
        }
    }

    // required .ErrorCode error_code = 10;

    pub fn clear_error_code(&mut self) {
        self.error_code = ::std::option::Option::None;
    }

    pub fn has_error_code(&self) -> bool {
        self.error_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_code(&mut self, v: ErrorCode) {
        self.error_code = ::std::option::Option::Some(v);
    }

    pub fn get_error_code(&self) -> ErrorCode {
        self.error_code.unwrap_or(ErrorCode::ProtocolError)
    }

    fn get_error_code_for_reflect(&self) -> &::std::option::Option<ErrorCode> {
        &self.error_code
    }

    fn mut_error_code_for_reflect(&mut self) -> &mut ::std::option::Option<ErrorCode> {
        &mut self.error_code
    }

    // optional int32 retry_delay = 20;

    pub fn clear_retry_delay(&mut self) {
        self.retry_delay = ::std::option::Option::None;
    }

    pub fn has_retry_delay(&self) -> bool {
        self.retry_delay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retry_delay(&mut self, v: i32) {
        self.retry_delay = ::std::option::Option::Some(v);
    }

    pub fn get_retry_delay(&self) -> i32 {
        self.retry_delay.unwrap_or(0)
    }

    fn get_retry_delay_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.retry_delay
    }

    fn mut_retry_delay_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.retry_delay
    }

    // optional int32 expiry = 30;

    pub fn clear_expiry(&mut self) {
        self.expiry = ::std::option::Option::None;
    }

    pub fn has_expiry(&self) -> bool {
        self.expiry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expiry(&mut self, v: i32) {
        self.expiry = ::std::option::Option::Some(v);
    }

    pub fn get_expiry(&self) -> i32 {
        self.expiry.unwrap_or(0)
    }

    fn get_expiry_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.expiry
    }

    fn mut_expiry_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.expiry
    }

    // optional string error_description = 40;

    pub fn clear_error_description(&mut self) {
        self.error_description.clear();
    }

    pub fn has_error_description(&self) -> bool {
        self.error_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_description(&mut self, v: ::std::string::String) {
        self.error_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_description(&mut self) -> &mut ::std::string::String {
        if self.error_description.is_none() {
            self.error_description.set_default();
        };
        self.error_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_description(&mut self) -> ::std::string::String {
        self.error_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_description(&self) -> &str {
        match self.error_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_description
    }

    fn mut_error_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_description
    }
}

impl ::protobuf::Message for APLoginFailed {
    fn is_initialized(&self) -> bool {
        if self.error_code.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.error_code = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.retry_delay = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.expiry = ::std::option::Option::Some(tmp);
                },
                40 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_description)?;
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
        if let Some(v) = self.error_code {
            my_size += ::protobuf::rt::enum_size(10, v);
        };
        if let Some(v) = self.retry_delay {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.expiry {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.error_description.as_ref() {
            my_size += ::protobuf::rt::string_size(40, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error_code {
            os.write_enum(10, v.value())?;
        };
        if let Some(v) = self.retry_delay {
            os.write_int32(20, v)?;
        };
        if let Some(v) = self.expiry {
            os.write_int32(30, v)?;
        };
        if let Some(v) = self.error_description.as_ref() {
            os.write_string(40, &v)?;
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

impl ::protobuf::MessageStatic for APLoginFailed {
    fn new() -> APLoginFailed {
        APLoginFailed::new()
    }

    fn descriptor_static(_: ::std::option::Option<APLoginFailed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ErrorCode>>(
                    "error_code",
                    APLoginFailed::get_error_code_for_reflect,
                    APLoginFailed::mut_error_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "retry_delay",
                    APLoginFailed::get_retry_delay_for_reflect,
                    APLoginFailed::mut_retry_delay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "expiry",
                    APLoginFailed::get_expiry_for_reflect,
                    APLoginFailed::mut_expiry_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_description",
                    APLoginFailed::get_error_description_for_reflect,
                    APLoginFailed::mut_error_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<APLoginFailed>(
                    "APLoginFailed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for APLoginFailed {
    fn clear(&mut self) {
        self.clear_error_code();
        self.clear_retry_delay();
        self.clear_expiry();
        self.clear_error_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for APLoginFailed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for APLoginFailed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientResponsePlaintext {
    // message fields
    login_crypto_response: ::protobuf::SingularPtrField<LoginCryptoResponseUnion>,
    pow_response: ::protobuf::SingularPtrField<PoWResponseUnion>,
    crypto_response: ::protobuf::SingularPtrField<CryptoResponseUnion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientResponsePlaintext {}

impl ClientResponsePlaintext {
    pub fn new() -> ClientResponsePlaintext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientResponsePlaintext {
        static mut instance: ::protobuf::lazy::Lazy<ClientResponsePlaintext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientResponsePlaintext,
        };
        unsafe {
            instance.get(ClientResponsePlaintext::new)
        }
    }

    // required .LoginCryptoResponseUnion login_crypto_response = 10;

    pub fn clear_login_crypto_response(&mut self) {
        self.login_crypto_response.clear();
    }

    pub fn has_login_crypto_response(&self) -> bool {
        self.login_crypto_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_login_crypto_response(&mut self, v: LoginCryptoResponseUnion) {
        self.login_crypto_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_login_crypto_response(&mut self) -> &mut LoginCryptoResponseUnion {
        if self.login_crypto_response.is_none() {
            self.login_crypto_response.set_default();
        };
        self.login_crypto_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_login_crypto_response(&mut self) -> LoginCryptoResponseUnion {
        self.login_crypto_response.take().unwrap_or_else(|| LoginCryptoResponseUnion::new())
    }

    pub fn get_login_crypto_response(&self) -> &LoginCryptoResponseUnion {
        self.login_crypto_response.as_ref().unwrap_or_else(|| LoginCryptoResponseUnion::default_instance())
    }

    fn get_login_crypto_response_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoResponseUnion> {
        &self.login_crypto_response
    }

    fn mut_login_crypto_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoResponseUnion> {
        &mut self.login_crypto_response
    }

    // required .PoWResponseUnion pow_response = 20;

    pub fn clear_pow_response(&mut self) {
        self.pow_response.clear();
    }

    pub fn has_pow_response(&self) -> bool {
        self.pow_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pow_response(&mut self, v: PoWResponseUnion) {
        self.pow_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pow_response(&mut self) -> &mut PoWResponseUnion {
        if self.pow_response.is_none() {
            self.pow_response.set_default();
        };
        self.pow_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_pow_response(&mut self) -> PoWResponseUnion {
        self.pow_response.take().unwrap_or_else(|| PoWResponseUnion::new())
    }

    pub fn get_pow_response(&self) -> &PoWResponseUnion {
        self.pow_response.as_ref().unwrap_or_else(|| PoWResponseUnion::default_instance())
    }

    fn get_pow_response_for_reflect(&self) -> &::protobuf::SingularPtrField<PoWResponseUnion> {
        &self.pow_response
    }

    fn mut_pow_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PoWResponseUnion> {
        &mut self.pow_response
    }

    // required .CryptoResponseUnion crypto_response = 30;

    pub fn clear_crypto_response(&mut self) {
        self.crypto_response.clear();
    }

    pub fn has_crypto_response(&self) -> bool {
        self.crypto_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crypto_response(&mut self, v: CryptoResponseUnion) {
        self.crypto_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_crypto_response(&mut self) -> &mut CryptoResponseUnion {
        if self.crypto_response.is_none() {
            self.crypto_response.set_default();
        };
        self.crypto_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_crypto_response(&mut self) -> CryptoResponseUnion {
        self.crypto_response.take().unwrap_or_else(|| CryptoResponseUnion::new())
    }

    pub fn get_crypto_response(&self) -> &CryptoResponseUnion {
        self.crypto_response.as_ref().unwrap_or_else(|| CryptoResponseUnion::default_instance())
    }

    fn get_crypto_response_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoResponseUnion> {
        &self.crypto_response
    }

    fn mut_crypto_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoResponseUnion> {
        &mut self.crypto_response
    }
}

impl ::protobuf::Message for ClientResponsePlaintext {
    fn is_initialized(&self) -> bool {
        if self.login_crypto_response.is_none() {
            return false;
        };
        if self.pow_response.is_none() {
            return false;
        };
        if self.crypto_response.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login_crypto_response)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pow_response)?;
                },
                30 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.crypto_response)?;
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
        if let Some(v) = self.login_crypto_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.pow_response.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.crypto_response.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.login_crypto_response.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.pow_response.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.crypto_response.as_ref() {
            os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ClientResponsePlaintext {
    fn new() -> ClientResponsePlaintext {
        ClientResponsePlaintext::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientResponsePlaintext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoResponseUnion>>(
                    "login_crypto_response",
                    ClientResponsePlaintext::get_login_crypto_response_for_reflect,
                    ClientResponsePlaintext::mut_login_crypto_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PoWResponseUnion>>(
                    "pow_response",
                    ClientResponsePlaintext::get_pow_response_for_reflect,
                    ClientResponsePlaintext::mut_pow_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoResponseUnion>>(
                    "crypto_response",
                    ClientResponsePlaintext::get_crypto_response_for_reflect,
                    ClientResponsePlaintext::mut_crypto_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientResponsePlaintext>(
                    "ClientResponsePlaintext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientResponsePlaintext {
    fn clear(&mut self) {
        self.clear_login_crypto_response();
        self.clear_pow_response();
        self.clear_crypto_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientResponsePlaintext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientResponsePlaintext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoResponseUnion {
    // message fields
    diffie_hellman: ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoResponseUnion {}

impl LoginCryptoResponseUnion {
    pub fn new() -> LoginCryptoResponseUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoResponseUnion {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoResponseUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoResponseUnion,
        };
        unsafe {
            instance.get(LoginCryptoResponseUnion::new)
        }
    }

    // optional .LoginCryptoDiffieHellmanResponse diffie_hellman = 10;

    pub fn clear_diffie_hellman(&mut self) {
        self.diffie_hellman.clear();
    }

    pub fn has_diffie_hellman(&self) -> bool {
        self.diffie_hellman.is_some()
    }

    // Param is passed by value, moved
    pub fn set_diffie_hellman(&mut self, v: LoginCryptoDiffieHellmanResponse) {
        self.diffie_hellman = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_diffie_hellman(&mut self) -> &mut LoginCryptoDiffieHellmanResponse {
        if self.diffie_hellman.is_none() {
            self.diffie_hellman.set_default();
        };
        self.diffie_hellman.as_mut().unwrap()
    }

    // Take field
    pub fn take_diffie_hellman(&mut self) -> LoginCryptoDiffieHellmanResponse {
        self.diffie_hellman.take().unwrap_or_else(|| LoginCryptoDiffieHellmanResponse::new())
    }

    pub fn get_diffie_hellman(&self) -> &LoginCryptoDiffieHellmanResponse {
        self.diffie_hellman.as_ref().unwrap_or_else(|| LoginCryptoDiffieHellmanResponse::default_instance())
    }

    fn get_diffie_hellman_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoDiffieHellmanResponse> {
        &self.diffie_hellman
    }

    fn mut_diffie_hellman_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanResponse> {
        &mut self.diffie_hellman
    }
}

impl ::protobuf::Message for LoginCryptoResponseUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.diffie_hellman)?;
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
        if let Some(v) = self.diffie_hellman.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.diffie_hellman.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LoginCryptoResponseUnion {
    fn new() -> LoginCryptoResponseUnion {
        LoginCryptoResponseUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoResponseUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoDiffieHellmanResponse>>(
                    "diffie_hellman",
                    LoginCryptoResponseUnion::get_diffie_hellman_for_reflect,
                    LoginCryptoResponseUnion::mut_diffie_hellman_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoResponseUnion>(
                    "LoginCryptoResponseUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoResponseUnion {
    fn clear(&mut self) {
        self.clear_diffie_hellman();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoResponseUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoResponseUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoDiffieHellmanResponse {
    // message fields
    hmac: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoDiffieHellmanResponse {}

impl LoginCryptoDiffieHellmanResponse {
    pub fn new() -> LoginCryptoDiffieHellmanResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoDiffieHellmanResponse {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoDiffieHellmanResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoDiffieHellmanResponse,
        };
        unsafe {
            instance.get(LoginCryptoDiffieHellmanResponse::new)
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

    fn get_hmac_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.hmac
    }

    fn mut_hmac_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.hmac
    }
}

impl ::protobuf::Message for LoginCryptoDiffieHellmanResponse {
    fn is_initialized(&self) -> bool {
        if self.hmac.is_none() {
            return false;
        };
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
        if let Some(v) = self.hmac.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hmac.as_ref() {
            os.write_bytes(10, &v)?;
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

impl ::protobuf::MessageStatic for LoginCryptoDiffieHellmanResponse {
    fn new() -> LoginCryptoDiffieHellmanResponse {
        LoginCryptoDiffieHellmanResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoDiffieHellmanResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hmac",
                    LoginCryptoDiffieHellmanResponse::get_hmac_for_reflect,
                    LoginCryptoDiffieHellmanResponse::mut_hmac_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoDiffieHellmanResponse>(
                    "LoginCryptoDiffieHellmanResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoDiffieHellmanResponse {
    fn clear(&mut self) {
        self.clear_hmac();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoDiffieHellmanResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoDiffieHellmanResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoWResponseUnion {
    // message fields
    hash_cash: ::protobuf::SingularPtrField<PoWHashCashResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoWResponseUnion {}

impl PoWResponseUnion {
    pub fn new() -> PoWResponseUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoWResponseUnion {
        static mut instance: ::protobuf::lazy::Lazy<PoWResponseUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoWResponseUnion,
        };
        unsafe {
            instance.get(PoWResponseUnion::new)
        }
    }

    // optional .PoWHashCashResponse hash_cash = 10;

    pub fn clear_hash_cash(&mut self) {
        self.hash_cash.clear();
    }

    pub fn has_hash_cash(&self) -> bool {
        self.hash_cash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash_cash(&mut self, v: PoWHashCashResponse) {
        self.hash_cash = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash_cash(&mut self) -> &mut PoWHashCashResponse {
        if self.hash_cash.is_none() {
            self.hash_cash.set_default();
        };
        self.hash_cash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash_cash(&mut self) -> PoWHashCashResponse {
        self.hash_cash.take().unwrap_or_else(|| PoWHashCashResponse::new())
    }

    pub fn get_hash_cash(&self) -> &PoWHashCashResponse {
        self.hash_cash.as_ref().unwrap_or_else(|| PoWHashCashResponse::default_instance())
    }

    fn get_hash_cash_for_reflect(&self) -> &::protobuf::SingularPtrField<PoWHashCashResponse> {
        &self.hash_cash
    }

    fn mut_hash_cash_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PoWHashCashResponse> {
        &mut self.hash_cash
    }
}

impl ::protobuf::Message for PoWResponseUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hash_cash)?;
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
        if let Some(v) = self.hash_cash.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash_cash.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for PoWResponseUnion {
    fn new() -> PoWResponseUnion {
        PoWResponseUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoWResponseUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PoWHashCashResponse>>(
                    "hash_cash",
                    PoWResponseUnion::get_hash_cash_for_reflect,
                    PoWResponseUnion::mut_hash_cash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoWResponseUnion>(
                    "PoWResponseUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoWResponseUnion {
    fn clear(&mut self) {
        self.clear_hash_cash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoWResponseUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoWResponseUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoWHashCashResponse {
    // message fields
    hash_suffix: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoWHashCashResponse {}

impl PoWHashCashResponse {
    pub fn new() -> PoWHashCashResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoWHashCashResponse {
        static mut instance: ::protobuf::lazy::Lazy<PoWHashCashResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoWHashCashResponse,
        };
        unsafe {
            instance.get(PoWHashCashResponse::new)
        }
    }

    // required bytes hash_suffix = 10;

    pub fn clear_hash_suffix(&mut self) {
        self.hash_suffix.clear();
    }

    pub fn has_hash_suffix(&self) -> bool {
        self.hash_suffix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash_suffix(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash_suffix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash_suffix(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hash_suffix.is_none() {
            self.hash_suffix.set_default();
        };
        self.hash_suffix.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash_suffix(&mut self) -> ::std::vec::Vec<u8> {
        self.hash_suffix.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hash_suffix(&self) -> &[u8] {
        match self.hash_suffix.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_hash_suffix_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.hash_suffix
    }

    fn mut_hash_suffix_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.hash_suffix
    }
}

impl ::protobuf::Message for PoWHashCashResponse {
    fn is_initialized(&self) -> bool {
        if self.hash_suffix.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hash_suffix)?;
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
        if let Some(v) = self.hash_suffix.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash_suffix.as_ref() {
            os.write_bytes(10, &v)?;
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

impl ::protobuf::MessageStatic for PoWHashCashResponse {
    fn new() -> PoWHashCashResponse {
        PoWHashCashResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoWHashCashResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash_suffix",
                    PoWHashCashResponse::get_hash_suffix_for_reflect,
                    PoWHashCashResponse::mut_hash_suffix_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoWHashCashResponse>(
                    "PoWHashCashResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoWHashCashResponse {
    fn clear(&mut self) {
        self.clear_hash_suffix();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoWHashCashResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoWHashCashResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoResponseUnion {
    // message fields
    shannon: ::protobuf::SingularPtrField<CryptoShannonResponse>,
    rc4_sha1_hmac: ::protobuf::SingularPtrField<CryptoRc4Sha1HmacResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoResponseUnion {}

impl CryptoResponseUnion {
    pub fn new() -> CryptoResponseUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoResponseUnion {
        static mut instance: ::protobuf::lazy::Lazy<CryptoResponseUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoResponseUnion,
        };
        unsafe {
            instance.get(CryptoResponseUnion::new)
        }
    }

    // optional .CryptoShannonResponse shannon = 10;

    pub fn clear_shannon(&mut self) {
        self.shannon.clear();
    }

    pub fn has_shannon(&self) -> bool {
        self.shannon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shannon(&mut self, v: CryptoShannonResponse) {
        self.shannon = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shannon(&mut self) -> &mut CryptoShannonResponse {
        if self.shannon.is_none() {
            self.shannon.set_default();
        };
        self.shannon.as_mut().unwrap()
    }

    // Take field
    pub fn take_shannon(&mut self) -> CryptoShannonResponse {
        self.shannon.take().unwrap_or_else(|| CryptoShannonResponse::new())
    }

    pub fn get_shannon(&self) -> &CryptoShannonResponse {
        self.shannon.as_ref().unwrap_or_else(|| CryptoShannonResponse::default_instance())
    }

    fn get_shannon_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoShannonResponse> {
        &self.shannon
    }

    fn mut_shannon_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoShannonResponse> {
        &mut self.shannon
    }

    // optional .CryptoRc4Sha1HmacResponse rc4_sha1_hmac = 20;

    pub fn clear_rc4_sha1_hmac(&mut self) {
        self.rc4_sha1_hmac.clear();
    }

    pub fn has_rc4_sha1_hmac(&self) -> bool {
        self.rc4_sha1_hmac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rc4_sha1_hmac(&mut self, v: CryptoRc4Sha1HmacResponse) {
        self.rc4_sha1_hmac = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rc4_sha1_hmac(&mut self) -> &mut CryptoRc4Sha1HmacResponse {
        if self.rc4_sha1_hmac.is_none() {
            self.rc4_sha1_hmac.set_default();
        };
        self.rc4_sha1_hmac.as_mut().unwrap()
    }

    // Take field
    pub fn take_rc4_sha1_hmac(&mut self) -> CryptoRc4Sha1HmacResponse {
        self.rc4_sha1_hmac.take().unwrap_or_else(|| CryptoRc4Sha1HmacResponse::new())
    }

    pub fn get_rc4_sha1_hmac(&self) -> &CryptoRc4Sha1HmacResponse {
        self.rc4_sha1_hmac.as_ref().unwrap_or_else(|| CryptoRc4Sha1HmacResponse::default_instance())
    }

    fn get_rc4_sha1_hmac_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoRc4Sha1HmacResponse> {
        &self.rc4_sha1_hmac
    }

    fn mut_rc4_sha1_hmac_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoRc4Sha1HmacResponse> {
        &mut self.rc4_sha1_hmac
    }
}

impl ::protobuf::Message for CryptoResponseUnion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shannon)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rc4_sha1_hmac)?;
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
        if let Some(v) = self.shannon.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.rc4_sha1_hmac.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.shannon.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.rc4_sha1_hmac.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CryptoResponseUnion {
    fn new() -> CryptoResponseUnion {
        CryptoResponseUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoResponseUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoShannonResponse>>(
                    "shannon",
                    CryptoResponseUnion::get_shannon_for_reflect,
                    CryptoResponseUnion::mut_shannon_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoRc4Sha1HmacResponse>>(
                    "rc4_sha1_hmac",
                    CryptoResponseUnion::get_rc4_sha1_hmac_for_reflect,
                    CryptoResponseUnion::mut_rc4_sha1_hmac_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CryptoResponseUnion>(
                    "CryptoResponseUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoResponseUnion {
    fn clear(&mut self) {
        self.clear_shannon();
        self.clear_rc4_sha1_hmac();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoResponseUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoResponseUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoShannonResponse {
    // message fields
    dummy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoShannonResponse {}

impl CryptoShannonResponse {
    pub fn new() -> CryptoShannonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoShannonResponse {
        static mut instance: ::protobuf::lazy::Lazy<CryptoShannonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoShannonResponse,
        };
        unsafe {
            instance.get(CryptoShannonResponse::new)
        }
    }

    // optional int32 dummy = 1;

    pub fn clear_dummy(&mut self) {
        self.dummy = ::std::option::Option::None;
    }

    pub fn has_dummy(&self) -> bool {
        self.dummy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy(&mut self, v: i32) {
        self.dummy = ::std::option::Option::Some(v);
    }

    pub fn get_dummy(&self) -> i32 {
        self.dummy.unwrap_or(0)
    }

    fn get_dummy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dummy
    }

    fn mut_dummy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dummy
    }
}

impl ::protobuf::Message for CryptoShannonResponse {
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
                    self.dummy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dummy {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dummy {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for CryptoShannonResponse {
    fn new() -> CryptoShannonResponse {
        CryptoShannonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoShannonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dummy",
                    CryptoShannonResponse::get_dummy_for_reflect,
                    CryptoShannonResponse::mut_dummy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CryptoShannonResponse>(
                    "CryptoShannonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoShannonResponse {
    fn clear(&mut self) {
        self.clear_dummy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoShannonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoShannonResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoRc4Sha1HmacResponse {
    // message fields
    dummy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoRc4Sha1HmacResponse {}

impl CryptoRc4Sha1HmacResponse {
    pub fn new() -> CryptoRc4Sha1HmacResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoRc4Sha1HmacResponse {
        static mut instance: ::protobuf::lazy::Lazy<CryptoRc4Sha1HmacResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoRc4Sha1HmacResponse,
        };
        unsafe {
            instance.get(CryptoRc4Sha1HmacResponse::new)
        }
    }

    // optional int32 dummy = 1;

    pub fn clear_dummy(&mut self) {
        self.dummy = ::std::option::Option::None;
    }

    pub fn has_dummy(&self) -> bool {
        self.dummy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy(&mut self, v: i32) {
        self.dummy = ::std::option::Option::Some(v);
    }

    pub fn get_dummy(&self) -> i32 {
        self.dummy.unwrap_or(0)
    }

    fn get_dummy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dummy
    }

    fn mut_dummy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dummy
    }
}

impl ::protobuf::Message for CryptoRc4Sha1HmacResponse {
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
                    self.dummy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dummy {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dummy {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for CryptoRc4Sha1HmacResponse {
    fn new() -> CryptoRc4Sha1HmacResponse {
        CryptoRc4Sha1HmacResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoRc4Sha1HmacResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dummy",
                    CryptoRc4Sha1HmacResponse::get_dummy_for_reflect,
                    CryptoRc4Sha1HmacResponse::mut_dummy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CryptoRc4Sha1HmacResponse>(
                    "CryptoRc4Sha1HmacResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoRc4Sha1HmacResponse {
    fn clear(&mut self) {
        self.clear_dummy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoRc4Sha1HmacResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoRc4Sha1HmacResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Product {
    PRODUCT_CLIENT = 0,
    PRODUCT_LIBSPOTIFY = 1,
    PRODUCT_MOBILE = 2,
    PRODUCT_PARTNER = 3,
    PRODUCT_LIBSPOTIFY_EMBEDDED = 5,
}

impl ::protobuf::ProtobufEnum for Product {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Product> {
        match value {
            0 => ::std::option::Option::Some(Product::PRODUCT_CLIENT),
            1 => ::std::option::Option::Some(Product::PRODUCT_LIBSPOTIFY),
            2 => ::std::option::Option::Some(Product::PRODUCT_MOBILE),
            3 => ::std::option::Option::Some(Product::PRODUCT_PARTNER),
            5 => ::std::option::Option::Some(Product::PRODUCT_LIBSPOTIFY_EMBEDDED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Product] = &[
            Product::PRODUCT_CLIENT,
            Product::PRODUCT_LIBSPOTIFY,
            Product::PRODUCT_MOBILE,
            Product::PRODUCT_PARTNER,
            Product::PRODUCT_LIBSPOTIFY_EMBEDDED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Product>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Product", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Product {
}

impl ::protobuf::reflect::ProtobufValue for Product {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ProductFlags {
    PRODUCT_FLAG_NONE = 0,
    PRODUCT_FLAG_DEV_BUILD = 1,
}

impl ::protobuf::ProtobufEnum for ProductFlags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ProductFlags> {
        match value {
            0 => ::std::option::Option::Some(ProductFlags::PRODUCT_FLAG_NONE),
            1 => ::std::option::Option::Some(ProductFlags::PRODUCT_FLAG_DEV_BUILD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ProductFlags] = &[
            ProductFlags::PRODUCT_FLAG_NONE,
            ProductFlags::PRODUCT_FLAG_DEV_BUILD,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ProductFlags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ProductFlags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ProductFlags {
}

impl ::protobuf::reflect::ProtobufValue for ProductFlags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Platform {
    PLATFORM_WIN32_X86 = 0,
    PLATFORM_OSX_X86 = 1,
    PLATFORM_LINUX_X86 = 2,
    PLATFORM_IPHONE_ARM = 3,
    PLATFORM_S60_ARM = 4,
    PLATFORM_OSX_PPC = 5,
    PLATFORM_ANDROID_ARM = 6,
    PLATFORM_WINDOWS_CE_ARM = 7,
    PLATFORM_LINUX_X86_64 = 8,
    PLATFORM_OSX_X86_64 = 9,
    PLATFORM_PALM_ARM = 10,
    PLATFORM_LINUX_SH = 11,
    PLATFORM_FREEBSD_X86 = 12,
    PLATFORM_FREEBSD_X86_64 = 13,
    PLATFORM_BLACKBERRY_ARM = 14,
    PLATFORM_SONOS = 15,
    PLATFORM_LINUX_MIPS = 16,
    PLATFORM_LINUX_ARM = 17,
    PLATFORM_LOGITECH_ARM = 18,
    PLATFORM_LINUX_BLACKFIN = 19,
    PLATFORM_WP7_ARM = 20,
    PLATFORM_ONKYO_ARM = 21,
    PLATFORM_QNXNTO_ARM = 22,
    PLATFORM_BCO_ARM = 23,
}

impl ::protobuf::ProtobufEnum for Platform {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Platform> {
        match value {
            0 => ::std::option::Option::Some(Platform::PLATFORM_WIN32_X86),
            1 => ::std::option::Option::Some(Platform::PLATFORM_OSX_X86),
            2 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_X86),
            3 => ::std::option::Option::Some(Platform::PLATFORM_IPHONE_ARM),
            4 => ::std::option::Option::Some(Platform::PLATFORM_S60_ARM),
            5 => ::std::option::Option::Some(Platform::PLATFORM_OSX_PPC),
            6 => ::std::option::Option::Some(Platform::PLATFORM_ANDROID_ARM),
            7 => ::std::option::Option::Some(Platform::PLATFORM_WINDOWS_CE_ARM),
            8 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_X86_64),
            9 => ::std::option::Option::Some(Platform::PLATFORM_OSX_X86_64),
            10 => ::std::option::Option::Some(Platform::PLATFORM_PALM_ARM),
            11 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_SH),
            12 => ::std::option::Option::Some(Platform::PLATFORM_FREEBSD_X86),
            13 => ::std::option::Option::Some(Platform::PLATFORM_FREEBSD_X86_64),
            14 => ::std::option::Option::Some(Platform::PLATFORM_BLACKBERRY_ARM),
            15 => ::std::option::Option::Some(Platform::PLATFORM_SONOS),
            16 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_MIPS),
            17 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_ARM),
            18 => ::std::option::Option::Some(Platform::PLATFORM_LOGITECH_ARM),
            19 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_BLACKFIN),
            20 => ::std::option::Option::Some(Platform::PLATFORM_WP7_ARM),
            21 => ::std::option::Option::Some(Platform::PLATFORM_ONKYO_ARM),
            22 => ::std::option::Option::Some(Platform::PLATFORM_QNXNTO_ARM),
            23 => ::std::option::Option::Some(Platform::PLATFORM_BCO_ARM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Platform] = &[
            Platform::PLATFORM_WIN32_X86,
            Platform::PLATFORM_OSX_X86,
            Platform::PLATFORM_LINUX_X86,
            Platform::PLATFORM_IPHONE_ARM,
            Platform::PLATFORM_S60_ARM,
            Platform::PLATFORM_OSX_PPC,
            Platform::PLATFORM_ANDROID_ARM,
            Platform::PLATFORM_WINDOWS_CE_ARM,
            Platform::PLATFORM_LINUX_X86_64,
            Platform::PLATFORM_OSX_X86_64,
            Platform::PLATFORM_PALM_ARM,
            Platform::PLATFORM_LINUX_SH,
            Platform::PLATFORM_FREEBSD_X86,
            Platform::PLATFORM_FREEBSD_X86_64,
            Platform::PLATFORM_BLACKBERRY_ARM,
            Platform::PLATFORM_SONOS,
            Platform::PLATFORM_LINUX_MIPS,
            Platform::PLATFORM_LINUX_ARM,
            Platform::PLATFORM_LOGITECH_ARM,
            Platform::PLATFORM_LINUX_BLACKFIN,
            Platform::PLATFORM_WP7_ARM,
            Platform::PLATFORM_ONKYO_ARM,
            Platform::PLATFORM_QNXNTO_ARM,
            Platform::PLATFORM_BCO_ARM,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Platform>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Platform", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Platform {
}

impl ::protobuf::reflect::ProtobufValue for Platform {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Fingerprint {
    FINGERPRINT_GRAIN = 0,
    FINGERPRINT_HMAC_RIPEMD = 1,
}

impl ::protobuf::ProtobufEnum for Fingerprint {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Fingerprint> {
        match value {
            0 => ::std::option::Option::Some(Fingerprint::FINGERPRINT_GRAIN),
            1 => ::std::option::Option::Some(Fingerprint::FINGERPRINT_HMAC_RIPEMD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Fingerprint] = &[
            Fingerprint::FINGERPRINT_GRAIN,
            Fingerprint::FINGERPRINT_HMAC_RIPEMD,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Fingerprint>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Fingerprint", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Fingerprint {
}

impl ::protobuf::reflect::ProtobufValue for Fingerprint {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Cryptosuite {
    CRYPTO_SUITE_SHANNON = 0,
    CRYPTO_SUITE_RC4_SHA1_HMAC = 1,
}

impl ::protobuf::ProtobufEnum for Cryptosuite {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Cryptosuite> {
        match value {
            0 => ::std::option::Option::Some(Cryptosuite::CRYPTO_SUITE_SHANNON),
            1 => ::std::option::Option::Some(Cryptosuite::CRYPTO_SUITE_RC4_SHA1_HMAC),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Cryptosuite] = &[
            Cryptosuite::CRYPTO_SUITE_SHANNON,
            Cryptosuite::CRYPTO_SUITE_RC4_SHA1_HMAC,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Cryptosuite>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Cryptosuite", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Cryptosuite {
}

impl ::protobuf::reflect::ProtobufValue for Cryptosuite {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Powscheme {
    POW_HASH_CASH = 0,
}

impl ::protobuf::ProtobufEnum for Powscheme {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Powscheme> {
        match value {
            0 => ::std::option::Option::Some(Powscheme::POW_HASH_CASH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Powscheme] = &[
            Powscheme::POW_HASH_CASH,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Powscheme>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Powscheme", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Powscheme {
}

impl ::protobuf::reflect::ProtobufValue for Powscheme {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorCode {
    ProtocolError = 0,
    TryAnotherAP = 2,
    BadConnectionId = 5,
    TravelRestriction = 9,
    PremiumAccountRequired = 11,
    BadCredentials = 12,
    CouldNotValidateCredentials = 13,
    AccountExists = 14,
    ExtraVerificationRequired = 15,
    InvalidAppKey = 16,
    ApplicationBanned = 17,
}

impl ::protobuf::ProtobufEnum for ErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorCode> {
        match value {
            0 => ::std::option::Option::Some(ErrorCode::ProtocolError),
            2 => ::std::option::Option::Some(ErrorCode::TryAnotherAP),
            5 => ::std::option::Option::Some(ErrorCode::BadConnectionId),
            9 => ::std::option::Option::Some(ErrorCode::TravelRestriction),
            11 => ::std::option::Option::Some(ErrorCode::PremiumAccountRequired),
            12 => ::std::option::Option::Some(ErrorCode::BadCredentials),
            13 => ::std::option::Option::Some(ErrorCode::CouldNotValidateCredentials),
            14 => ::std::option::Option::Some(ErrorCode::AccountExists),
            15 => ::std::option::Option::Some(ErrorCode::ExtraVerificationRequired),
            16 => ::std::option::Option::Some(ErrorCode::InvalidAppKey),
            17 => ::std::option::Option::Some(ErrorCode::ApplicationBanned),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorCode] = &[
            ErrorCode::ProtocolError,
            ErrorCode::TryAnotherAP,
            ErrorCode::BadConnectionId,
            ErrorCode::TravelRestriction,
            ErrorCode::PremiumAccountRequired,
            ErrorCode::BadCredentials,
            ErrorCode::CouldNotValidateCredentials,
            ErrorCode::AccountExists,
            ErrorCode::ExtraVerificationRequired,
            ErrorCode::InvalidAppKey,
            ErrorCode::ApplicationBanned,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ErrorCode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ErrorCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ErrorCode {
}

impl ::protobuf::reflect::ProtobufValue for ErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x6b, 0x65, 0x79, 0x65, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0xb2, 0x03, 0x0a, 0x0b, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x48, 0x65,
    0x6c, 0x6c, 0x6f, 0x12, 0x29, 0x0a, 0x0a, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x6e, 0x66,
    0x6f, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0a, 0x2e, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x49,
    0x6e, 0x66, 0x6f, 0x52, 0x09, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x43,
    0x0a, 0x16, 0x66, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x73, 0x5f, 0x73,
    0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x18, 0x14, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x0c,
    0x2e, 0x46, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x52, 0x15, 0x66, 0x69,
    0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x73, 0x53, 0x75, 0x70, 0x70, 0x6f, 0x72,
    0x74, 0x65, 0x64, 0x12, 0x43, 0x0a, 0x16, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x73, 0x75, 0x69,
    0x74, 0x65, 0x73, 0x5f, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x18, 0x1e, 0x20,
    0x03, 0x28, 0x0e, 0x32, 0x0c, 0x2e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x73, 0x75, 0x69, 0x74,
    0x65, 0x52, 0x15, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x73, 0x75, 0x69, 0x74, 0x65, 0x73, 0x53,
    0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x12, 0x3d, 0x0a, 0x14, 0x70, 0x6f, 0x77, 0x73,
    0x63, 0x68, 0x65, 0x6d, 0x65, 0x73, 0x5f, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64,
    0x18, 0x28, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x0a, 0x2e, 0x50, 0x6f, 0x77, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x65, 0x52, 0x13, 0x70, 0x6f, 0x77, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x73, 0x53, 0x75,
    0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x12, 0x44, 0x0a, 0x12, 0x6c, 0x6f, 0x67, 0x69, 0x6e,
    0x5f, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x5f, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x18, 0x32, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74,
    0x6f, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x52, 0x10, 0x6c, 0x6f, 0x67,
    0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x12, 0x21, 0x0a,
    0x0c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x18, 0x3c, 0x20,
    0x02, 0x28, 0x0c, 0x52, 0x0b, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4e, 0x6f, 0x6e, 0x63, 0x65,
    0x12, 0x18, 0x0a, 0x07, 0x70, 0x61, 0x64, 0x64, 0x69, 0x6e, 0x67, 0x18, 0x46, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x07, 0x70, 0x61, 0x64, 0x64, 0x69, 0x6e, 0x67, 0x12, 0x2c, 0x0a, 0x0b, 0x66, 0x65,
    0x61, 0x74, 0x75, 0x72, 0x65, 0x5f, 0x73, 0x65, 0x74, 0x18, 0x50, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0b, 0x2e, 0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x53, 0x65, 0x74, 0x52, 0x0a, 0x66, 0x65,
    0x61, 0x74, 0x75, 0x72, 0x65, 0x53, 0x65, 0x74, 0x22, 0xa4, 0x01, 0x0a, 0x09, 0x42, 0x75, 0x69,
    0x6c, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x22, 0x0a, 0x07, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63,
    0x74, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x08, 0x2e, 0x50, 0x72, 0x6f, 0x64, 0x75, 0x63,
    0x74, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x12, 0x32, 0x0a, 0x0d, 0x70, 0x72,
    0x6f, 0x64, 0x75, 0x63, 0x74, 0x5f, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x14, 0x20, 0x03, 0x28,
    0x0e, 0x32, 0x0d, 0x2e, 0x50, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x46, 0x6c, 0x61, 0x67, 0x73,
    0x52, 0x0c, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x46, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x25,
    0x0a, 0x08, 0x70, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x18, 0x1e, 0x20, 0x02, 0x28, 0x0e,
    0x32, 0x09, 0x2e, 0x50, 0x6c, 0x61, 0x74, 0x66, 0x6f, 0x72, 0x6d, 0x52, 0x08, 0x70, 0x6c, 0x61,
    0x74, 0x66, 0x6f, 0x72, 0x6d, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x28, 0x20, 0x02, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22,
    0x5e, 0x0a, 0x15, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x48, 0x65,
    0x6c, 0x6c, 0x6f, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x45, 0x0a, 0x0e, 0x64, 0x69, 0x66, 0x66,
    0x69, 0x65, 0x5f, 0x68, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1e, 0x2e, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x44, 0x69,
    0x66, 0x66, 0x69, 0x65, 0x48, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x48, 0x65, 0x6c, 0x6c, 0x6f,
    0x52, 0x0d, 0x64, 0x69, 0x66, 0x66, 0x69, 0x65, 0x48, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x22,
    0x5b, 0x0a, 0x1d, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x44, 0x69,
    0x66, 0x66, 0x69, 0x65, 0x48, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x48, 0x65, 0x6c, 0x6c, 0x6f,
    0x12, 0x0e, 0x0a, 0x02, 0x67, 0x63, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x02, 0x67, 0x63,
    0x12, 0x2a, 0x0a, 0x11, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x6b, 0x65, 0x79, 0x73, 0x5f,
    0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x18, 0x14, 0x20, 0x02, 0x28, 0x0d, 0x52, 0x0f, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x4b, 0x65, 0x79, 0x73, 0x4b, 0x6e, 0x6f, 0x77, 0x6e, 0x22, 0x59, 0x0a, 0x0a,
    0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x53, 0x65, 0x74, 0x12, 0x20, 0x0a, 0x0b, 0x61, 0x75,
    0x74, 0x6f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x32, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x0b, 0x61, 0x75, 0x74, 0x6f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x32, 0x12, 0x29, 0x0a, 0x10,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x4c,
    0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0xa5, 0x01, 0x0a, 0x11, 0x41, 0x50, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2a, 0x0a,
    0x09, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0c, 0x2e, 0x41, 0x50, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52, 0x09,
    0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12, 0x31, 0x0a, 0x07, 0x75, 0x70, 0x67,
    0x72, 0x61, 0x64, 0x65, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x55, 0x70, 0x67,
    0x72, 0x61, 0x64, 0x65, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x52, 0x07, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x12, 0x31, 0x0a, 0x0c,
    0x6c, 0x6f, 0x67, 0x69, 0x6e, 0x5f, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x18, 0x1e, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x41, 0x50, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x46, 0x61, 0x69, 0x6c,
    0x65, 0x64, 0x52, 0x0b, 0x6c, 0x6f, 0x67, 0x69, 0x6e, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x22,
    0xe8, 0x02, 0x0a, 0x0b, 0x41, 0x50, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12,
    0x50, 0x0a, 0x16, 0x6c, 0x6f, 0x67, 0x69, 0x6e, 0x5f, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x5f,
    0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x43, 0x68, 0x61,
    0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x52, 0x14, 0x6c, 0x6f, 0x67,
    0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67,
    0x65, 0x12, 0x4f, 0x0a, 0x15, 0x66, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74,
    0x5f, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18, 0x14, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x1a, 0x2e, 0x46, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x43, 0x68,
    0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x52, 0x14, 0x66, 0x69,
    0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e,
    0x67, 0x65, 0x12, 0x37, 0x0a, 0x0d, 0x70, 0x6f, 0x77, 0x5f, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65,
    0x6e, 0x67, 0x65, 0x18, 0x1e, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x50, 0x6f, 0x57, 0x43,
    0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x52, 0x0c, 0x70,
    0x6f, 0x77, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12, 0x40, 0x0a, 0x10, 0x63,
    0x72, 0x79, 0x70, 0x74, 0x6f, 0x5f, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18,
    0x28, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x43, 0x68,
    0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x52, 0x0f, 0x63, 0x72,
    0x79, 0x70, 0x74, 0x6f, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12, 0x21, 0x0a,
    0x0c, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x18, 0x32, 0x20,
    0x02, 0x28, 0x0c, 0x52, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4e, 0x6f, 0x6e, 0x63, 0x65,
    0x12, 0x18, 0x0a, 0x07, 0x70, 0x61, 0x64, 0x64, 0x69, 0x6e, 0x67, 0x18, 0x3c, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x07, 0x70, 0x61, 0x64, 0x64, 0x69, 0x6e, 0x67, 0x22, 0x66, 0x0a, 0x19, 0x4c, 0x6f,
    0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e,
    0x67, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x49, 0x0a, 0x0e, 0x64, 0x69, 0x66, 0x66, 0x69,
    0x65, 0x5f, 0x68, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x22, 0x2e, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x44, 0x69, 0x66,
    0x66, 0x69, 0x65, 0x48, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65,
    0x6e, 0x67, 0x65, 0x52, 0x0d, 0x64, 0x69, 0x66, 0x66, 0x69, 0x65, 0x48, 0x65, 0x6c, 0x6c, 0x6d,
    0x61, 0x6e, 0x22, 0x88, 0x01, 0x0a, 0x21, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70,
    0x74, 0x6f, 0x44, 0x69, 0x66, 0x66, 0x69, 0x65, 0x48, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x43,
    0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x67, 0x73, 0x18, 0x0a,
    0x20, 0x02, 0x28, 0x0c, 0x52, 0x02, 0x67, 0x73, 0x12, 0x30, 0x0a, 0x14, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x5f, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x5f, 0x6b, 0x65, 0x79,
    0x18, 0x14, 0x20, 0x02, 0x28, 0x05, 0x52, 0x12, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x69,
    0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x4b, 0x65, 0x79, 0x12, 0x21, 0x0a, 0x0c, 0x67, 0x73,
    0x5f, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x1e, 0x20, 0x02, 0x28, 0x0c,
    0x52, 0x0b, 0x67, 0x73, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x22, 0x8f, 0x01,
    0x0a, 0x19, 0x46, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x43, 0x68, 0x61,
    0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x30, 0x0a, 0x05, 0x67,
    0x72, 0x61, 0x69, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x46, 0x69, 0x6e,
    0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x47, 0x72, 0x61, 0x69, 0x6e, 0x43, 0x68, 0x61,
    0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52, 0x05, 0x67, 0x72, 0x61, 0x69, 0x6e, 0x12, 0x40, 0x0a,
    0x0b, 0x68, 0x6d, 0x61, 0x63, 0x5f, 0x72, 0x69, 0x70, 0x65, 0x6d, 0x64, 0x18, 0x14, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x46, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74,
    0x48, 0x6d, 0x61, 0x63, 0x52, 0x69, 0x70, 0x65, 0x6d, 0x64, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65,
    0x6e, 0x67, 0x65, 0x52, 0x0a, 0x68, 0x6d, 0x61, 0x63, 0x52, 0x69, 0x70, 0x65, 0x6d, 0x64, 0x22,
    0x2d, 0x0a, 0x19, 0x46, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x47, 0x72,
    0x61, 0x69, 0x6e, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12, 0x10, 0x0a, 0x03,
    0x6b, 0x65, 0x6b, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x6b, 0x22, 0x3e,
    0x0a, 0x1e, 0x46, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70, 0x72, 0x69, 0x6e, 0x74, 0x48, 0x6d, 0x61,
    0x63, 0x52, 0x69, 0x70, 0x65, 0x6d, 0x64, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65,
    0x12, 0x1c, 0x0a, 0x09, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18, 0x0a, 0x20,
    0x02, 0x28, 0x0c, 0x52, 0x09, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x22, 0x47,
    0x0a, 0x11, 0x50, 0x6f, 0x57, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x55, 0x6e,
    0x69, 0x6f, 0x6e, 0x12, 0x32, 0x0a, 0x09, 0x68, 0x61, 0x73, 0x68, 0x5f, 0x63, 0x61, 0x73, 0x68,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x50, 0x6f, 0x57, 0x48, 0x61, 0x73, 0x68,
    0x43, 0x61, 0x73, 0x68, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52, 0x08, 0x68,
    0x61, 0x73, 0x68, 0x43, 0x61, 0x73, 0x68, 0x22, 0x5e, 0x0a, 0x14, 0x50, 0x6f, 0x57, 0x48, 0x61,
    0x73, 0x68, 0x43, 0x61, 0x73, 0x68, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12,
    0x16, 0x0a, 0x06, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x06, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74,
    0x68, 0x18, 0x14, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x12,
    0x16, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x1e, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x22, 0x8a, 0x01, 0x0a, 0x14, 0x43, 0x72, 0x79, 0x70,
    0x74, 0x6f, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e,
    0x12, 0x31, 0x0a, 0x07, 0x73, 0x68, 0x61, 0x6e, 0x6e, 0x6f, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x17, 0x2e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x53, 0x68, 0x61, 0x6e, 0x6e, 0x6f,
    0x6e, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52, 0x07, 0x73, 0x68, 0x61, 0x6e,
    0x6e, 0x6f, 0x6e, 0x12, 0x3f, 0x0a, 0x0d, 0x72, 0x63, 0x34, 0x5f, 0x73, 0x68, 0x61, 0x31, 0x5f,
    0x68, 0x6d, 0x61, 0x63, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x43, 0x72, 0x79,
    0x70, 0x74, 0x6f, 0x52, 0x63, 0x34, 0x53, 0x68, 0x61, 0x31, 0x48, 0x6d, 0x61, 0x63, 0x43, 0x68,
    0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52, 0x0b, 0x72, 0x63, 0x34, 0x53, 0x68, 0x61, 0x31,
    0x48, 0x6d, 0x61, 0x63, 0x22, 0x18, 0x0a, 0x16, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x53, 0x68,
    0x61, 0x6e, 0x6e, 0x6f, 0x6e, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x22, 0x1c,
    0x0a, 0x1a, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x52, 0x63, 0x34, 0x53, 0x68, 0x61, 0x31, 0x48,
    0x6d, 0x61, 0x63, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x22, 0x87, 0x01, 0x0a,
    0x16, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2e, 0x0a, 0x13, 0x75, 0x70, 0x67, 0x72, 0x61,
    0x64, 0x65, 0x5f, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x18, 0x0a,
    0x20, 0x02, 0x28, 0x0c, 0x52, 0x11, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x53, 0x69, 0x67,
    0x6e, 0x65, 0x64, 0x50, 0x61, 0x72, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61,
    0x74, 0x75, 0x72, 0x65, 0x18, 0x14, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x09, 0x73, 0x69, 0x67, 0x6e,
    0x61, 0x74, 0x75, 0x72, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x73, 0x75,
    0x66, 0x66, 0x69, 0x78, 0x18, 0x1e, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x68, 0x74, 0x74, 0x70,
    0x53, 0x75, 0x66, 0x66, 0x69, 0x78, 0x22, 0xa0, 0x01, 0x0a, 0x0d, 0x41, 0x50, 0x4c, 0x6f, 0x67,
    0x69, 0x6e, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x12, 0x29, 0x0a, 0x0a, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x0a, 0x2e, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x43, 0x6f, 0x64, 0x65, 0x52, 0x09, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x43,
    0x6f, 0x64, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x65, 0x74, 0x72, 0x79, 0x5f, 0x64, 0x65, 0x6c,
    0x61, 0x79, 0x18, 0x14, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x72, 0x65, 0x74, 0x72, 0x79, 0x44,
    0x65, 0x6c, 0x61, 0x79, 0x12, 0x16, 0x0a, 0x06, 0x65, 0x78, 0x70, 0x69, 0x72, 0x79, 0x18, 0x1e,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x65, 0x78, 0x70, 0x69, 0x72, 0x79, 0x12, 0x2b, 0x0a, 0x11,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x28, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x44, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0xdd, 0x01, 0x0a, 0x17, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x6c, 0x61, 0x69,
    0x6e, 0x74, 0x65, 0x78, 0x74, 0x12, 0x4d, 0x0a, 0x15, 0x6c, 0x6f, 0x67, 0x69, 0x6e, 0x5f, 0x63,
    0x72, 0x79, 0x70, 0x74, 0x6f, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x0a,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70,
    0x74, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x52,
    0x13, 0x6c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x34, 0x0a, 0x0c, 0x70, 0x6f, 0x77, 0x5f, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x18, 0x14, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x50, 0x6f, 0x57,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x52, 0x0b, 0x70,
    0x6f, 0x77, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3d, 0x0a, 0x0f, 0x63, 0x72,
    0x79, 0x70, 0x74, 0x6f, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x1e, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x52, 0x0e, 0x63, 0x72, 0x79, 0x70, 0x74,
    0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x64, 0x0a, 0x18, 0x4c, 0x6f, 0x67,
    0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x48, 0x0a, 0x0e, 0x64, 0x69, 0x66, 0x66, 0x69, 0x65, 0x5f,
    0x68, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e,
    0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x44, 0x69, 0x66, 0x66, 0x69,
    0x65, 0x48, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x52, 0x0d, 0x64, 0x69, 0x66, 0x66, 0x69, 0x65, 0x48, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x22,
    0x36, 0x0a, 0x20, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x44, 0x69,
    0x66, 0x66, 0x69, 0x65, 0x48, 0x65, 0x6c, 0x6c, 0x6d, 0x61, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x6d, 0x61, 0x63, 0x18, 0x0a, 0x20, 0x02, 0x28,
    0x0c, 0x52, 0x04, 0x68, 0x6d, 0x61, 0x63, 0x22, 0x45, 0x0a, 0x10, 0x50, 0x6f, 0x57, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x31, 0x0a, 0x09, 0x68,
    0x61, 0x73, 0x68, 0x5f, 0x63, 0x61, 0x73, 0x68, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14,
    0x2e, 0x50, 0x6f, 0x57, 0x48, 0x61, 0x73, 0x68, 0x43, 0x61, 0x73, 0x68, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x52, 0x08, 0x68, 0x61, 0x73, 0x68, 0x43, 0x61, 0x73, 0x68, 0x22, 0x36,
    0x0a, 0x13, 0x50, 0x6f, 0x57, 0x48, 0x61, 0x73, 0x68, 0x43, 0x61, 0x73, 0x68, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x68, 0x61, 0x73, 0x68, 0x5f, 0x73, 0x75,
    0x66, 0x66, 0x69, 0x78, 0x18, 0x0a, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x0a, 0x68, 0x61, 0x73, 0x68,
    0x53, 0x75, 0x66, 0x66, 0x69, 0x78, 0x22, 0x87, 0x01, 0x0a, 0x13, 0x43, 0x72, 0x79, 0x70, 0x74,
    0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x12, 0x30,
    0x0a, 0x07, 0x73, 0x68, 0x61, 0x6e, 0x6e, 0x6f, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x16, 0x2e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x53, 0x68, 0x61, 0x6e, 0x6e, 0x6f, 0x6e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x07, 0x73, 0x68, 0x61, 0x6e, 0x6e, 0x6f, 0x6e,
    0x12, 0x3e, 0x0a, 0x0d, 0x72, 0x63, 0x34, 0x5f, 0x73, 0x68, 0x61, 0x31, 0x5f, 0x68, 0x6d, 0x61,
    0x63, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f,
    0x52, 0x63, 0x34, 0x53, 0x68, 0x61, 0x31, 0x48, 0x6d, 0x61, 0x63, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x52, 0x0b, 0x72, 0x63, 0x34, 0x53, 0x68, 0x61, 0x31, 0x48, 0x6d, 0x61, 0x63,
    0x22, 0x2d, 0x0a, 0x15, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x53, 0x68, 0x61, 0x6e, 0x6e, 0x6f,
    0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x75, 0x6d,
    0x6d, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x64, 0x75, 0x6d, 0x6d, 0x79, 0x22,
    0x31, 0x0a, 0x19, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x52, 0x63, 0x34, 0x53, 0x68, 0x61, 0x31,
    0x48, 0x6d, 0x61, 0x63, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x05,
    0x64, 0x75, 0x6d, 0x6d, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x64, 0x75, 0x6d,
    0x6d, 0x79, 0x2a, 0x7f, 0x0a, 0x07, 0x50, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x12, 0x12, 0x0a,
    0x0e, 0x50, 0x52, 0x4f, 0x44, 0x55, 0x43, 0x54, 0x5f, 0x43, 0x4c, 0x49, 0x45, 0x4e, 0x54, 0x10,
    0x00, 0x12, 0x16, 0x0a, 0x12, 0x50, 0x52, 0x4f, 0x44, 0x55, 0x43, 0x54, 0x5f, 0x4c, 0x49, 0x42,
    0x53, 0x50, 0x4f, 0x54, 0x49, 0x46, 0x59, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x50, 0x52, 0x4f,
    0x44, 0x55, 0x43, 0x54, 0x5f, 0x4d, 0x4f, 0x42, 0x49, 0x4c, 0x45, 0x10, 0x02, 0x12, 0x13, 0x0a,
    0x0f, 0x50, 0x52, 0x4f, 0x44, 0x55, 0x43, 0x54, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x4e, 0x45, 0x52,
    0x10, 0x03, 0x12, 0x1f, 0x0a, 0x1b, 0x50, 0x52, 0x4f, 0x44, 0x55, 0x43, 0x54, 0x5f, 0x4c, 0x49,
    0x42, 0x53, 0x50, 0x4f, 0x54, 0x49, 0x46, 0x59, 0x5f, 0x45, 0x4d, 0x42, 0x45, 0x44, 0x44, 0x45,
    0x44, 0x10, 0x05, 0x2a, 0x41, 0x0a, 0x0c, 0x50, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x46, 0x6c,
    0x61, 0x67, 0x73, 0x12, 0x15, 0x0a, 0x11, 0x50, 0x52, 0x4f, 0x44, 0x55, 0x43, 0x54, 0x5f, 0x46,
    0x4c, 0x41, 0x47, 0x5f, 0x4e, 0x4f, 0x4e, 0x45, 0x10, 0x00, 0x12, 0x1a, 0x0a, 0x16, 0x50, 0x52,
    0x4f, 0x44, 0x55, 0x43, 0x54, 0x5f, 0x46, 0x4c, 0x41, 0x47, 0x5f, 0x44, 0x45, 0x56, 0x5f, 0x42,
    0x55, 0x49, 0x4c, 0x44, 0x10, 0x01, 0x2a, 0xdc, 0x04, 0x0a, 0x08, 0x50, 0x6c, 0x61, 0x74, 0x66,
    0x6f, 0x72, 0x6d, 0x12, 0x16, 0x0a, 0x12, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f,
    0x57, 0x49, 0x4e, 0x33, 0x32, 0x5f, 0x58, 0x38, 0x36, 0x10, 0x00, 0x12, 0x14, 0x0a, 0x10, 0x50,
    0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x4f, 0x53, 0x58, 0x5f, 0x58, 0x38, 0x36, 0x10,
    0x01, 0x12, 0x16, 0x0a, 0x12, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x4c, 0x49,
    0x4e, 0x55, 0x58, 0x5f, 0x58, 0x38, 0x36, 0x10, 0x02, 0x12, 0x17, 0x0a, 0x13, 0x50, 0x4c, 0x41,
    0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x49, 0x50, 0x48, 0x4f, 0x4e, 0x45, 0x5f, 0x41, 0x52, 0x4d,
    0x10, 0x03, 0x12, 0x14, 0x0a, 0x10, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x53,
    0x36, 0x30, 0x5f, 0x41, 0x52, 0x4d, 0x10, 0x04, 0x12, 0x14, 0x0a, 0x10, 0x50, 0x4c, 0x41, 0x54,
    0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x4f, 0x53, 0x58, 0x5f, 0x50, 0x50, 0x43, 0x10, 0x05, 0x12, 0x18,
    0x0a, 0x14, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x41, 0x4e, 0x44, 0x52, 0x4f,
    0x49, 0x44, 0x5f, 0x41, 0x52, 0x4d, 0x10, 0x06, 0x12, 0x1b, 0x0a, 0x17, 0x50, 0x4c, 0x41, 0x54,
    0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x57, 0x49, 0x4e, 0x44, 0x4f, 0x57, 0x53, 0x5f, 0x43, 0x45, 0x5f,
    0x41, 0x52, 0x4d, 0x10, 0x07, 0x12, 0x19, 0x0a, 0x15, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52,
    0x4d, 0x5f, 0x4c, 0x49, 0x4e, 0x55, 0x58, 0x5f, 0x58, 0x38, 0x36, 0x5f, 0x36, 0x34, 0x10, 0x08,
    0x12, 0x17, 0x0a, 0x13, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x4f, 0x53, 0x58,
    0x5f, 0x58, 0x38, 0x36, 0x5f, 0x36, 0x34, 0x10, 0x09, 0x12, 0x15, 0x0a, 0x11, 0x50, 0x4c, 0x41,
    0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x50, 0x41, 0x4c, 0x4d, 0x5f, 0x41, 0x52, 0x4d, 0x10, 0x0a,
    0x12, 0x15, 0x0a, 0x11, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x4c, 0x49, 0x4e,
    0x55, 0x58, 0x5f, 0x53, 0x48, 0x10, 0x0b, 0x12, 0x18, 0x0a, 0x14, 0x50, 0x4c, 0x41, 0x54, 0x46,
    0x4f, 0x52, 0x4d, 0x5f, 0x46, 0x52, 0x45, 0x45, 0x42, 0x53, 0x44, 0x5f, 0x58, 0x38, 0x36, 0x10,
    0x0c, 0x12, 0x1b, 0x0a, 0x17, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x46, 0x52,
    0x45, 0x45, 0x42, 0x53, 0x44, 0x5f, 0x58, 0x38, 0x36, 0x5f, 0x36, 0x34, 0x10, 0x0d, 0x12, 0x1b,
    0x0a, 0x17, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x42, 0x4c, 0x41, 0x43, 0x4b,
    0x42, 0x45, 0x52, 0x52, 0x59, 0x5f, 0x41, 0x52, 0x4d, 0x10, 0x0e, 0x12, 0x12, 0x0a, 0x0e, 0x50,
    0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x53, 0x4f, 0x4e, 0x4f, 0x53, 0x10, 0x0f, 0x12,
    0x17, 0x0a, 0x13, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x4c, 0x49, 0x4e, 0x55,
    0x58, 0x5f, 0x4d, 0x49, 0x50, 0x53, 0x10, 0x10, 0x12, 0x16, 0x0a, 0x12, 0x50, 0x4c, 0x41, 0x54,
    0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x4c, 0x49, 0x4e, 0x55, 0x58, 0x5f, 0x41, 0x52, 0x4d, 0x10, 0x11,
    0x12, 0x19, 0x0a, 0x15, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x4c, 0x4f, 0x47,
    0x49, 0x54, 0x45, 0x43, 0x48, 0x5f, 0x41, 0x52, 0x4d, 0x10, 0x12, 0x12, 0x1b, 0x0a, 0x17, 0x50,
    0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x4c, 0x49, 0x4e, 0x55, 0x58, 0x5f, 0x42, 0x4c,
    0x41, 0x43, 0x4b, 0x46, 0x49, 0x4e, 0x10, 0x13, 0x12, 0x14, 0x0a, 0x10, 0x50, 0x4c, 0x41, 0x54,
    0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x57, 0x50, 0x37, 0x5f, 0x41, 0x52, 0x4d, 0x10, 0x14, 0x12, 0x16,
    0x0a, 0x12, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x4f, 0x4e, 0x4b, 0x59, 0x4f,
    0x5f, 0x41, 0x52, 0x4d, 0x10, 0x15, 0x12, 0x17, 0x0a, 0x13, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f,
    0x52, 0x4d, 0x5f, 0x51, 0x4e, 0x58, 0x4e, 0x54, 0x4f, 0x5f, 0x41, 0x52, 0x4d, 0x10, 0x16, 0x12,
    0x14, 0x0a, 0x10, 0x50, 0x4c, 0x41, 0x54, 0x46, 0x4f, 0x52, 0x4d, 0x5f, 0x42, 0x43, 0x4f, 0x5f,
    0x41, 0x52, 0x4d, 0x10, 0x17, 0x2a, 0x41, 0x0a, 0x0b, 0x46, 0x69, 0x6e, 0x67, 0x65, 0x72, 0x70,
    0x72, 0x69, 0x6e, 0x74, 0x12, 0x15, 0x0a, 0x11, 0x46, 0x49, 0x4e, 0x47, 0x45, 0x52, 0x50, 0x52,
    0x49, 0x4e, 0x54, 0x5f, 0x47, 0x52, 0x41, 0x49, 0x4e, 0x10, 0x00, 0x12, 0x1b, 0x0a, 0x17, 0x46,
    0x49, 0x4e, 0x47, 0x45, 0x52, 0x50, 0x52, 0x49, 0x4e, 0x54, 0x5f, 0x48, 0x4d, 0x41, 0x43, 0x5f,
    0x52, 0x49, 0x50, 0x45, 0x4d, 0x44, 0x10, 0x01, 0x2a, 0x47, 0x0a, 0x0b, 0x43, 0x72, 0x79, 0x70,
    0x74, 0x6f, 0x73, 0x75, 0x69, 0x74, 0x65, 0x12, 0x18, 0x0a, 0x14, 0x43, 0x52, 0x59, 0x50, 0x54,
    0x4f, 0x5f, 0x53, 0x55, 0x49, 0x54, 0x45, 0x5f, 0x53, 0x48, 0x41, 0x4e, 0x4e, 0x4f, 0x4e, 0x10,
    0x00, 0x12, 0x1e, 0x0a, 0x1a, 0x43, 0x52, 0x59, 0x50, 0x54, 0x4f, 0x5f, 0x53, 0x55, 0x49, 0x54,
    0x45, 0x5f, 0x52, 0x43, 0x34, 0x5f, 0x53, 0x48, 0x41, 0x31, 0x5f, 0x48, 0x4d, 0x41, 0x43, 0x10,
    0x01, 0x2a, 0x1e, 0x0a, 0x09, 0x50, 0x6f, 0x77, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x12, 0x11,
    0x0a, 0x0d, 0x50, 0x4f, 0x57, 0x5f, 0x48, 0x41, 0x53, 0x48, 0x5f, 0x43, 0x41, 0x53, 0x48, 0x10,
    0x00, 0x2a, 0x89, 0x02, 0x0a, 0x09, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x43, 0x6f, 0x64, 0x65, 0x12,
    0x11, 0x0a, 0x0d, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x45, 0x72, 0x72, 0x6f, 0x72,
    0x10, 0x00, 0x12, 0x10, 0x0a, 0x0c, 0x54, 0x72, 0x79, 0x41, 0x6e, 0x6f, 0x74, 0x68, 0x65, 0x72,
    0x41, 0x50, 0x10, 0x02, 0x12, 0x13, 0x0a, 0x0f, 0x42, 0x61, 0x64, 0x43, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x10, 0x05, 0x12, 0x15, 0x0a, 0x11, 0x54, 0x72, 0x61,
    0x76, 0x65, 0x6c, 0x52, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0x09,
    0x12, 0x1a, 0x0a, 0x16, 0x50, 0x72, 0x65, 0x6d, 0x69, 0x75, 0x6d, 0x41, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x10, 0x0b, 0x12, 0x12, 0x0a, 0x0e,
    0x42, 0x61, 0x64, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x10, 0x0c,
    0x12, 0x1f, 0x0a, 0x1b, 0x43, 0x6f, 0x75, 0x6c, 0x64, 0x4e, 0x6f, 0x74, 0x56, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x10,
    0x0d, 0x12, 0x11, 0x0a, 0x0d, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x45, 0x78, 0x69, 0x73,
    0x74, 0x73, 0x10, 0x0e, 0x12, 0x1d, 0x0a, 0x19, 0x45, 0x78, 0x74, 0x72, 0x61, 0x56, 0x65, 0x72,
    0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65,
    0x64, 0x10, 0x0f, 0x12, 0x11, 0x0a, 0x0d, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x41, 0x70,
    0x70, 0x4b, 0x65, 0x79, 0x10, 0x10, 0x12, 0x15, 0x0a, 0x11, 0x41, 0x70, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x61, 0x6e, 0x6e, 0x65, 0x64, 0x10, 0x11, 0x4a, 0xbd, 0x36,
    0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xe2, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x0b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x03, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03,
    0x17, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x24, 0x27,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x04, 0x37, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x04, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x04, 0x19, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x04, 0x32, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x05,
    0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x05, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x05, 0x0d, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x19, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x32, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x06, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03,
    0x06, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x17,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x06, 0x2e, 0x32, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x07, 0x04, 0x3d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x06, 0x12, 0x03, 0x07, 0x0d, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x07, 0x23, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x07, 0x38, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x08, 0x04,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x08, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x08, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x08, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x08, 0x22, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x06, 0x12, 0x03, 0x09, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x09,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x09, 0x13, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x09, 0x1d, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0a, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x06, 0x12, 0x03, 0x0a, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x0a, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x0a, 0x26, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0e, 0x00, 0x13, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x0f, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f,
    0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x1f, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x10, 0x04, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x10, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x10, 0x1a, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x10, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x11,
    0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x11, 0x0d, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x11, 0x21, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x03, 0x12, 0x03, 0x12, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x12, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x12, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x12, 0x14,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x12, 0x1e, 0x22, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x15, 0x00, 0x1b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x00, 0x01, 0x12, 0x03, 0x15, 0x05, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x16, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16,
    0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x16, 0x15, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x17, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x18, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x18, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x18,
    0x15, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x19, 0x04, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x19, 0x04, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x19, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x1a, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x1a, 0x22, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x1d, 0x00, 0x20, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1e, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x1e, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1f,
    0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x04, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1f, 0x1d, 0x20, 0x0a, 0x0a,
    0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x22, 0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x02,
    0x01, 0x12, 0x03, 0x22, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x23, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x04,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x23, 0x19, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x24, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x24, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x25, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x25, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x03, 0x25, 0x19,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x04, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x26, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x03, 0x02, 0x12, 0x03, 0x26, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x27, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x27, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04, 0x02, 0x12, 0x03,
    0x27, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x28, 0x04, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x28, 0x04, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x02, 0x12, 0x03, 0x28, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x29, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x29, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x06, 0x02,
    0x12, 0x03, 0x29, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x07, 0x12, 0x03, 0x2a,
    0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x2a, 0x04, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x07, 0x02, 0x12, 0x03, 0x2a, 0x1e, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x02, 0x02, 0x08, 0x12, 0x03, 0x2b, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x2b, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x08, 0x02, 0x12, 0x03, 0x2b, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x09, 0x12,
    0x03, 0x2c, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x09, 0x01, 0x12, 0x03, 0x2c,
    0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x09, 0x02, 0x12, 0x03, 0x2c, 0x1a, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x0a, 0x12, 0x03, 0x2d, 0x04, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x2d, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x2d, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x0b, 0x12, 0x03, 0x2e, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0b, 0x01, 0x12,
    0x03, 0x2e, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x2e,
    0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x0c, 0x12, 0x03, 0x2f, 0x04, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x2f, 0x04, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x2f, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x0d, 0x12, 0x03, 0x30, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0d,
    0x01, 0x12, 0x03, 0x30, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0d, 0x02, 0x12,
    0x03, 0x30, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x0e, 0x12, 0x03, 0x31, 0x04,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x31, 0x04, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x31, 0x1e, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x02, 0x02, 0x0f, 0x12, 0x03, 0x32, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x0f, 0x01, 0x12, 0x03, 0x32, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x0f,
    0x02, 0x12, 0x03, 0x32, 0x15, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x10, 0x12, 0x03,
    0x33, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x10, 0x01, 0x12, 0x03, 0x33, 0x04,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x10, 0x02, 0x12, 0x03, 0x33, 0x1a, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x11, 0x12, 0x03, 0x34, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x11, 0x01, 0x12, 0x03, 0x34, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x11, 0x02, 0x12, 0x03, 0x34, 0x19, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x12,
    0x12, 0x03, 0x35, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x12, 0x01, 0x12, 0x03,
    0x35, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x12, 0x02, 0x12, 0x03, 0x35, 0x1c,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x13, 0x12, 0x03, 0x36, 0x04, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x13, 0x01, 0x12, 0x03, 0x36, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x13, 0x02, 0x12, 0x03, 0x36, 0x1e, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02,
    0x02, 0x14, 0x12, 0x03, 0x37, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x14, 0x01,
    0x12, 0x03, 0x37, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x14, 0x02, 0x12, 0x03,
    0x37, 0x17, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x15, 0x12, 0x03, 0x38, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x15, 0x01, 0x12, 0x03, 0x38, 0x04, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x15, 0x02, 0x12, 0x03, 0x38, 0x19, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x02, 0x02, 0x16, 0x12, 0x03, 0x39, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x16, 0x01, 0x12, 0x03, 0x39, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x16, 0x02,
    0x12, 0x03, 0x39, 0x1a, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x17, 0x12, 0x03, 0x3a,
    0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x17, 0x01, 0x12, 0x03, 0x3a, 0x04, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x17, 0x02, 0x12, 0x03, 0x3a, 0x17, 0x1b, 0x0a, 0x0a,
    0x0a, 0x02, 0x05, 0x03, 0x12, 0x04, 0x3d, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x03,
    0x01, 0x12, 0x03, 0x3d, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x00, 0x12, 0x03,
    0x3e, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x04,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x02, 0x12, 0x03, 0x3e, 0x18, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x3f, 0x1e, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x04, 0x12, 0x04,
    0x42, 0x00, 0x45, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x04, 0x01, 0x12, 0x03, 0x42, 0x05, 0x10,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x00, 0x12, 0x03, 0x43, 0x04, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x04, 0x02, 0x00, 0x02, 0x12, 0x03, 0x43, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x44, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x44, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x01, 0x02, 0x12, 0x03, 0x44,
    0x21, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x05, 0x12, 0x04, 0x47, 0x00, 0x49, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x05, 0x01, 0x12, 0x03, 0x47, 0x05, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x48, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x48, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x48, 0x14, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x4c, 0x00, 0x4e, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x4d, 0x04, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x4d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x4d, 0x0d, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d,
    0x2b, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x3c, 0x3f,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x51, 0x00, 0x54, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x03, 0x01, 0x12, 0x03, 0x51, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x52, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x52, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x52, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x13, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x18, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x53, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x53, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x53, 0x14, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x53,
    0x28, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x57, 0x00, 0x5a, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x57, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x58, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x58, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x58, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x58, 0x12,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x58, 0x20, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x59, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x59, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x59, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x59, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x59, 0x25, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x5d, 0x00, 0x61, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x5d, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x5e, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x5e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x5e, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x5e, 0x19, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5e, 0x25,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x04, 0x33, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x5f, 0x0d, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x5f, 0x24, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x5f, 0x2e, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03,
    0x60, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x60, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x60, 0x0d, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x60, 0x1b, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x60, 0x2a, 0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x06, 0x12, 0x04, 0x63, 0x00, 0x6a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03,
    0x63, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x64, 0x04, 0x44,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x64, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x64, 0x0d, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x27, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x64, 0x40, 0x43, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01,
    0x12, 0x03, 0x65, 0x04, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x65, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x65, 0x0d,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x65, 0x27, 0x3c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x65, 0x3f, 0x43, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x66, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x66, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x66, 0x0d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x66, 0x1f, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x66,
    0x2f, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x67, 0x04, 0x3a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x67, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x67, 0x0d, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x67, 0x22, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x67, 0x35, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12,
    0x03, 0x68, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x68,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x68, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x68, 0x13, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x68, 0x22, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x69, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x69, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x69, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x69, 0x1d,
    0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x6c, 0x00, 0x6e, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x6c, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x00, 0x12, 0x03, 0x6d, 0x04, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x6d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6d,
    0x0d, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6d, 0x2f, 0x3d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6d, 0x40, 0x43, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x70, 0x00, 0x74, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08,
    0x01, 0x12, 0x03, 0x70, 0x08, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03,
    0x71, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x71, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x71, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x71, 0x13, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x71, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x01, 0x12, 0x03, 0x72, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x72, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x72, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x72,
    0x13, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x72, 0x2a, 0x2e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x73, 0x04, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x73, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x73, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x73, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x73, 0x22, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x76, 0x00, 0x79,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x76, 0x08, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x77, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x77, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x77, 0x0d, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x77, 0x27, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x77,
    0x2f, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x78, 0x04, 0x3f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x78, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x03, 0x78, 0x0d, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x78, 0x2c, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x78, 0x3a, 0x3e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x7c,
    0x00, 0x7e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x7c, 0x08, 0x21, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x7d, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x7d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x7d, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x7d, 0x19, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0x81, 0x01, 0x00, 0x83,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0x81, 0x01, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x82, 0x01, 0x04, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x82, 0x01, 0x13, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x82, 0x01, 0x1f, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0c, 0x12,
    0x06, 0x86, 0x01, 0x00, 0x88, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04,
    0x86, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0x87, 0x01,
    0x04, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x87, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x04, 0x87, 0x01, 0x0d, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x01, 0x22, 0x2b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x01, 0x2e, 0x31, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0x8a, 0x01, 0x00, 0x8e, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0d, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x00, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x8b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12,
    0x04, 0x8b, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x8b, 0x01, 0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8b,
    0x01, 0x1c, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x04,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8c, 0x01, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x13, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x1c, 0x20, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x02, 0x05, 0x12, 0x04, 0x8d, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x13, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02,
    0x03, 0x12, 0x04, 0x8d, 0x01, 0x1c, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0x91,
    0x01, 0x00, 0x94, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0x91, 0x01,
    0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0x92, 0x01, 0x04, 0x32,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0x92, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x04, 0x92, 0x01, 0x0d, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0x92, 0x01, 0x24, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x92, 0x01, 0x2e, 0x31, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0x93, 0x01, 0x04, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x93, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x06, 0x12, 0x04, 0x93, 0x01, 0x0d, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x93, 0x01, 0x28, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x93, 0x01, 0x38, 0x3c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0x97, 0x01,
    0x00, 0x98, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0x97, 0x01, 0x08,
    0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x9b, 0x01, 0x00, 0x9c, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x11, 0x12, 0x06, 0x9f, 0x01, 0x00, 0xa3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11,
    0x01, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12,
    0x04, 0xa0, 0x01, 0x04, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xa0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa0,
    0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa0, 0x01,
    0x13, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x29,
    0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa1, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x13, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x1f, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x11, 0x02, 0x02, 0x12, 0x04, 0xa2, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xa2, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xa2, 0x01, 0x14, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xa2, 0x01, 0x22, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xa5, 0x01, 0x00,
    0xaa, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa6, 0x01, 0x0d, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x17, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x24, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12,
    0x02, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xa7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xa7, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xa7, 0x01, 0x13, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xa7, 0x01, 0x21, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x02, 0x12, 0x04, 0xa8, 0x01,
    0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa8, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa8, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x13, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa8, 0x01, 0x1c, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x2d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa9, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x14, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x28, 0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x06, 0x12, 0x06,
    0xac, 0x01, 0x00, 0xb8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x06, 0x01, 0x12, 0x04, 0xac,
    0x01, 0x05, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x00, 0x12, 0x04, 0xad, 0x01, 0x04,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x00, 0x01, 0x12, 0x04, 0xad, 0x01, 0x04, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x00, 0x02, 0x12, 0x04, 0xad, 0x01, 0x14, 0x17, 0x0a,
    0x0c, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x01, 0x12, 0x04, 0xae, 0x01, 0x04, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x06, 0x02, 0x01, 0x01, 0x12, 0x04, 0xae, 0x01, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x06, 0x02, 0x01, 0x02, 0x12, 0x04, 0xae, 0x01, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x05,
    0x06, 0x02, 0x02, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x02,
    0x02, 0x12, 0x04, 0xaf, 0x01, 0x16, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x03, 0x12,
    0x04, 0xb0, 0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x03, 0x01, 0x12, 0x04,
    0xb0, 0x01, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x03, 0x02, 0x12, 0x04, 0xb0,
    0x01, 0x18, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x04,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x04, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x04, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x04, 0x02, 0x12, 0x04, 0xb1, 0x01, 0x1d, 0x20, 0x0a,
    0x0c, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x05, 0x12, 0x04, 0xb2, 0x01, 0x04, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x06, 0x02, 0x05, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x04, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x06, 0x02, 0x05, 0x02, 0x12, 0x04, 0xb2, 0x01, 0x15, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x05,
    0x06, 0x02, 0x06, 0x12, 0x04, 0xb3, 0x01, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02,
    0x06, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x06,
    0x02, 0x12, 0x04, 0xb3, 0x01, 0x22, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x07, 0x12,
    0x04, 0xb4, 0x01, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x07, 0x01, 0x12, 0x04,
    0xb4, 0x01, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x07, 0x02, 0x12, 0x04, 0xb4,
    0x01, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x08, 0x12, 0x04, 0xb5, 0x01, 0x04,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x08, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x04, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x08, 0x02, 0x12, 0x04, 0xb5, 0x01, 0x20, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x09, 0x12, 0x04, 0xb6, 0x01, 0x04, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x06, 0x02, 0x09, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x06, 0x02, 0x09, 0x02, 0x12, 0x04, 0xb6, 0x01, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x05,
    0x06, 0x02, 0x0a, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02,
    0x0a, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x0a,
    0x02, 0x12, 0x04, 0xb7, 0x01, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xba,
    0x01, 0x00, 0xbe, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xba, 0x01,
    0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xbb, 0x01, 0x04, 0x42,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbb, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xbb, 0x01, 0x0d, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x26, 0x3b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x3e, 0x41, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x04, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xbc, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xbc, 0x01, 0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xbc, 0x01, 0x1e, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xbc, 0x01, 0x2d, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04,
    0xbd, 0x01, 0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbd,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x06, 0x12, 0x04, 0xbd, 0x01,
    0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x21,
    0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbd, 0x01, 0x33, 0x37,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xc1, 0x01, 0x00, 0xc3, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x14, 0x02, 0x00, 0x12, 0x04, 0xc2, 0x01, 0x04, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x06, 0x12, 0x04, 0xc2, 0x01, 0x0d, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xc2, 0x01, 0x2e, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xc2, 0x01, 0x3f, 0x42, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xc6, 0x01, 0x00,
    0xc8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x28,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xc7, 0x01, 0x04, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc7, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc7, 0x01, 0x1a, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16,
    0x12, 0x06, 0xcb, 0x01, 0x00, 0xcd, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12,
    0x04, 0xcb, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xcc,
    0x01, 0x04, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcc, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x06, 0x12, 0x04, 0xcc, 0x01, 0x0d,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x21, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x2d, 0x30, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xd0, 0x01, 0x00, 0xd2, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17,
    0x02, 0x00, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xd1, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd1, 0x01, 0x13, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xd1, 0x01, 0x21, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xd5, 0x01, 0x00, 0xd8,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x08, 0x1b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xd6, 0x01, 0x04, 0x31, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd6, 0x01, 0x0d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x23, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x2d, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02,
    0x01, 0x12, 0x04, 0xd7, 0x01, 0x04, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xd7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xd7, 0x01, 0x0d, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xd7, 0x01, 0x27, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd7,
    0x01, 0x37, 0x3b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xdb, 0x01, 0x00, 0xdd, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x08, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdc, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdc, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x13, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xdc, 0x01, 0x1b, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06,
    0xe0, 0x01, 0x00, 0xe2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xe0,
    0x01, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xe1, 0x01, 0x04,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe1, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe1, 0x01, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x13, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe1, 0x01, 0x1b, 0x1e,
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
