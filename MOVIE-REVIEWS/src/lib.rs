use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use std::collections::HashMap;

// use near_sdk::collections::{Vector};
// std::string::ToString::to_string{};

//Movie struct
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
//holds for the movie and details about it.
pub struct Movie {
    movie_name: String,
    author:String,
    stream_platform: String,
    rated:String,//this is either GE or PG
    
}
//reviews struct
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
//holds for the movie and the reviews about it
pub struct Review {
    movie_name: String,
    description:String,//either action , arcade , comedy etc.
    reviewer:String,
    best_quote:String,
    
}
 
 // storing the movies and reviews 
#[near_bindgen]
#[derive( BorshDeserialize, BorshSerialize)]
pub struct App {
    moviestore: HashMap<String, Movie>,
    reviewstore: HashMap<String, Review>,
}

//initiating an initial state
impl Default for App {
   fn default() -> Self {
        Self {
            moviestore: HashMap::new(),
            reviewstore: HashMap::new(),
        }
    }
}
#[near_bindgen]
impl App {
    // ADD CONTRACT METHODS HERE
    //adding a movie 
    pub fn add_movie(&mut self ,
        movie_name: String,
        author:String,
        stream_platform: String,
        rated:String,
    ){
        let new_movie: Movie = Movie {
            movie_name: movie_name.to_string(),
            author:author.to_string(),
            stream_platform: stream_platform.to_string(),
            rated: rated.to_string(),
        };
        self.moviestore.insert(movie_name, new_movie);
    }
    //counting the number of my movies
    pub fn movie_count(&mut self )-> usize{
        self.moviestore.len()
    }
    
    //viewing the movies
    pub fn get_movie(&self, movie_name: String) -> Vec<&String> {
        let movie = &self.moviestore[&movie_name];
        let mut vect = Vec::new();
        vect.push(&movie.author);
        vect.push(&movie.stream_platform);
        vect.push(&movie.rated);
        vect
    }
    // removing a movie 
    pub fn remove_movie(& mut self , movie_name: String){
        self.moviestore.remove(&movie_name);
    }
    
    
    // adding reviews 
    pub fn add_review(&mut self,
        movie_name: String,
        description:String,
        reviewer:String,
        best_quote:String,
    ){
        let new_review:Review = Review {
            movie_name:movie_name.to_string(),
            description:description.to_string(),
            reviewer:reviewer.to_string(),
            best_quote:best_quote.to_string(),
        };
        self.reviewstore.insert(movie_name,new_review);
    }
    //counting the number of my  movie reviews
    pub fn review_count(&mut self)-> usize{
        self.reviewstore.len()
    }
    //viewing the movie  reviews 
    pub fn get_review(&self, movie_name: String)->Vec<&String>{
        let review = &self.reviewstore[&movie_name];
        let mut r_vect = Vec::new();
        r_vect.push(&review.movie_name);
        r_vect.push(&review.description);
        r_vect.push(&review.reviewer);
        r_vect.push(&review.best_quote);
        r_vect
    }
    //removing a movie review
    pub fn remove_review(& mut self , movie_name: String){
        self.reviewstore.remove(&movie_name);
    }

   



}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    // use near_sdk::test_utils::{VMContextBuilder};
    // use near_sdk::{ AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    // fn get_context(predecessor: AccountId) -> VMContextBuilder {
        //     let mut builder = VMContextBuilder::new();
        //     builder.predecessor_account_id(predecessor);
        //     builder
        // }
        
    #[test]
    // TESTS HERE
     fn create_movie(){
        
        let mut contract = App::default();
        // let mut moviestore = HashMap::new();
        contract.add_movie(
            "Nightmare".to_string(),
            "Martin Luther King-jr".to_string(),
            "Netflix".to_string(),  
            "GE".to_string(),
        );
        contract.add_movie(
            "Harry Potter".to_string(),
            "J K Rowling".to_string(),
            "HBO Max".to_string(),  
            "GE".to_string(),
        );
        contract.add_movie(
            "Who Killed Sara".to_string(),
            "José Ignacio Valenzuela".to_string(),
            "Netflix".to_string(),  
            "PG".to_string(),
        );
        contract.add_movie(
            "Pablo Escobar".to_string(),
            "Sebastián Marroquín".to_string(),
            "Prime Video".to_string(),  
            "PG".to_string(),
        );
        
        let counts = contract.movie_count();
        
        assert_eq!(4,counts);
       

    }
    #[test]
    fn create_review(){
        let mut contrac = App::default();
        contrac.add_review(
            String::from("Nightmare"),
            String::from("Thriller based Movie"),
            String::from("Jerry Joseph"),
            String::from("Whatever you do do not fall asleep"),
        );
        contrac.add_review(
            String::from("Who Killed Sara"),
            String::from("Crime Thriller Mystery based Film"),
            String::from("Alex Guzman"),
            String::from("You do not hurt the ones you love."),
        );
        contrac.add_review(
            String::from("Harry Potter"),
            String::from("Adventure/Fiction based Movie "),
            String::from("Telesco L"),
            String::from("The thing we loose have a way of comming back to us"),
        );
        contrac.add_review(
            String::from("Pablo Escobar"),
            String::from("Crime,Drama Film"),
            String::from("Narcos Empire"),
            String::from("Whatever you do do not fall asleep"),
        );
        let count = contrac.review_count();
        assert_eq!(4,count);
    }
    #[test]
    fn delete_movie(){
        let mut contract = App::default();
        // let mut moviestore = HashMap::new();
        contract.add_movie(
            "Nightmare".to_string(),
            "Martin Luther King-jr".to_string(),
            "Netflix".to_string(),  
            "GE".to_string(),
        );
        contract.add_movie(
            "Harry Potter".to_string(),
            "J K Rowling".to_string(),
            "HBO Max".to_string(),  
            "GE".to_string(),
        );
        
        contract.remove_movie("Nightmare".to_string());
        let counts = contract.movie_count();
        assert_eq!(1,counts);
    }
    #[test]
    fn delete_review(){
        let mut contrac = App::default();
        contrac.add_review(
            String::from("Nightmare"),
            String::from("Thriller based Movie"),
            String::from("Jerry Joseph"),
            String::from("Whatever you do do not fall asleep"),
        );
        contrac.add_review(
            String::from("Who Killed Sara"),
            String::from("Crime Thriller Mystery based Film"),
            String::from("Alex Guzman"),
            String::from("You do not hurt the ones you love."),
        );
        contrac.remove_review("Nightmare".to_string());
        let count = contrac.review_count();
        assert_eq!(1,count)
    }
    #[test]
    fn acquire_movie(){
        let mut contract = App::default();
        // let mut moviestore = HashMap::new();
        contract.add_movie(
            "Nightmare".to_string(),
            "Martin Luther King-jr".to_string(),
            "Netflix".to_string(),  
            "GE".to_string(),
        );
        let acquired = contract.get_movie("Nightmare".to_string());
        let veclength = acquired.len();
        assert_eq!(3,veclength)
    }
    #[test]
    fn acquire_review(){
        let mut contrac = App::default();
        contrac.add_review(
            String::from("Nightmare"),
            String::from("Thriller based Movie"),
            String::from("Jerry Joseph"),
            String::from("Whatever you do do not fall asleep"),
        );
        let acquired = contrac.get_review("Nightmare".to_string());
        let veclength = acquired.len();
        assert_eq!(4,veclength)
    }
}
