pub mod graph {

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub mod graph_items {

        pub mod node {
            use std::collections::HashMap;

            #[derive(Default, Debug, PartialEq, Eq, Clone)]
            pub struct Node<'a> {
                pub name: &'a str,
                attrs: HashMap<String, String>,
            }

            impl<'a> Node<'a> {
                pub fn new(name: &'a str) -> Node<'a> {
                    Node {
                        name,
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Node<'a> {
                    for (key, value) in attrs {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| &s[..])
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Default, Debug, PartialEq, Eq, Clone)]
            pub struct Edge<'a> {
                pub x: &'a str,
                pub y: &'a str,
                attrs: HashMap<String, String>,
            }

            impl<'a> Edge<'a> {
                pub fn new(x: &'a str, y: &'a str) -> Edge<'a> {
                    Edge {
                        x,
                        y,
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Edge<'a> {
                    for (key, value) in attrs {
                        self.attrs.insert(key.to_string(), value.to_string());
                    }

                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| &s[..])
                }
            }
        }
    }

    #[derive(Default, Debug)]
    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Graph<'a> {
            for (key, value) in attrs {
                self.attrs.insert(key.to_string(), value.to_string());
            }

            self
        }

        pub fn with_nodes(mut self, nodes: &[Node<'a>]) -> Graph<'a> {
            for node in nodes {
                self.nodes.push(node.clone());
            }

            self
        }

        pub fn with_edges(mut self, edges: &[Edge<'a>]) -> Graph<'a> {
            for edge in edges {
                self.edges.push(edge.clone())
            }

            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node<'a>> {
            self.nodes.iter().find(|&node| node.name == name)
        }

        pub fn get_edge(&self, x: &str, y: &str) -> Option<&Edge<'a>> {
            self.edges.iter().find(|&edge| edge.x == x && edge.y == y)
        }

        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| &s[..])
        }
    }
}
