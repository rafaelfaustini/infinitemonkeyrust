use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;



struct TextElement {
    target: String,
    value: String,
}

impl TextElement {
    fn new(target: String, defined_value: String) -> Self {
        let mut value = defined_value;
        if value.is_empty() {
           value = TextElement::generate_random_value(&target);
        }
        
        TextElement { target, value }
    }
    
    fn generate_random_value(target: &String) -> String {
        let target_length = target.len();
        
        let value: String = Alphanumeric.sample_string(&mut rand::thread_rng(), target_length);
            
        value as String
    }

    fn get_score(&self) -> f32 {
        let mut total = 0.0;
    
        for (i, c) in self.target.char_indices() {
          if self.value.chars().nth(i).unwrap() == c {
              total = total + 1.0;
          }
        }
        
        let score = total / self.target.len() as f32;
        
        score as f32
    }
    
    fn mutate(&mut self, mutation_rate: f32) {
        let mut rng = rand::thread_rng();
        let mut mutated_value = String::with_capacity(self.value.len());
        
        for c in self.value.chars() {
            let random_flag = rng.gen::<f32>();
            if random_flag <= mutation_rate {
                let value: char = Alphanumeric.sample_string(&mut rand::thread_rng(), 1).chars().next().unwrap();
                mutated_value.push(value);
            } else {
                mutated_value.push(c);
            }
        }
        
        self.value = mutated_value;

    }
}

struct Guesser {
    target: String,
    population_size: i32,
    mutation_rate: f32,
    population :  Vec<TextElement>,
    generation: i32,
    best_score: f32
}

impl Guesser {
    fn new(target: String, population_size: i32, mutation_rate: f32) -> Self {
        
        Guesser { 
            target: target, 
            population_size: population_size, 
            mutation_rate: mutation_rate,  
            population: Vec::new(),
            generation: 0,
            best_score: 0.0
        }
    }
    
    
    fn initialize_population(&mut self) {
        for i in 1..=self.population_size {
            let mut text_element = TextElement::new( self.target.clone(), String::from("") );
            self.population.push(text_element);
        }
    }
    
    fn get_best_from_population(&mut self) -> &mut TextElement {
        let mut best_score_index = 0 as usize;
        for index in 0..=self.population_size-1 {
            let current_text = &self.population[index as usize];
            let best_score = &self.population[best_score_index];
            if current_text.get_score() > best_score.get_score() {
                best_score_index = index as usize;
            }
        }

        
        &mut self.population[best_score_index]
    }
    
    fn select(&mut self) -> usize {
        let mut best_score_index = 0;
        for index in 0..self.population.len() {
            if self.population[index].get_score() > self.population[best_score_index].get_score() {
                best_score_index = index;
            }
        }
        self.best_score = self.population[best_score_index].get_score();
        println!("{}. {}:{}", self.generation, self.population[best_score_index].value, self.population[best_score_index].get_score());
        best_score_index
    }
    
    fn breed(&mut self) {
        let mutation_rate = self.mutation_rate;
        let mut new_population: Vec<TextElement> = Vec::new();
    
        let pivot_index = self.select();
    
        for (index, element) in self.population.iter().enumerate() {
            let mut child = self.crossover(&self.population[pivot_index], element);
            child.mutate(mutation_rate);
            new_population.push(child);
        }
    
        self.population = new_population;
    }
    
    fn crossover(&self, pivot: &TextElement, partner: &TextElement) -> TextElement {
        let mut rng = rand::thread_rng();
        let mut child_value : String = String::with_capacity(pivot.value.len());

        for (pivot_char, partner_char) in pivot.value.chars().zip(partner.value.chars()) {
            let random_flag: bool = rng.gen();
            if random_flag == true {
                child_value.push(pivot_char);
            } else {
                child_value.push(partner_char);
            }
        }
        
        let child : TextElement = TextElement::new( self.target.clone(), child_value );
        
        child as TextElement
    }
    
    fn start(&mut self) {
        self.initialize_population();
        while self.best_score < 1.0 {
            self.breed();
            self.generation = self.generation + 1;
        }

    }
    

}





fn main() {
    let mut guesser = Guesser::new(String::from("InfiniteMonkey"), 100, 0.01);
    guesser.start();
    
    
    
    match true {
        true => println!("true"),
        false => println!("false"),
    }
}