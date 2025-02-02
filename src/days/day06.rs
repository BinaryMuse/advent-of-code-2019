use std::collections::{BTreeSet, HashMap};

pub(crate) fn run(input: String) {
    let mut orbits = Orbits::new();
    for line in input.trim().lines() {
        let mut parts = line.split(")");
        let orbitee = parts.next().unwrap();
        let orbiter = parts.next().unwrap();
        orbits.add_orbit(orbiter.into(), orbitee.into());
    }

    let count = orbits.count_orbits();
    println!("Part 1: {}", count);

    let count = orbits.count_transfers_to_san();
    println!("Part 2: {}", count);
}

struct Orbits {
    data: HashMap<String, String>,
}

impl Orbits {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add_orbit(&mut self, orbiter: String, orbitee: String) {
        self.data.insert(orbiter, orbitee);
    }

    pub fn count_orbits(&self) -> usize {
        let mut cache: HashMap<String, usize> = HashMap::new();

        let mut total = 0;
        for (orbiter, _) in self.data.iter() {
            let orbit_count = self.count_orbits_for(orbiter, &mut cache);
            total += orbit_count
        }

        total
    }

    fn count_orbits_for(&self, orbiter: &str, cache: &mut HashMap<String, usize>) -> usize {
        if cache.contains_key(orbiter) {
            *cache.get(orbiter).unwrap()
        } else {
            if let Some(parent) = self.data.get(orbiter) {
                let count = 1 + self.count_orbits_for(parent, cache);
                cache.insert(orbiter.into(), count);
                count
            } else {
                0
            }
        }
    }

    fn count_transfers_to_san(&self) -> usize {
        let mut my_visits: BTreeSet<String> = BTreeSet::new();
        let mut my_path: Vec<String> = vec![];

        let mut san_visits: BTreeSet<String> = BTreeSet::new();
        let mut san_path: Vec<String> = vec![];

        let mut my_pos = self.data.get("SAN").unwrap().clone();
        let mut san_pos = self.data.get("YOU").unwrap().clone();
        san_visits.insert(san_pos.clone());
        my_visits.insert(my_pos.clone());

        loop {
            let parent = self.data.get(&san_pos).unwrap().clone();
            san_visits.insert(parent.clone());
            san_path.push(parent.clone());
            if parent == "COM" {
                break;
            }
            san_pos = parent;
        }

        loop {
            let parent = self.data.get(&my_pos).unwrap().clone();
            my_visits.insert(parent.clone());
            my_path.push(parent.clone());

            if san_visits.contains(&parent) {
                break;
            }
            my_pos = parent;
        }

        let my_last_visit = my_path.last().unwrap();
        let san_index = san_path
            .iter()
            .position(|visit| visit == my_last_visit)
            .unwrap();
        my_path.len() + san_index + 1
    }
}
