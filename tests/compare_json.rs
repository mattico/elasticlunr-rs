extern crate elasticlunr;

#[test]
fn compare_json() {
    let mut index_builder = elasticlunr::IndexBuilder::new();
    index_builder.add_document("Oracle released its latest database Oracle 12g",
        "Yestaday Oracle has released its new database Oracle 12g, this would make more money for this company and lead to a nice profit report of annual year.");
    index_builder.add_document("Oracle released its profit report of 2015",
        "As expected, Oracle released its profit report of 2015, during the good sales of database and hardware, Oracle's profit of 2015 reached 12.5 Billion.");
    assert_eq!(index_builder.to_json(), include_str!("example.json"));
}