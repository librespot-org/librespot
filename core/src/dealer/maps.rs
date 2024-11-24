use std::collections::HashMap;

use crate::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandlerMapError {
    #[error("request was already handled")]
    AlreadyHandled,
}

impl From<HandlerMapError> for Error {
    fn from(err: HandlerMapError) -> Self {
        Error::aborted(err)
    }
}

pub enum HandlerMap<T> {
    Leaf(T),
    Branch(HashMap<String, HandlerMap<T>>),
}

impl<T> Default for HandlerMap<T> {
    fn default() -> Self {
        Self::Branch(HashMap::new())
    }
}

impl<T> HandlerMap<T> {
    pub fn contains(&self, path: &str) -> bool {
        matches!(self, HandlerMap::Branch(map) if map.contains_key(path))
    }

    pub fn insert<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        handler: T,
    ) -> Result<(), Error> {
        match self {
            Self::Leaf(_) => Err(HandlerMapError::AlreadyHandled.into()),
            Self::Branch(children) => {
                if let Some(component) = path.next() {
                    let node = children.entry(component.to_owned()).or_default();
                    node.insert(path, handler)
                } else if children.is_empty() {
                    *self = Self::Leaf(handler);
                    Ok(())
                } else {
                    Err(HandlerMapError::AlreadyHandled.into())
                }
            }
        }
    }

    pub fn get<'a>(&self, mut path: impl Iterator<Item = &'a str>) -> Option<&T> {
        match self {
            Self::Leaf(t) => Some(t),
            Self::Branch(m) => {
                let component = path.next()?;
                m.get(component)?.get(path)
            }
        }
    }

    pub fn remove<'a>(&mut self, mut path: impl Iterator<Item = &'a str>) -> Option<T> {
        match self {
            Self::Leaf(_) => match std::mem::take(self) {
                Self::Leaf(t) => Some(t),
                _ => unreachable!(),
            },
            Self::Branch(map) => {
                let component = path.next()?;
                let next = map.get_mut(component)?;
                let result = next.remove(path);
                match &*next {
                    Self::Branch(b) if b.is_empty() => {
                        map.remove(component);
                    }
                    _ => (),
                }
                result
            }
        }
    }
}

pub struct SubscriberMap<T> {
    subscribed: Vec<T>,
    children: HashMap<String, SubscriberMap<T>>,
}

impl<T> Default for SubscriberMap<T> {
    fn default() -> Self {
        Self {
            subscribed: Vec::new(),
            children: HashMap::new(),
        }
    }
}

impl<T> SubscriberMap<T> {
    pub fn insert<'a>(&mut self, mut path: impl Iterator<Item = &'a str>, handler: T) {
        if let Some(component) = path.next() {
            self.children
                .entry(component.to_owned())
                .or_default()
                .insert(path, handler);
        } else {
            self.subscribed.push(handler);
        }
    }

    pub fn contains<'a>(&self, mut path: impl Iterator<Item = &'a str>) -> bool {
        if !self.subscribed.is_empty() {
            return true;
        }

        if let Some(next) = path.next() {
            if let Some(next_map) = self.children.get(next) {
                return next_map.contains(path);
            }
        } else {
            return !self.is_empty();
        }

        false
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty() && self.subscribed.is_empty()
    }

    pub fn retain<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a str>,
        fun: &mut impl FnMut(&T) -> bool,
    ) -> bool {
        let mut handled_by_any = false;
        self.subscribed.retain(|x| {
            handled_by_any = true;
            fun(x)
        });

        if let Some(next) = path.next() {
            if let Some(y) = self.children.get_mut(next) {
                handled_by_any = handled_by_any || y.retain(path, fun);
                if y.is_empty() {
                    self.children.remove(next);
                }
            }
        }

        handled_by_any
    }
}
