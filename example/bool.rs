use anyhow::Result;
use serde_json::Value;
use soap::{
    action::{Action, Consequence},
    field::Field,
    goal::Goal,
    planner::plan,
    requirement::{CompareRequirement, Requirement},
    state::State,
};

pub struct SetTrueAction {}

impl Action for SetTrueAction {
    fn key(&self) -> String {
        "set_true".to_owned()
    }

    fn prepare(&self, state: &State) -> State {
        let mut prepared_state = state.clone();
        if prepared_state.contains_key("done") == false {
            prepared_state = prepared_state.with_field("done", Field::from(false));
        }
        prepared_state
    }

    fn options(&self, state: &State) -> Vec<(Consequence, u64)> {
        vec![(
            Consequence {
                action: self.key(),
                argument: None,
                result: state.with_field("done", Field::from(true)),
            },
            1,
        )]
    }
}

fn main() -> Result<()> {
    let start = State::new().with_field("done", Field::from(false));
    let goal = Goal::new().with_req(
        "done",
        Box::new(CompareRequirement::Equals(Field::from(true))),
    );
    let actions: Vec<Box<dyn Action>> = vec![Box::new(SetTrueAction {})];

    println!("Start: {:#?}", start);
    println!("Goal: {:#?}", goal);
    println!("-------------------------------------");
    let plan = plan(&start, &actions[..], &goal);
    println!("Plan: {:#?}", plan);

    Ok(())
}
