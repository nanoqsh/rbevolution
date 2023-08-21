mod chart;
mod model;

use crate::model::{Choice, Population};

fn main() {
    let mut population = Population::start(200);
    let mut choices = vec![];
    for _ in 0..1500 {
        let mut choise = Choice::default();
        for button in population.select() {
            choise.push(button);
        }

        if choise.loss() {
            population.kill_blues();
        }

        population.life();
        population.breed();
        choices.push(choise);
    }

    crate::chart::render(choices);
}
