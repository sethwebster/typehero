use std::collections::HashMap;

enum State {
    Idle,
    Processing,
    Done,
    Failed,
}

enum Event {
    Start,
    Complete,
    Error,
}

struct StateMachine {
    state: State,
    transitions: HashMap<(String, String), State>,
}

impl StateMachine {
    fn new() -> Self {
        let mut transitions = HashMap::new();
        transitions.insert(("Idle".into(), "Start".into()), State::Processing);
        transitions.insert(("Processing".into(), "Complete".into()), State::Done);
        transitions.insert(("Processing".into(), "Error".into()), State::Failed);

        Self {
            state: State::Idle,
            transitions,
        }
    }

    fn transition(&mut self, event: &Event) -> Result<(), String> {
        let current = state_name(&self.state);
        let event_name = event_name(event);

        if let Some(next_state) = self.transitions.get(&(current.clone(), event_name)) {
            self.state = next_state;
            Ok(())
        } else {
            Err(format!("Invalid transition from {} with event {}", current, event_name))
        }
    }
}

fn state_name(state: &State) -> String {
    match state {
        State::Idle => "Idle".into(),
        State::Processing => "Processing".into(),
        State::Done => "Done".into(),
        State::Failed => "Failed".into(),
    }
}

fn event_name(event: &Event) -> String {
    match event {
        Event::Start => "Start".into(),
        Event::Complete => "Complete".into(),
        Event::Error => "Error".into(),
    }
}
