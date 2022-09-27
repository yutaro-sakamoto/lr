use std::collections::HashSet;
use std::hash::Hash;

struct DFA<State: Eq + Hash + Copy, Alphabet: Eq> {
    final_state_set: HashSet<State>,
    first_state: State,
    transition: fn(State, &Alphabet) -> State
}

impl <State: Eq + Hash + Copy, Alphabet: Eq> DFA<State, Alphabet> {
    fn accept(&self, input: &Vec<Alphabet>) -> bool {
        let mut state: State = self.first_state;
        for a in input {
            state = (self.transition)(state, &a)
        }
        self.final_state_set.contains(&state)
    }
}

fn transition(state: char, input: &i32) -> char {
    match state {
        'p' => match input {
             0 => 'p',
             _ => 'q',
        },
        _ => 'q',
    }
}

fn main() {
    let final_state_set : HashSet<char> = vec!['q'].into_iter().collect();
    let first_state : char = 'p';
    let dfa: DFA<char, i32> = DFA {final_state_set: final_state_set, first_state: first_state, transition: transition};

    let input1 = vec![0, 0, 1];
    let input2 = vec![0, 0, 0, 0, 0];
    let input3 = vec![0, 1, 0, 0, 0];
    let input4 = vec![0, 1, 0, 1, 0];
    println!("result {}", dfa.accept(&input1));
    println!("result {}", dfa.accept(&input2));
    println!("result {}", dfa.accept(&input3));
    println!("result {}", dfa.accept(&input4));
}
