use anyhow::Result;
use serde_json::Value;
use soap::{
    action::{Action, Consequence},
    field::Field,
    planner::plan,
    state::State,
};

pub struct SetTrueAction {}

impl Action for SetTrueAction {
    fn key(&self) -> String {
        "set_true".to_owned()
    }

    fn options(&self, state: &State) -> Vec<(Consequence, u64)> {
        vec![(
            Consequence {
                action: self.key(),
                argument: None,
                result: state.with_field("done", Field::Value(Value::Bool(true))),
            },
            0,
        )]
    }
}

fn main() -> Result<()> {
    let start = State::new().with_field("done", Field::Value(Value::Bool(false)));
    let goal = start.with_field("done", Field::Value(Value::Bool(true)));
    let actions: Vec<Box<dyn Action>> = vec![Box::new(SetTrueAction {})];

    println!("Start: {:#?}", start);
    println!("Goal: {:#?}", goal);
    println!("-------------------------------------");
    let plan = plan(&start, &actions[..], &goal);
    println!("Plan: {:#?}", plan);

    Ok(())
}
