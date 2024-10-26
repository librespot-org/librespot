#![cfg(feature = "with-avahi")]

#[allow(unused)]
pub use server::ServerProxy;

#[allow(unused)]
pub use entry_group::{
    EntryGroupProxy, EntryGroupState, StateChangedStream as EntryGroupStateChangedStream,
};

mod server {
    // This is not the full interface, just the methods we need!
    // Avahi also implements a newer version of the interface ("org.freedesktop.Avahi.Server2"), but
    // the additions are not relevant for us, and the older version is not intended to be deprecated.
    // cf. the release notes for 0.8 at https://github.com/avahi/avahi/blob/master/docs/NEWS
    #[zbus::proxy(
        interface = "org.freedesktop.Avahi.Server",
        default_service = "org.freedesktop.Avahi",
        default_path = "/",
        gen_blocking = false
    )]
    trait Server {
        /// EntryGroupNew method
        #[zbus(object = "super::entry_group::EntryGroup")]
        fn entry_group_new(&self);

        /// GetState method
        fn get_state(&self) -> zbus::Result<i32>;

        /// StateChanged signal
        #[zbus(signal)]
        fn state_changed(&self, state: i32, error: &str) -> zbus::Result<()>;
    }
}

mod entry_group {
    use serde_repr::Deserialize_repr;
    use zbus::zvariant;

    #[derive(Clone, Copy, Debug, Deserialize_repr)]
    #[repr(i32)]
    pub enum EntryGroupState {
        // The group has not yet been committed, the user must still call avahi_entry_group_commit()
        Uncommited = 0,
        // The entries of the group are currently being registered
        Registering = 1,
        // The entries have successfully been established
        Established = 2,
        // A name collision for one of the entries in the group has been detected, the entries have been withdrawn
        Collision = 3,
        // Some kind of failure happened, the entries have been withdrawn
        Failure = 4,
    }

    impl zvariant::Type for EntryGroupState {
        fn signature() -> zvariant::Signature<'static> {
            zvariant::Signature::try_from("i").unwrap()
        }
    }

    #[zbus::proxy(
        interface = "org.freedesktop.Avahi.EntryGroup",
        default_service = "org.freedesktop.Avahi",
        gen_blocking = false
    )]
    trait EntryGroup {
        /// AddAddress method
        fn add_address(
            &self,
            interface: i32,
            protocol: i32,
            flags: u32,
            name: &str,
            address: &str,
        ) -> zbus::Result<()>;

        /// AddRecord method
        #[allow(clippy::too_many_arguments)]
        fn add_record(
            &self,
            interface: i32,
            protocol: i32,
            flags: u32,
            name: &str,
            clazz: u16,
            type_: u16,
            ttl: u32,
            rdata: &[u8],
        ) -> zbus::Result<()>;

        /// AddService method
        #[allow(clippy::too_many_arguments)]
        fn add_service(
            &self,
            interface: i32,
            protocol: i32,
            flags: u32,
            name: &str,
            type_: &str,
            domain: &str,
            host: &str,
            port: u16,
            txt: &[&[u8]],
        ) -> zbus::Result<()>;

        /// AddServiceSubtype method
        #[allow(clippy::too_many_arguments)]
        fn add_service_subtype(
            &self,
            interface: i32,
            protocol: i32,
            flags: u32,
            name: &str,
            type_: &str,
            domain: &str,
            subtype: &str,
        ) -> zbus::Result<()>;

        /// Commit method
        fn commit(&self) -> zbus::Result<()>;

        /// Free method
        fn free(&self) -> zbus::Result<()>;

        /// GetState method
        fn get_state(&self) -> zbus::Result<EntryGroupState>;

        /// IsEmpty method
        fn is_empty(&self) -> zbus::Result<bool>;

        /// Reset method
        fn reset(&self) -> zbus::Result<()>;

        /// UpdateServiceTxt method
        #[allow(clippy::too_many_arguments)]
        fn update_service_txt(
            &self,
            interface: i32,
            protocol: i32,
            flags: u32,
            name: &str,
            type_: &str,
            domain: &str,
            txt: &[&[u8]],
        ) -> zbus::Result<()>;

        /// StateChanged signal
        #[zbus(signal)]
        fn state_changed(&self, state: EntryGroupState, error: &str) -> zbus::Result<()>;
    }
}
