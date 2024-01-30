use std::env;
use std::io::Result;
use yarnspinner_codegen::*;

fn main() -> Result<()> {
    let include_dir = path(ProjectPath::ThirdPersonYarnSpinner).join("YarnSpinner");
    let proto_file = include_dir.join("yarn_spinner.proto");
    let output_dir = path(ProjectPath::Core).join("src/generated");
    env::set_var("OUT_DIR", output_dir);

    prost_build::Config::new()
        .type_attribute(
            ".",
            "use crate::prelude::*;\
             #[cfg_attr(feature = \"serde\", derive(Serialize, Deserialize))]\n\
             #[cfg_attr(feature = \"bevy\", derive(Reflect))]\n\
             #[cfg_attr(feature = \"bevy\", reflect(Debug, PartialEq))]\n\
             #[cfg_attr(\n\
                 all(feature = \"bevy\", feature = \"serde\"),\n\
                 reflect(Serialize, Deserialize)\n\
             )]",
        )
        .compile_protos(&[proto_file], &[include_dir])?;
    Ok(())
}
