/// `lib_kickstart` is a Rust module to parse Kickstart files based on the `pykickstart` and
/// largely follows the same structure.
///
/// This entire file was written while listening to Motley Crue (on repeat, of
/// course).

pub mod section {
    trait Section {
        fn handle_head();
        fn handle_line();

        fn finalize();
    }

    struct Null {}

    struct PreInstall {}
    struct PostInstall {}

    struct Traceback {}

    struct Package {}
}

pub mod version {
    use std::collections::HashMap;

    pub enum Version {
        RHEL3 = 900,
        FC3 = 1000,
        RHEL4 = 1100,
        FC4 = 2000,
        FC5 = 3000,
        FC6 = 4000,
        RHEL5 = 4100,
        F7 = 5000,
        F8 = 6000,
        F9 = 7000,
        F10 = 8000,
        F11 = 9000,
        F12 = 10000,
        F13 = 11000,
        RHEL6 = 11100,
        F14 = 12000,
        F15 = 13000,
        F16 = 14000,
        F17 = 15000,
        F18 = 16000,
        F19 = 17000,
        F20 = 18000,
        F21 = 19000,
        RHEL7 = 19100,
        F22 = 20000,
        F23 = 21000,
        F24 = 22000,
        F25 = 23000,
        F26 = 24000,
        F27 = 25000,
        F28 = 26000,
        RHEL8 = 26100,
        F29 = 27000,
        F30 = 30000,
        F31 = 31000,
        F32 = 32000,
        F33 = 33000,
        F34 = 34000,
        RHEL9 = 34100,
        F35 = 35000,
        F36 = 36000,
        F37 = 37000,
    }

    impl Version {
        pub const DEVEL: Version = Version::F37;
    }

    fn version_map() -> HashMap<&'static str, Version> {
        HashMap::from([("DEVEL", Version::DEVEL)])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
