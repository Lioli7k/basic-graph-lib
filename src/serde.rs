use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use nom::{
    character::complete as cc,
    combinator,
    error::{Error as NError, ErrorKind, ParseError},
    multi, sequence, Finish, IResult,
};

use super::{Graph, GraphId};

impl<T> Graph<T> {
    pub fn serialize(&self) -> String
    where
        T: Display,
    {
        self.nodes
            .iter()
            .map(|(id, value)| format!("{id} {value}\n"))
            .chain(["#\n".to_string()])
            .chain(
                self.edges
                    .iter()
                    .map(|edge| format!("{} {}\n", edge.from, edge.to)),
            )
            .collect()
    }
}

impl<T: FromStr> FromStr for Graph<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        sequence::separated_pair(
            parse_pairs,
            sequence::delimited(cc::line_ending, cc::char('#'), cc::line_ending),
            parse_pairs,
        )(s)
        .finish()
        .map(|(_, (nodes, edges))| {
            let mut graph = Graph::new();
            for (id, value) in nodes {
                graph.add_node(id, value);
            }
            for (from, to) in edges {
                graph.add_edge(from, to);
            }

            graph
        })
        .map_err(|e| anyhow!("Parse error: {e}"))
    }
}

fn parse_pairs<T: FromStr>(s: &str) -> IResult<&str, Vec<(GraphId, T)>> {
    multi::separated_list0(
        cc::line_ending,
        sequence::separated_pair(
            cc::u64,
            cc::space1,
            combinator::map_parser(cc::not_line_ending, parse_value),
        ),
    )(s)
}

fn parse_value<T: FromStr>(s: &str) -> IResult<&str, T> {
    s.parse()
        .map(|value| ("", value))
        .map_err(|_| nom::Err::Failure(NError::from_error_kind(s, ErrorKind::Fail)))
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::Edge;

    use super::*;

    #[test]
    fn parse_value_string() {
        assert_eq!(
            parse_value::<String>("Test string"),
            Ok(("", String::from("Test string")))
        );
    }

    #[test]
    fn parse_value_i32_valid() {
        assert_eq!(parse_value::<i32>("-123456"), Ok(("", -123456)));
    }

    #[test]
    fn parse_value_i32_invalid() {
        assert_eq!(
            parse_value::<i32>("banana"),
            Err(nom::Err::Failure(NError::from_error_kind(
                "banana",
                ErrorKind::Fail,
            )))
        );
    }

    #[test]
    fn parse_pairs_empty() {
        assert_eq!(parse_pairs::<GraphId>(""), Ok(("", vec![])));
    }

    #[test]
    fn parse_pairs_one() {
        assert_eq!(
            parse_pairs::<GraphId>("1 2"),
            Ok(("", vec![(1 as GraphId, 2 as GraphId)]))
        );
    }

    #[test]
    fn parse_pairs_three() {
        assert_eq!(
            parse_pairs::<GraphId>("1 2\n3 2\n4 3"),
            Ok(("", vec![(1, 2), (3, 2), (4, 3)]))
        );
    }

    #[test]
    fn parse_pairs_invalid() {
        assert_eq!(
            parse_pairs::<GraphId>("1 2\n3 banana\n4 3"),
            Err(nom::Err::Failure(NError::from_error_kind(
                "banana",
                ErrorKind::Fail,
            )))
        );
    }

    #[test]
    fn parse_graph_simple() {
        let graph = include_str!("../test-data/test-graph-simple").parse::<Graph<String>>();
        assert!(graph.is_ok(), "Expected graph to parse");
        let graph = graph.unwrap();
        assert_eq!(
            graph.nodes,
            HashMap::from([
                (1, "First node".to_string()),
                (2, "Second node".to_string())
            ]),
            "Nodes don't match"
        );
        assert_eq!(
            graph.edges,
            HashSet::from([Edge { from: 1, to: 2 }]),
            "Edges don't match"
        );
    }

    #[test]
    fn parse_graph_complex() {
        let graph = include_str!("../test-data/test-graph").parse::<Graph<String>>();
        assert!(graph.is_ok(), "Expected graph to parse");
        let graph = graph.unwrap();
        assert_eq!(
            graph.nodes,
            HashMap::from([
                (1, "January".to_string()),
                (2, "March".to_string()),
                (3, "April".to_string()),
                (4, "May".to_string()),
                (5, "December".to_string()),
                (6, "June".to_string()),
                (7, "September".to_string())
            ]),
            "Nodes don't match"
        );
        assert_eq!(
            graph.edges,
            HashSet::from([
                Edge { from: 1, to: 2 },
                Edge { from: 3, to: 2 },
                Edge { from: 4, to: 3 },
                Edge { from: 5, to: 1 },
                Edge { from: 5, to: 3 },
                Edge { from: 6, to: 3 },
                Edge { from: 6, to: 1 },
                Edge { from: 7, to: 5 },
                Edge { from: 7, to: 6 },
                Edge { from: 7, to: 1 },
            ]),
            "Edges don't match"
        );
    }

    #[test]
    fn serialize_graph_simple() {
        let graph = include_str!("../test-data/test-graph-simple").parse::<Graph<String>>();
        assert!(graph.is_ok(), "Expected graph to parse");
        let graph = graph.unwrap();
        assert!(
            graph.serialize().parse::<Graph<String>>().is_ok(),
            "Expected serialized graph to parse"
        );
    }

    #[test]
    fn serialize_graph_complex() {
        let graph = include_str!("../test-data/test-graph").parse::<Graph<String>>();
        assert!(graph.is_ok(), "Expected graph to parse");
        let graph = graph.unwrap();
        assert!(
            graph.serialize().parse::<Graph<String>>().is_ok(),
            "Expected serialized graph to parse"
        );
    }
}
