use clap::Parser;

use crate::prelude::*;

use super::Algorithm;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args{
    ///Input image filename
    #[arg(short, long, required = true)]
    pub filename: String,

    ///Pathfinding algorithm
    #[arg(value_enum, short, long, default_value_t = Algorithm::Dijkstra)]
    pub algorithm: Algorithm,

    ///Output file
    #[arg(short, long, default_value_t = DEFAULT_OUTPUT_NAME.to_string())]
    pub output_file: String,

    ///Activate stdout logging
    #[arg(short, long, default_value_t = false)]
    pub logging: bool,

    ///Draw a wider path
    #[arg(short, long, default_value_t = false)]
    pub wider: bool,

    ///Show the nodes that have been seen
    #[arg(short, long, default_value_t = false)]
    pub seen: bool,
}