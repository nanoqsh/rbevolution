use rand::{distributions::Open01, Rng};

#[derive(Clone, Copy, Default)]
pub struct Choice {
    pub n_blu: u32,
    pub n_red: u32,
}

impl Choice {
    pub fn push(&mut self, button: Button) {
        match button {
            Button::Blue => self.n_blu += 1,
            Button::Red => self.n_red += 1,
        }
    }

    pub fn loss(&self) -> bool {
        self.n_red > self.n_blu
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Button {
    #[default]
    Blue, // default *uninitialized* value
    Red,
}

struct Actor {
    dna: f64,
    lifetime: u32,
}

impl Actor {
    const LIFETIME: u32 = 50;
    const VARIATION_COEFFICIENT: f64 = 0.05;
    const FERTILITY: f64 = 0.028;

    fn new(dna: f64) -> Self {
        Self {
            dna,
            lifetime: Self::LIFETIME,
        }
    }

    fn breed(&self) -> Option<Self> {
        let f: f64 = rand::thread_rng().gen(); // generates [0, 1)
        if f > Self::FERTILITY {
            return None;
        }

        let v: f64 = rand::thread_rng().sample(Open01); // generates (0, 1);
        let v = (v - 0.5) * 2. * Self::VARIATION_COEFFICIENT;
        let v = self.dna + v;
        Some(Self::new(v.clamp(0., 1.)))
    }

    fn select_button(&self) -> Button {
        let r: f64 = rand::thread_rng().gen(); // generates [0, 1)

        // [0, f) - red
        // [f, 1) - blue
        if r < self.dna {
            Button::Red
        } else {
            Button::Blue
        }
    }

    fn life(&mut self) -> bool {
        self.lifetime -= 1;
        self.lifetime != 0
    }
}

fn spawn_random_actors() -> impl Iterator<Item = Actor> {
    use std::iter;

    let mut rng = rand::thread_rng();
    iter::repeat_with(move || {
        let f = rng.sample(Open01); // generates (0, 1)
        Actor::new(f)
    })
}

pub struct Population {
    actors: Vec<(Actor, Button)>,
}

impl Population {
    pub fn start(n: usize) -> Self {
        Self {
            actors: spawn_random_actors()
                .take(n)
                .map(|a| (a, Button::default()))
                .collect(),
        }
    }

    pub fn select(&mut self) -> impl Iterator<Item = Button> + '_ {
        self.actors.iter_mut().map(|(a, c)| {
            let button = a.select_button();
            *c = button;
            button
        })
    }

    pub fn kill_blues(&mut self) {
        const NTH: usize = 9;

        let mut ev = EveryNth::new(NTH);
        self.actors.retain(|&(_, button)| {
            if button == Button::Blue {
                !ev.next()
            } else {
                true
            }
        })
    }

    pub fn life(&mut self) {
        self.actors.retain_mut(|(actor, _)| actor.life())
    }

    pub fn breed(&mut self) {
        const MAX_POPULATION: usize = 800;

        let mut brood = vec![];
        for (actor, _) in &self.actors {
            if brood.len() + self.actors.len() >= MAX_POPULATION {
                break;
            }

            brood.extend(actor.breed());
        }

        self.actors
            .extend(brood.into_iter().map(|a| (a, Button::default())));
    }
}

struct EveryNth {
    c: usize,
    n: usize,
}

impl EveryNth {
    fn new(n: usize) -> Self {
        Self { c: 0, n }
    }

    fn next(&mut self) -> bool {
        if self.c == self.n {
            self.c = 0;
            true
        } else {
            self.c += 1;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every0() {
        let mut ev = EveryNth::new(0);
        assert!(ev.next());
        assert!(ev.next());
        assert!(ev.next());
        assert!(ev.next());
    }

    #[test]
    fn every1() {
        let mut ev = EveryNth::new(1);
        assert!(!ev.next());
        assert!(ev.next());
        assert!(!ev.next());
        assert!(ev.next());
    }

    #[test]
    fn every2() {
        let mut ev = EveryNth::new(2);
        assert!(!ev.next());
        assert!(!ev.next());
        assert!(ev.next());
        assert!(!ev.next());
        assert!(!ev.next());
        assert!(ev.next());
    }
}
