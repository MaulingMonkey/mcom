use std::fmt::Display;

fn is_feature_enabled(f: &str) -> bool {
    let mut f = f.replace("-", "_");
    f.make_ascii_uppercase();
    std::env::var_os(format!("CARGO_FEATURE_{}", f)).is_some()
}

fn cfg(k: &str, v: impl Display) {
    println!("cargo:rustc-cfg={}={:?}", k, v.to_string());
}

fn main() {
    // https://en.wikipedia.org/wiki/List_of_Microsoft_Windows_versions
    let feature_ver = [
        ("windows-2000",    "2000"),
        ("windows-xp",      "xp"),
        ("windows-vista",   "vista"),
        ("windows-7",       "7"),
        ("windows-7",       "7.0"),
        ("windows-8",       "8"),
        ("windows-8",       "8.0"),
        ("windows-8-1",     "8.1"),
        ("windows-10",      "10"),
        ("windows-10",      "10.0"),
    ];

    print!(r#"cargo::rustc-check-cfg=cfg(windows, values("#);
    for (_, ver) in feature_ver.iter().copied() { print!("{:?}, ", ver); }
    println!("))");

    for (feature, ver) in feature_ver.iter().copied() {
        if is_feature_enabled(feature) {
            cfg("windows", ver);
        }
    }

    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\shared\winapifamily.h
    let feature_partition = [
        ("winapi-family-desktop-app",   "desktop"),
        ("winapi-family-desktop-app",   "app"),
        ("winapi-family-desktop-app",   "pc-app"),

        ("winapi-family-pc-app",        "app"),
        ("winapi-family-pc-app",        "pc-app"),

        ("winapi-family-phone-app",     "app"),
        ("winapi-family-phone-app",     "phone-app"),

        ("winapi-family-system",        "system"),

        ("winapi-family-server",        "system"),
        ("winapi-family-server",        "server"),

        ("winapi-family-games",         "games"),
    ];

    print!(r#"cargo::rustc-check-cfg=cfg(partition, values("#);
    for (_, partition) in feature_partition.iter().copied() { print!("{:?}, ", partition); }
    println!(r#"))"#);

    for (feature, partition) in feature_partition.iter().copied() {
        if is_feature_enabled(feature) {
            cfg("partition", partition);
        }
    }
}
