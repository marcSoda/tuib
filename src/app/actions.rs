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
    Scale(u8),
}

impl Action {
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 17] = [
            Action::Quit,
            Action::MoveRight,
            Action::MoveLeft,
            Action::MoveUp,
            Action::MoveDown,
            Action::TabRight,
            Action::TabLeft,
            Action::Scale(1),
            Action::Scale(2),
            Action::Scale(3),
            Action::Scale(4),
            Action::Scale(5),
            Action::Scale(6),
            Action::Scale(7),
            Action::Scale(8),
            Action::Scale(9),
            Action::Scale(0),
        ];
        ACTIONS.iter()
    }

    pub fn keys(&self) -> Vec<Key> {
        match self {
            Action::Quit => vec![Key::Char('q')],
            Action::MoveRight => vec![Key::Char('l'), Key::Right],
            Action::MoveLeft => vec![Key::Char('h'), Key::Left],
            Action::MoveUp => vec![Key::Char('k'), Key::Up],
            Action::MoveDown => vec![Key::Char('j'), Key::Down],
            Action::TabRight => vec![Key::Char('L'), Key::Char('.'), Key::Char('>')],
            Action::TabLeft => vec![Key::Char('H'), Key::Char(','), Key::Char('<')],
            Action::Scale(n) => vec![Key::Char(('0' as u8 + n) as char)],
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Quit => write!(f, "Quit"),
            Action::MoveRight => write!(f, "MoveRight"),
            Action::MoveLeft => write!(f, "MoveLeft"),
            Action::MoveUp => write!(f, "MoveUp"),
            Action::MoveDown => write!(f, "MoveDown"),
            Action::TabRight => write!(f, "TabRight"),
            Action::TabLeft => write!(f, "TabLeft"),
            Action::Scale(n) => write!(f, "Scale{}", n),
        }
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
