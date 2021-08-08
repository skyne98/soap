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

// Chop wood
pub struct ChopAction {}

impl Action for ChopAction {
    fn key(&self) -> String {
        "chop".to_owned()
    }

    fn prepare(&self, state: &State) -> State {
        let mut prepared_state = state.clone();
        if prepared_state.contains_key("wood") == false {
            prepared_state = prepared_state.with_field("wood", Field::from(0u64));
        }
        if prepared_state.contains_key("axe") == false {
            prepared_state = prepared_state.with_field("axe", Field::from(false));
        }
        prepared_state
    }

    fn options(&self, state: &State) -> Vec<(Consequence, u64)> {
        let wood = state.get_as_u64("wood").unwrap_or(0);
        let axe = state.get_as_bool("axe").unwrap_or(false);

        if axe {
            vec![
                (
                    Consequence {
                        action: self.key(),
                        argument: None,
                        result: state.with_field("wood", Field::from(wood + 2)),
                    },
                    1,
                ),
                (
                    Consequence {
                        action: self.key(),
                        argument: None,
                        result: state.with_field("wood", Field::from(wood + 8)),
                    },
                    4,
                ),
            ]
        } else {
            vec![]
        }
    }
}

// Collect
pub struct CollectAction {}

impl Action for CollectAction {
    fn key(&self) -> String {
        "collect".to_owned()
    }

    fn prepare(&self, state: &State) -> State {
        let mut prepared_state = state.clone();
        if prepared_state.contains_key("shrooms") == false {
            prepared_state = prepared_state.with_field("shrooms", Field::from(0u64));
        }
        prepared_state
    }

    fn options(&self, state: &State) -> Vec<(Consequence, u64)> {
        let shrooms = state.get_as_u64("shrooms").unwrap_or(0);

        vec![(
            Consequence {
                action: self.key(),
                argument: Some(Value::from("shrooms")),
                result: state.with_field("shrooms", Field::from(shrooms + 1)),
            },
            1,
        )]
    }
}

// Buy
pub struct BuyAction {}

impl Action for BuyAction {
    fn key(&self) -> String {
        "buy".to_owned()
    }

    fn prepare(&self, state: &State) -> State {
        let mut prepared_state = state.clone();
        if prepared_state.contains_key("coins") == false {
            prepared_state = prepared_state.with_field("coins", Field::from(0u64));
        }
        if prepared_state.contains_key("axe") == false {
            prepared_state = prepared_state.with_field("axe", Field::from(false));
        }
        if prepared_state.contains_key("shrooms") == false {
            prepared_state = prepared_state.with_field("shrooms", Field::from(0u64));
        }
        if prepared_state.contains_key("wood") == false {
            prepared_state = prepared_state.with_field("wood", Field::from(0u64));
        }
        prepared_state
    }

    fn options(&self, state: &State) -> Vec<(Consequence, u64)> {
        let coins = state.get_as_u64("coins").unwrap_or(0);

        let mut consequences = vec![];
        if coins > 2 {
            consequences.push((
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from("axe")),
                    result: state
                        .with_field("axe", Field::from(true))
                        .with_field("coins", Field::from(coins - 2)),
                },
                1,
            ));
        }

        consequences
    }
}

// Sell
pub struct SellAction {}

impl Action for SellAction {
    fn key(&self) -> String {
        "sell".to_owned()
    }

    fn prepare(&self, state: &State) -> State {
        let mut prepared_state = state.clone();
        if prepared_state.contains_key("coins") == false {
            prepared_state = prepared_state.with_field("coins", Field::from(0u64));
        }
        if prepared_state.contains_key("axe") == false {
            prepared_state = prepared_state.with_field("axe", Field::from(false));
        }
        if prepared_state.contains_key("shrooms") == false {
            prepared_state = prepared_state.with_field("shrooms", Field::from(0u64));
        }
        if prepared_state.contains_key("wood") == false {
            prepared_state = prepared_state.with_field("wood", Field::from(0u64));
        }
        prepared_state
    }

    fn options(&self, state: &State) -> Vec<(Consequence, u64)> {
        let coins = state.get_as_u64("coins").unwrap_or(0);
        let wood = state.get_as_u64("wood").unwrap_or(0);
        let shrooms = state.get_as_u64("shrooms").unwrap_or(0);
        let axe = state.get_as_bool("axe").unwrap_or(false);

        let mut consequences = vec![];
        if wood > 1 {
            consequences.push((
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from("wood")),
                    result: state
                        .with_field("wood", Field::from(wood - 1))
                        .with_field("coins", Field::from(coins + 3)),
                },
                1,
            ));
            consequences.push((
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from("wood")),
                    result: state
                        .with_field("wood", Field::from(0u64))
                        .with_field("coins", Field::from(coins + wood * 3)),
                },
                1,
            ));
        }
        if shrooms > 1 {
            consequences.push((
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from("shrooms")),
                    result: state
                        .with_field("shrooms", Field::from(shrooms - 1))
                        .with_field("coins", Field::from(coins + 1)),
                },
                1,
            ));
            consequences.push((
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from("shrooms")),
                    result: state
                        .with_field("shrooms", Field::from(0u64))
                        .with_field("coins", Field::from(coins + shrooms)),
                },
                1,
            ));
        }
        if axe {
            consequences.push((
                Consequence {
                    action: self.key(),
                    argument: Some(Value::from("axe")),
                    result: state
                        .with_field("axe", Field::from(false))
                        .with_field("coins", Field::from(coins + 1)),
                },
                1,
            ));
        }

        consequences
    }
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let start = State::new();
    let goal = Goal::new()
        .with_req(
            "coins",
            Box::new(CompareRequirement::MoreThanEquals(Field::from(10u64))),
        )
        .with_req(
            "wood",
            Box::new(CompareRequirement::Equals(Field::from(10u64))),
        )
        .with_req(
            "shrooms",
            Box::new(CompareRequirement::Equals(Field::from(2u64))),
        );
    let actions: Vec<Box<dyn Action>> = vec![
        Box::new(ChopAction {}),
        Box::new(BuyAction {}),
        Box::new(SellAction {}),
        Box::new(CollectAction {}),
    ];

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
