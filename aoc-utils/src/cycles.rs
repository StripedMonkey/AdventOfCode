use std::hash::Hash;

/// Marker trait for values which can be cached
pub trait CacheableState: Eq + Hash {}

impl<T> CacheableState for T where T: Eq + Hash {}

pub trait CacheableSequence {
    type State: CacheableState;

    fn cycle_len(&self) -> usize;

    // Get the Nth step in the sequence, calculating if necessary
    fn until_n(&mut self, step_n: usize) -> &Self::State;
}

pub struct SequenceDetector<State, F>
where
    State: CacheableState,
    F: Fn(&State) -> State,
{
    /// The currently cached states which is being walked
    states: Vec<State>,
    /// A function which may derive the next state, this is what gets cached
    get_next_state: F,
    /// The index of the first state detected in a cycle
    detected_cycle_start: Option<usize>,
}

impl<State, F> SequenceDetector<State, F>
where
    State: CacheableState,
    F: Fn(&State) -> State,
{
    pub fn new(initial: State, next_state: F) -> Self {
        Self {
            states: vec![initial],
            get_next_state: next_state,
            detected_cycle_start: None,
        }
    }

    pub fn until_n(&mut self, step_n: usize) -> &State {
        while step_n > self.states.len() {
            let next_state = (self.get_next_state)(self.states.last().unwrap());
            if let Some((idx, _state)) = self
                .states
                .iter()
                .enumerate()
                .rev()
                .find(|(_, s)| **s == next_state)
            {
                self.detected_cycle_start = Some(idx);
                break;
            }
        }
        let Some(cycle_start) = self.detected_cycle_start else {
            return &self.states[step_n];
        };
        let cycle_length = self.states.len() - cycle_start;
        let step_idx = cycle_start + ((step_n - cycle_start) % cycle_length);
        &self.states[step_idx]
    }
}
