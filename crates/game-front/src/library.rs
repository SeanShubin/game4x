//! The command files, carried in the binary.
//!
//! A browser has no filesystem, so the files a desktop build could read off disk are
//! compiled in instead. Both builds get the same ones, which is what keeps `run setup`
//! meaning the same thing everywhere - and what lets the acceptance test, which *does*
//! read them off disk, be a test of the thing that ships.

use game_console::Embedded;

/// Every file the release needs, in the language `spec/console.md` describes.
pub fn library() -> Embedded {
    Embedded::of(&[
        ("setup", include_str!("../../../commands/setup.4x")),
        ("nodes", include_str!("../../../commands/nodes.4x")),
        ("forces", include_str!("../../../commands/forces.4x")),
        ("play", include_str!("../../../commands/play.4x")),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_console::Library;

    /// Every command file the release needs is carried in the binary, so a browser build
    /// can run exactly what a desktop build runs.
    #[test]
    fn the_release_command_files_travel_with_the_binary() {
        let library = library();
        for name in ["setup", "nodes", "forces", "play"] {
            assert!(library.fetch(name).is_some(), "`{name}` is not embedded");
        }
        assert!(
            library.fetch("setup").unwrap().contains("run nodes"),
            "setup should call its subroutines"
        );
    }

    /// The files carried in the binary are the files on disk, byte for byte.
    ///
    /// `include_str!` guarantees that at build time, and this says so at test time: it is
    /// the reason `tests/first_release.rs` reading them off disk is a test of what ships.
    #[test]
    fn what_is_carried_is_what_is_on_disk() {
        let library = library();
        for name in ["setup", "nodes", "forces", "play"] {
            let path = format!(
                "{}/../../commands/{name}.4x",
                env!("CARGO_MANIFEST_DIR").replace('\\', "/")
            );
            let disk = std::fs::read_to_string(&path)
                .unwrap_or_else(|why| panic!("cannot read {path}: {why}"));
            assert_eq!(library.fetch(name).unwrap(), disk, "`{name}` has drifted");
        }
    }
}
