use std::collections::HashMap;

pub struct Player {
    name: String,
    weight: f32
}

impl Player {
    pub fn new(name: String, weight: f32) -> Self {
        Player {
            name,
            weight
        }
    }
}

pub struct Shapley {
    players: Vec<Player>,
    results: HashMap<String, usize>,
    total: usize,
    thresh: f32
}

impl Shapley {
    pub fn new(thresh: f32) -> Self {
        let results = HashMap::new();
        let players = Vec::new();
        let total = 0;

        Shapley {
            players,
            results,
            total,
            thresh
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn generate_power(&mut self) {
        // populates coalitions vec
        let coalitions = self.populate_permutations();
        self.total = coalitions.len();

        // populates results
        coalitions.iter().for_each(|coalition| {
            let mut total = 0.0;
            for idx in coalition {
                let player = &self.players[*idx];
                total += player.weight;
                if total >= self.thresh {
                    *self.results.entry(player.name.clone()).or_insert(0) += 1;
                    break;
                }
            }
        });
    }

    // uses Heap's algorithm
    fn populate_permutations(&mut self) -> Vec<Vec<usize>> {
        let n = self.players.len();
        let mut indices: Vec<usize> = (0..n).collect();

        let mut stack = vec![0; n];
        let mut i = 0;

        let mut coalitions = Vec::new();

        // push the OG perm
        coalitions.push(indices.clone());

        while i < n {
            if stack[i] < i {
                if i % 2 == 0 {
                    let temp = indices[0];
                    indices[0] = indices[i];
                    indices[i] = temp;
                } else {
                    let temp = indices[stack[i]];
                    indices[stack[i]] = indices[i];
                    indices[i] = temp;
                }
                coalitions.push(indices.clone());
                stack[i] += 1;
                i = 0;
            } else {
                stack[i] = 0;
                i += 1;
            }
        }

        coalitions
    }

    // TODO: fix later
    pub fn show_results(&self) {
        println!("{:?}", self.results);

        println!("Coalitions: {:?}", self.total);

        for (k, v) in &self.results {
            println!("Name: {}", k);
            println!("Total: {}", v);
            println!("Power Index: {}", *v as f32/self.total as f32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_populate_permutations() {
        let mut shapley = Shapley::new(1.0);
        assert_eq!(shapley.populate_permutations(), vec![vec![]]);
    }

    #[test]
    fn test_populate_permutations() {
        let mut shapley = Shapley::new(1.0);
        shapley.add_player(Player::new("foo".to_string(), 2.0));
        shapley.add_player(Player::new("bar".to_string(), 3.0));
        shapley.add_player(Player::new("baz".to_string(), 4.0));
        assert_eq!(shapley.populate_permutations(),
                   vec![vec![0, 1, 2],
                        vec![1, 0, 2],
                        vec![2, 0, 1],
                        vec![0, 2, 1],
                        vec![1, 2, 0],
                        vec![2, 1, 0]]);
    }

    #[test]
    fn test_populate_permutations_big() {
        let mut shapley = Shapley::new(1.0);
        (0..10).for_each(|n| shapley.add_player(Player::new(n.to_string(), 1.0)));
        assert_eq!(shapley.populate_permutations().len(), 3628800);
    }

    #[test]
    fn test_show_results() {
        let mut shapley = Shapley::new(6.0);
        shapley.add_player(Player::new("foo".to_string(), 4.0));
        shapley.add_player(Player::new("bar".to_string(), 3.0));
        shapley.add_player(Player::new("baz".to_string(), 2.0));
        shapley.generate_power();
        shapley.show_results();
    }
}

