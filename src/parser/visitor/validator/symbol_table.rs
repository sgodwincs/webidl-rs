use petgraph::Direction;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::cell::Cell;
use std::collections::HashMap;

use parser::ast::Identifier;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Symbol {
    pub name: Identifier,
    pub type_: SymbolType,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SymbolType {
    Argument,
    Callback,
    CallbackInterface,
    Const,
    Enum,
    ExtendedAttributeArgumentList,
    Iterable,
    Maplike,
    NonPartialDictionary,
    NonPartialInterface,
    NonPartialNamespace,
    PartialDictionary,
    PartialNamespace,
    PartialInterface,
    RegularAttribute,
    RegularOperation,
    Setlike,
    SpecialOperation,
    StaticAttribute,
    StaticOperation,
    StringifierAttribute,
    StringifierOperation,
    Typedef,
}

type SymbolNode = HashMap<Identifier, Symbol>;

#[derive(Clone, Debug)]
pub struct SymbolTable {
    current_scope: Cell<NodeIndex>,
    scope_tree: Graph<SymbolNode, Identifier>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut scope_tree = Graph::<SymbolNode, Identifier>::new();
        let current_scope = Cell::new(scope_tree.add_node(HashMap::new()));

        SymbolTable {
            current_scope,
            scope_tree,
        }
    }

    pub fn create_scope(&mut self, name: Identifier) {
        assert!(!self.does_child_scope_exist(&*name),
                "Cannot create scope that already exists");

        let new_scope = self.scope_tree.add_node(HashMap::new());
        self.scope_tree
            .add_edge(self.current_scope.get(), new_scope, name);
    }

    pub fn does_child_scope_exist(&self, name: &str) -> bool {
        self.find_child_scope(name, self.current_scope.get())
            .is_some()
    }

    pub fn enter_scope(&self, name: &str) {
        let child_scope = self.find_child_scope(name, self.current_scope.get())
            .expect("Cannot enter scope that does not exist");
        self.current_scope.set(child_scope);
    }

    pub fn exit_scope(&self) {
        let parent_scope = self.get_parent_scope(self.current_scope.get())
            .expect("Cannot exit out of global scope");
        self.current_scope.set(parent_scope);
    }

    pub fn lookup_local_symbol(&self, name: &str) -> Option<&Symbol> {
        self.get_symbol_node(self.current_scope.get()).get(name)
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        let mut current_scope = self.current_scope.get();

        loop {
            let symbol_node = self.get_symbol_node(current_scope);

            if let Some(symbol) = symbol_node.get(name) {
                return Some(symbol);
            }

            match self.get_parent_scope(current_scope) {
                Some(parent_scope) => current_scope = parent_scope,
                None => return None,
            }
        }
    }

    pub fn insert_symbol(&mut self, name: Identifier, symbol: Symbol) -> Option<Symbol> {
        let current_scope = self.current_scope.get();
        let symbol_node = self.get_symbol_node_mut(current_scope);
        symbol_node.insert(name, symbol)
    }

    fn find_child_scope(&self, name: &str, scope: NodeIndex) -> Option<NodeIndex> {
        let edge = self.scope_tree
            .edges_directed(scope, Direction::Outgoing)
            .find(|edge| edge.weight() == name);

        edge.map(|edge| edge.target())
    }

    fn get_parent_scope(&self, scope: NodeIndex) -> Option<NodeIndex> {
        let edge = self.scope_tree
            .edges_directed(scope, Direction::Incoming)
            .nth(0);

        edge.map(|edge| edge.target())
    }

    fn get_symbol_node(&self, scope: NodeIndex) -> &SymbolNode {
        self.scope_tree
            .node_weight(scope)
            .expect("\"scope\" must reference an existing symbol node")
    }

    fn get_symbol_node_mut(&mut self, scope: NodeIndex) -> &mut SymbolNode {
        self.scope_tree
            .node_weight_mut(scope)
            .expect("\"scope\" must reference an existing symbol node")
    }
}
