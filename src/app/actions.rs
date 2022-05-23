use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;
use crate::inputs::key::Key;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
    TabRight,
    TabLeft,
}

impl Action {
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 7] = [
            Action::Quit,
            Action::MoveRight,
            Action::MoveLeft,
            Action::MoveUp,
            Action::MoveDown,
            Action::TabRight,
            Action::TabLeft,
        ];
        ACTIONS.iter()
    }

    pub fn keys(&self) -> &[Key] {
        match self {
            Action::Quit => &[Key::Char('q')],
            Action::MoveRight => &[Key::Char('l')],
            Action::MoveLeft => &[Key::Char('h')],
            Action::MoveUp => &[Key::Char('k')],
            Action::MoveDown => &[Key::Char('j')],
            Action::TabRight => &[Key::Char('L')],
            Action::TabLeft => &[Key::Char('H')],
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Action::Quit => "Quit",
            Action::MoveRight => "MoveRight",
            Action::MoveLeft => "MoveLeft",
            Action::MoveUp => "MoveUp",
            Action::MoveDown => "MoveDown",
            Action::TabRight => "TabRight",
            Action::TabLeft => "TabLeft",
        };
        write!(f, "{}", str)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    pub fn find(&self, key: Key) -> Option<&Action> {
        Action::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    pub fn actions(&self) -> &[Action] {
        self.0.as_slice()
    }
}

impl From<Vec<Action>> for Actions {
    fn from(actions: Vec<Action>) -> Self {
        let mut map: HashMap<Key, Vec<Action>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }
        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1)
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(Action::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Conflict key {} with actions {}", key, actions)
            })
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            panic!("{}", errors.join("; "))
        }
        Self(actions)
    }
}
