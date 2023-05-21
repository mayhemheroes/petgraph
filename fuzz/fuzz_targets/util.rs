use arbitrary::{Arbitrary, Unstructured};
use petgraph::{Directed, Graph};

// To avoid running out of memory
const MAX_NODES: u32 = 1000u32;

#[derive(Debug)]
struct Edge(u32, u32, u8);

impl<'a> Arbitrary<'a> for Edge {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Edge(
            u.int_in_range(0..=MAX_NODES - 1)?,
            u.int_in_range(0..=MAX_NODES - 1)?,
            u.arbitrary()?,
        ))
    }
}

#[derive(Arbitrary, Debug)]
pub struct Edges(Vec<Edge>);

pub fn to_graph(edges: Edges) -> Graph<u32, u8, Directed> {
    let Edges(inner) = edges;
    let edges: Vec<(u32, u32, u8)> = inner
        .into_iter()
        .map(|Edge(from, to, weight)| (from, to, weight))
        .collect();
    Graph::<u32, u8>::from_edges(&edges)
}

// impl<'a> Arbitrary<'a> for Edges {
//     fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
//         let edges: Vec<(u32, u32, u8)> = u
//             .arbitrary_take_rest_iter::<Edge>()?
//             .map(|e| {
//             let Edge(from, to, weight) = e.unwrap();
//             (from, to, weight)
//         })
//         .collect();

//         Ok(Edges(edges))
//     }
// }
