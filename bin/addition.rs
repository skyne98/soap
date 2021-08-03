use anyhow::Result;
use serde_json::{Number, Value};
use soap::{
    action::{Action, Consequence},
    field::Field,
    planner::plan,
    state::State,
};

// Add
pub struct AddAction {}

impl Action for AddAction {
    fn key(&self) -> String {
        "addition".to_owned()
    }

    fn options(&self, state: &State) -> Vec<(Consequence, u64)> {
        let prev_value = state.get_as_i64("value").unwrap_or(0);

        vec![
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(1)),
                    result: state.with_field("value", Field::from(prev_value + 1)),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(2)),
                    result: state.with_field("value", Field::from(prev_value + 2)),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(4)),
                    result: state.with_field("value", Field::from(prev_value + 4)),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(8)),
                    result: state.with_field("value", Field::from(prev_value + 8)),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(16)),
                    result: state.with_field("value", Field::from(prev_value + 16)),
                },
                1,
            ),
        ]
    }
}

// Subtract
pub struct SubAction {}

impl Action for SubAction {
    fn key(&self) -> String {
        "subtraction".to_owned()
    }

    fn options(&self, state: &State) -> Vec<(Consequence, u64)> {
        let prev_value = state.get_as_i64("value").unwrap_or(0);

        vec![
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(1)),
                    result: state.with_field("value", Field::from(prev_value - 1)),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(2)),
                    result: state.with_field("value", Field::from(prev_value - 2)),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(4)),
                    result: state.with_field("value", Field::from(prev_value - 4)),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(8)),
                    result: state.with_field("value", Field::from(prev_value - 8)),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(16)),
                    result: state.with_field("value", Field::from(prev_value - 16)),
                },
                1,
            ),
        ]
    }
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let start = State::new().with_field("value", Field::from(0i64));
    let goal = start.with_field("value", Field::Value(Value::from(50)));
    let actions: Vec<Box<dyn Action>> = vec![Box::new(AddAction {}), Box::new(SubAction {})];

    println!("Start: {:#?}", start);
    println!("Goal: {:#?}", goal);
    println!("-------------------------------------");
    let start_time = std::time::Instant::now();
    let plan = plan(&start, &actions[..], &goal);
    let done_in = std::time::Instant::now().duration_since(start_time);
    println!("Plan: {:#?}", plan);
    println!("Done in {} ms", done_in.as_millis());

    Ok(())
}
