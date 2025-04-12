pub mod definition;
pub mod handler;

use crate::commands::common::CommandMeta;

pub const META: CommandMeta = CommandMeta {
  name: "seq-compare",
  aliases: &["seqcomp", "seq-comp"],
  about: "Compare biological sequences",
};
