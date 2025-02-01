use std::collections::BTreeMap;

use crate::{common::Method, request::Request, response::Response};

pub type Params = BTreeMap<String, String>;
pub type Handler<T> = fn(req: Request, params: &Params, state: &T) -> Response;

#[derive(Clone, Debug)]
pub struct Router<T> {
    routes: Vec<Route<T>>,
    state: T,
}

impl<T> Router<T> {
    pub fn new(state: T) -> Self {
        Self {
            routes: Vec::new(),
            state,
        }
    }

    pub fn add(&mut self, method: Method, path_pattern: &str, handler: Handler<T>) {
        self.routes.push(Route {
            path_pattern: Pattern::new(path_pattern).expect("Invalid path pattern"),
            method,
            handler,
        });
    }

    pub fn handle(&self, req: Request) -> Option<Response> {
        let route = self.routes.iter().find_map(|r| {
            if r.method != req.method {
                return None;
            }

            return r.path_pattern.matches(&req.path).map(|params| RouteMatch {
                handler: r.handler,
                params,
            });
        });

        route.map(|route| (route.handler)(req, &route.params, &self.state))
    }
}

#[derive(Clone, Debug)]
struct Route<T> {
    method: Method,
    path_pattern: Pattern,
    handler: Handler<T>,
}

#[derive(Clone, Debug)]
struct Pattern {
    sections: Vec<PatternSection>,
}

#[derive(Clone, Debug, PartialEq)]
enum PatternSection {
    Concrete(String),
    Param(String),
    Wildcard(String),
}

// Patterns can match /path/:param/*wildcard
// `param` and `wildcard` will be caputred
// Params cannot include slashes, wildcards can
// Wildcard must be at the end
impl Pattern {
    fn new(pattern: &str) -> Result<Self, ()> {
        let parts = pattern.split("/");
        let sections = parts
            .into_iter()
            .map(|part| {
                if part.starts_with(':') {
                    return PatternSection::Param(part[1..].to_string());
                }

                if part.starts_with('*') {
                    return PatternSection::Wildcard(part[1..].to_string());
                }

                return PatternSection::Concrete(part.to_string());
            })
            .collect::<Vec<_>>();

        // Wildcards only work at the end
        if sections[..sections.len() - 1]
            .iter()
            .any(|p| matches!(p, PatternSection::Wildcard(_)))
        {
            return Err(());
        }

        return Ok(Self { sections });
    }

    fn matches(&self, path: &str) -> Option<Params> {
        let mut part_iter = path.split("/").collect::<Vec<_>>().into_iter();
        let mut params = Params::new();

        let mut next_part = part_iter.next();
        for (i, section) in self.sections.iter().enumerate() {
            let Some(part) = next_part else {
                // Get here if there is no part of the path left
                // We still have some of the pattern left
                // The only way this is valid is there is only a single wildcard left (that is
                // empty)
                if let PatternSection::Wildcard(name) = section {
                    if i == self.sections.len() - 1 {
                        params.insert(name.to_string(), String::new());
                        return Some(params);
                    }
                }

                return None;
            };

            match section {
                PatternSection::Param(name) => {
                    params.insert(name.to_string(), part.to_string());
                    next_part = part_iter.next();
                }
                PatternSection::Wildcard(name) => {
                    let mut capture = part.to_string();

                    next_part = part_iter.next();
                    while let Some(part) = next_part {
                        capture.push('/');
                        capture.push_str(part);
                        next_part = part_iter.next();
                    }

                    params.insert(name.to_string(), capture);
                }
                PatternSection::Concrete(value) => {
                    if value != part {
                        return None;
                    }

                    next_part = part_iter.next();
                }
            }
        }

        // If no parts left, the match was successful
        match next_part {
            Some(_) => return None,
            None => return Some(params),
        };
    }
}

#[derive(Clone, Debug)]
pub struct RouteMatch<T> {
    pub handler: Handler<T>,
    pub params: Params,
}
