extern crate handlebars;

use handlebars::Handlebars;
// use serde_json::json;
use std::collections::HashMap;
use sophia::graph::{*, inmem::FastGraph};
// use utils;


// pub fn store_to_graph_pattern() {

// }


/// Create an update query from add, delete and graph pattern options.
/// All three of these variables are optional, (but with out an add or delete, nothing will happen with the resulting query)
/// The add string goes into the `INSERT {}` clause, the delete string goes into the `DELETE {}` clause, and the graph_pattern
/// goes into the graph pattern clause.
///
/// All three arguments must be valid graph patterns to be inserted in a sparql query.
///
/// # Examples
///
/// ```
/// let add = Some("<a> <b> <c> .");
/// let delete = Some("<d> <e> <f> .");
/// let query = update_query(add, delete, None);
/// assert_eq!(query, "\nDELETE {\n  <d> <e> <f> .\n}\nINSERT {\n  <a> <b> <c> .\n}\nWHERE {\n  \n}".to_string())
/// ```
///
pub fn update_query(add: Option<&str>, delete: Option<&str>, graph_pattern: Option<&str>) -> String {
  let mut handlebars = Handlebars::new();
  handlebars.register_template_file("insert", "./src/db/queries/insert.sparql").unwrap();

  let mut replacement_dic  = HashMap::new();
  if let Some(value) = add {
    replacement_dic.insert("insert_triples".to_string(), value.to_owned());
  }
  if let Some(value) = delete {
    replacement_dic.insert("delete_triples".to_string(), value.to_owned());
  }
  if let Some(value) = graph_pattern {
    replacement_dic.insert("graph_pattern".to_string(), value.to_owned());
  }

  let res = handlebars.render("insert", &replacement_dic).unwrap();
  return res;
}

#[cfg(test)]
mod tests {
  use sophia::ns::Namespace;
use sophia::serializer::TripleSerializer;
use sophia::serializer::nt::NtSerializer;
use sophia::serializer::Stringifier;


use super::*;

  // Testing Sophia
  #[test]
  fn create_graph_vec() {
    let mut graph: FastGraph = FastGraph::new();
    let ex = Namespace::new("https://example.com/").unwrap();
    graph.insert(&ex.get("Bob").unwrap(), &ex.get("knows").unwrap(), &ex.get("Alice").unwrap()).unwrap();

    let result = graph.triples();
    let mut nt_string = NtSerializer::new_stringifier();
    let example = nt_string.serialize_triples(result).unwrap().as_str();
    assert_eq!(example, "<https://example.com/Bob> <https://example.com/knows> <https://example.com/Alice>.\n");

  }


  // testing query insertion
  #[test]
  fn it_works() {
    assert_eq!( 2 + 2, 4);
  }

  #[test]
  fn expect_insert_with_add() {
    let add = Some("hello");
    let ret = update_query(add, None, None);
    let expected = "\nDELETE {\n  \n}\nINSERT {\n  hello\n}\nWHERE {\n  \n}".to_string();
    assert_eq!(ret, expected);
  }

  #[test]
  fn expect_all_three() {
    let add = Some("hello");
    let delete = Some("world");
    let graph_pattern = Some("!");
    let ret = update_query(add, delete, graph_pattern);
    assert_eq!(ret, "\nDELETE {\n  world\n}\nINSERT {\n  hello\n}\nWHERE {\n  !\n}".to_string());
  }

}