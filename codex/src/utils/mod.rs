use md5;


pub fn hash_nt(nt_string: &str) -> String {
  print!("{}\n", nt_string);

  return format!("{:?}", md5::compute(nt_string.as_bytes()));

}



#[cfg(test)]

mod test {

  use super::*;
  use sophia::ns::Namespace;
  use sophia::graph::Graph;
  use sophia::graph::{*, inmem::FastGraph};
  use sophia::serializer::{Stringifier, TripleSerializer};
  use sophia::serializer::nt::NtSerializer;

  #[test]
  fn expect_to_hash_triple() {
    let mut graph: FastGraph = FastGraph::new();
    let ex = Namespace::new("https://example.com/").unwrap();
    graph.insert(&ex.get("Bob").unwrap(), &ex.get("knows").unwrap(), &ex.get("Alice").unwrap()).unwrap();
    let mut nt_stringifier = NtSerializer::new_stringifier();

    let triple = graph.triples();
    let nt_string = nt_stringifier.serialize_triples(triple).unwrap().as_str();
    let hash = hash_nt(nt_string);

    assert_eq!(hash, "e89f8164c25701b72e11c3313aecc758")
  }

}