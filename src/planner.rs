use crate::{
    action::{Action, Consequence},
    goal::Goal,
    state::State,
};

#[derive(Clone, Eq, Hash, PartialEq)]
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

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Consequence(consequence) => consequence.fmt(f),
            Node::State(state) => state.fmt(f),
        }
    }
}

pub fn plan<'a>(
    start: &State,
    actions: &[Box<dyn Action + 'a>],
    goal: &Goal,
) -> Option<(Vec<Node>, u64)> {
    // Prepare the state
    let mut start = start.clone();
    for action in actions {
        start = action.prepare(&start);
    }
    // Plan
    let start_node = Node::State(start.clone());
    let result = pathfinding::prelude::astar(
        &start_node,
        |node| consequences(node, actions),
        |node| {
            debug!("----- Heuristic -----");
            debug!("To node: {:?}", node);
            node.state().distance_to_goal(goal)
        },
        |node| {
            debug!("-------------------");
            debug!("----- Success -----");
            debug!("-------------------");
            debug!("From node: {:?}", node);
            node.state().distance_to_goal(goal) == 0
        },
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
