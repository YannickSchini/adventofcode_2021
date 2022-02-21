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
        let mut map: HashMap<usize, usize> = HashMap::new();
        for fish in s.split(",") {
            let number = map.entry(fish.parse::<usize>().unwrap()).or_insert(0);
            *number += 1;
        }
        Ok(LanternfishColony { population: map })
    }
}

impl LanternfishColony {
    pub fn evolve_pop_by_one_day(self) -> LanternfishColony {
        let mut temp: HashMap<usize, usize> = HashMap::new();
        for key in (1..8).rev() {
            temp.insert(key-1, *self.population.get(&key).unwrap());
        }
        temp.insert(8, *self.population.get(&0).unwrap());
        *temp.get_mut(&6).unwrap() += self.population.get(&0).unwrap();
        
        return LanternfishColony { population: temp };
    }
}
