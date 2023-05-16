use std::env;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let current_dir = Path::new(file!()).parent().unwrap();
    let include_dir = current_dir.join("../../../../third-party/YarnSpinner/YarnSpinner");
    let proto_file = include_dir.join("yarn_spinner.proto");
    let output_dir = current_dir.join("../../../core/src/generated");
    env::set_var("OUT_DIR", output_dir);

    prost_build::Config::new()
        .type_attribute(
            ".",
            "use crate::prelude::*;\
             #[cfg_attr(feature = \"serde\", derive(Serialize, Deserialize))]\n\
             #[cfg_attr(feature = \"bevy\", derive(Reflect, FromReflect))]\n\
             #[cfg_attr(feature = \"bevy\", reflect(Debug, PartialEq))]\n\
             #[cfg_attr(\n\
                 all(feature = \"bevy\", feature = \"serde\"),\n\
                 reflect(Serialize, Deserialize)\n\
             )]",
        )
        .compile_protos(&[proto_file], &[include_dir])?;
    Ok(())
}
