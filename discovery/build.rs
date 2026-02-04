use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        discovery: { any(feature = "with-libmdns", feature = "with-dns-sd", feature = "with-avahi") }
    }
}
