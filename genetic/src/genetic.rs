use rand::Rng;    // For random number generation
use std::f64::consts::E; // For base of logarithm

#[derive(Debug, Clone)]
pub struct Candidate{
    pub code: String,
    pub fitness: f64
}

pub fn fx(x1: f64, x2:f64)->f64{
    x1*x1 + x2*x2 - x1*x2
}

//initialize a gen-0 population
pub fn generate_population(x1min: f64, x1max: f64, x2min: f64, x2max: f64, pop_size: usize) -> Vec<Candidate> {
    let mut population = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..pop_size {
        let x1 = rng.gen_range(x1min..x1max);
        let x2 = rng.gen_range(x2min..x2max);
        let code = format!("{:b}", rng.gen::<u32>());  // A placeholder binary string
        let fitness = fx(x1, x2);
        population.push(Candidate { code, fitness });
    }

    population
}

// Get the number of bits required for a given precision
pub fn get_length(xmin: f64, xmax: f64, epsilon: f64) -> usize {
    (((xmax - xmin) / epsilon).log2().ceil()) as usize
}

// Decode binary string to a float value
pub fn decode(chromosome: &str) -> u64 {
    u64::from_str_radix(chromosome, 2).unwrap()
}


// Create a mating pool using roulette wheel selection
pub fn generate_mating_pool(population: &Vec<Candidate>, pop_size: usize) -> Vec<Candidate> {
    let total_fitness: f64 = population.iter().map(|c| c.fitness).sum();

    // Sort population by fitness in descending order
    let mut sorted_population = population.clone();
    sorted_population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

    // Mating pool
    let mut mating_pool = Vec::new();
    let mut slots_filled = 0;

    // Distribute candidates based on fitness proportion
    for candidate in &sorted_population {
        let proportion = (candidate.fitness / total_fitness) * pop_size as f64;
        let num_slots = proportion.floor() as usize; // Calculate the number of slots this candidate occupies

        for _ in 0..num_slots {
            if slots_filled < pop_size {
                mating_pool.push(candidate.clone());
                slots_filled += 1;
            } else {
                break;
            }
        }

        // If the pool is filled, we can stop
        if slots_filled >= pop_size {
            break;
        }
    }

    // Handle remaining slots if rounding down left the pool under-filled
    while slots_filled < pop_size {
        mating_pool.push(sorted_population[0].clone()); // Add the most fit candidate again to fill remaining slots
        slots_filled += 1;
    }

    //thus we incorporate elitism also

    mating_pool
}



// Evaluate the fitness of a candidate based on the function value
pub fn fitness(value1: f64, value2: f64) -> f64 {
    fx(value1, value2)
}

// Perform single point crossover between two parents
pub fn crossover(p1: &str, p2: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let crossover_point = rng.gen_range(1..p1.len());
    
    let child1 = format!("{}{}", &p1[..crossover_point], &p2[crossover_point..]);
    let child2 = format!("{}{}", &p2[..crossover_point], &p1[crossover_point..]);

    (child1, child2)
}

// Generate a value between a range from a decoded binary string
pub fn value(xmin: f64, xmax: f64, d: u64, length: usize) -> f64 {
    xmin + (d as f64 / ((2_u64.pow(length as u32) - 1) as f64)) * (xmax - xmin)
}

// Mutate the chromosome by flipping random bits
pub fn mutate(chromosome: &mut String, mutation_rate: f64) {
    let mut rng = rand::thread_rng();
    for i in 0..chromosome.len() {
        if rng.gen::<f64>() < mutation_rate {
            chromosome.replace_range(i..=i, if &chromosome[i..=i] == "0" { "1" } else { "0" });
        }
    }
}

pub fn genetic_algorithm(
    pop_size: usize, 
    x1min: f64, x1max: f64, 
    x2min: f64, x2max: f64, 
    epsilon: f64, 
    generations: usize,
    mutation_rate: f64
) {
    let length_x1 = get_length(x1min, x1max, epsilon);
    let length_x2 = get_length(x2min, x2max, epsilon);
    let chromosome_length = length_x1 + length_x2;
    
    let mut population: Vec<Candidate> = generate_population(x1min, x1max, x2min, x2max, pop_size);
    
    for gen in 0..generations {
        println!("Generation {}: Best fitness = {:.4}", gen, population.iter().map(|c| c.fitness).fold(f64::MIN, f64::max));

        let mating_pool = generate_mating_pool(&population, pop_size);

        let mut new_population = Vec::new();
        let mut rng = rand::thread_rng();
        for i in (0..mating_pool.len()).step_by(2) {
            let parent1 = &mating_pool[i].code;
            let parent2 = &mating_pool[(i + 1) % mating_pool.len()].code;

            let (mut child1, mut child2) = crossover(parent1, parent2);

            mutate(&mut child1, mutation_rate);
            mutate(&mut child2, mutation_rate);

            let (x1_bin, x2_bin) = child1.split_at(length_x1);
            let x1_child1 = value(x1min, x1max, decode(x1_bin), length_x1);
            let x2_child1 = value(x2min, x2max, decode(x2_bin), length_x2);
            
            let (x1_bin, x2_bin) = child2.split_at(length_x1);
            let x1_child2 = value(x1min, x1max, decode(x1_bin), length_x1);
            let x2_child2 = value(x2min, x2max, decode(x2_bin), length_x2);

            new_population.push(Candidate {
                code: child1,
                fitness: fitness(x1_child1, x2_child1),
            });
            new_population.push(Candidate {
                code: child2,
                fitness: fitness(x1_child2, x2_child2),
            });
        }

        population = new_population;
    }

    let best_candidate = population.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap()).unwrap();
    println!("Best candidate: {:?}", best_candidate);
}

fn main(){
    println!("main of genetic.rs")
}