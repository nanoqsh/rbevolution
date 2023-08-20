mod chart;
mod model;

fn main() {
    use crate::model::{Actor, Choice};

    let actors: Vec<Actor> = model::spawn_random_actors().take(300).collect();
    let mut choices = vec![];
    for _ in 0..400 {
        let mut c = Choice::default();
        for actor in &actors {
            let button = actor.select_button();
            c.push(button);
        }

        //

        choices.push(c);
    }

    crate::chart::render(choices);
}
