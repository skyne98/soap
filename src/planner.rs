use crate::{
    action::{Action, Consequence},
    state::State,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Node {
    Consequence(Consequence),
    State(State),
}

impl Node {
    pub fn state(&self) -> &State {
        match self {
            Node::Consequence(con) => &con.result,
            Node::State(state) => state,
        }
    }
}

pub fn plan<'a>(
    start: &State,
    actions: &[Box<dyn Action + 'a>],
    goal: &State,
) -> Option<(Vec<Node>, u64)> {
    let start_node = Node::State(start.clone());
    let result = pathfinding::prelude::astar(
        &start_node,
        |node| consequences(node, actions),
        |node| start.distance_to(node.state()),
        |node| node.state().distance_to(goal) == 0,
    );

    result
}

fn consequences<'a>(node: &Node, actions: &[Box<dyn Action + 'a>]) -> Vec<(Node, u64)> {
    actions
        .iter()
        .flat_map(|action| action.options(node.state()))
        .map(|(consequence, cost)| (Node::Consequence(consequence), cost))
        .collect()
}
