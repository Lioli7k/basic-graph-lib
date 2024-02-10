use std::{fs, path::PathBuf};

use basic_graph_lib::{Graph, GraphId};

fn main() {
    let matches = clap::command!()
        .about("Traverses graph from provided TGF file and starting node")
        .arg(clap::arg!(<file> "Path to TGF file").value_parser(clap::value_parser!(PathBuf)))
        .arg(
            clap::arg!([source] "Starting node ID")
                .value_parser(clap::value_parser!(GraphId))
                .default_value("1"),
        )
        .arg_required_else_help(true)
        .get_matches();

    let path = matches.get_one::<PathBuf>("file").expect("required");
    let id = *matches
        .get_one::<GraphId>("source")
        .expect("has default value");

    if let Err(e) = traverse_graph(path, id) {
        eprintln!("{e}");
    }
}

fn traverse_graph(file: &PathBuf, id: GraphId) -> Result<(), String> {
    let graph: Graph<String> = fs::read_to_string(file)
        .map_err(|e| format!("Failed to read graph file: {e}"))?
        .parse()
        .map_err(|e| format!("Failed to parse graph: {e}"))?;
    graph.bfs(id);

    Ok(())
}
