use anyhow::Result;
use serde_json::{Number, Value};
use soap::{
    action::{Action, Consequence},
    field::Field,
    goal::Goal,
    planner::plan,
    requirement::CompareRequirement,
    state::State,
};

// Add
pub struct AddAction {}

impl Action for AddAction {
    fn key(&self) -> String {
        "addition".to_owned()
    }

    fn prepare(&self, state: &State) -> State {
        let mut prepared_state = state.clone();
        if prepared_state.contains_key("value") == false {
            prepared_state = prepared_state.with_field("value", Field::from(0u64));
        }
        prepared_state
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

    fn prepare(&self, state: &State) -> State {
        let mut prepared_state = state.clone();
        if prepared_state.contains_key("value") == false {
            prepared_state = prepared_state.with_field("value", Field::from(0u64));
        }
        prepared_state
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
    let goal = Goal::new().with_req(
        "value",
        Box::new(CompareRequirement::Equals(Field::Value(Value::from(50)))),
    );
    let actions: Vec<Box<dyn Action>> = vec![Box::new(AddAction {}), Box::new(SubAction {})];

    println!("Start: {:#?}", start);
    println!("Goal: {:#?}", goal);
    println!("-------------------------------------");
    let start_time = std::time::Instant::now();
    let plan = plan(&start, &actions[..], &goal);
    let done_in = std::time::Instant::now().duration_since(start_time);
    println!("Plan: {:#?}", plan);
    println!(
        "Done in {} ms ({} μs)",
        done_in.as_millis(),
        done_in.as_micros()
    );

    Ok(())
}
