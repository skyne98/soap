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
        let prev_value = state.get("value").unwrap_or(Field::Value(Value::from(0)));
        let prev_value = match prev_value {
            Field::Value(value) => match value {
                Value::Number(num) => num.as_i64().unwrap_or(0),
                _ => 0,
            },
            _ => 0,
        };

        vec![
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(1)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 1))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(2)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 2))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(4)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 4))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(8)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 8))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(16)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 16))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(32)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 32))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(64)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 64))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(128)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 128))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(256)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 256))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(512)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 512))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(1024)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 1024))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(2048)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 2048))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(4096)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 4096))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(8192)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value + 8192))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(16384)),
                    result: state
                        .with_field("value", Field::Value(Value::from(prev_value + 16384))),
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
        let prev_value = state.get("value").unwrap_or(Field::Value(Value::from(0)));
        let prev_value = match prev_value {
            Field::Value(value) => match value {
                Value::Number(num) => num.as_i64().unwrap_or(0),
                _ => 0,
            },
            _ => 0,
        };

        vec![
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(1)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 1))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(2)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 2))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(4)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 4))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(8)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 8))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(16)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 16))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(32)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 32))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(64)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 64))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(128)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 128))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(256)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 256))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(512)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 512))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(1024)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 1024))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(2048)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 2048))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(4096)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 4096))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(8192)),
                    result: state.with_field("value", Field::Value(Value::from(prev_value - 8192))),
                },
                1,
            ),
            (
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from(16384)),
                    result: state
                        .with_field("value", Field::Value(Value::from(prev_value - 16384))),
                },
                1,
            ),
        ]
    }
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let start = State::new().with_field("value", Field::Value(Value::from(0)));
    let goal = start.with_field("value", Field::Value(Value::from(1_000_000_000)));
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
