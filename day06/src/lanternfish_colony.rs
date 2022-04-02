use std::str::FromStr;
use std::num::ParseIntError;
use std::usize;
use std::collections::hash_map::HashMap;

#[derive(Debug)]
pub struct LanternfishColony {
    population: HashMap<usize, usize>
}

impl FromStr for LanternfishColony {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut population: HashMap<usize, usize> = HashMap::with_capacity(9);
        for fish in s.split(",") {
            let fish_of_a_given_age = population.entry(fish.parse::<usize>().unwrap()).or_insert(0);
            *fish_of_a_given_age += 1;
        }
        Ok(LanternfishColony { population })
    }
}

impl LanternfishColony {
    pub fn evolve_pop_by_one_day(self) -> LanternfishColony {
        let mut new_pop: HashMap<usize, usize> = HashMap::with_capacity(9);
        for key in (0..8).rev() {
            new_pop.insert(key, *self.population.get(&(key+1)).unwrap_or(&0));
        }
        new_pop.insert(8, *self.population.get(&0).unwrap_or(&0));
        *new_pop.get_mut(&6).unwrap_or(&mut 0) += self.population.get(&0).unwrap_or(&0);
        
        return LanternfishColony { population: new_pop };
    }

    pub fn get_size_of_population(self) -> usize {
        self.population.into_values().sum()
    }
}
