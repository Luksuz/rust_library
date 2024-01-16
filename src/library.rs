use std::io;
use reqwest;
use tokio;
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(serde::Deserialize)]
pub struct ApiResponse {
    results: Vec<Question>,
}

#[derive(serde::Deserialize)]
pub struct Question {
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Book{
    pub title: String,
    pub author: String,
    pub year: i32
}

impl Book{
    pub fn new(title: String, author: String, year: i32) -> Book{
        Book{
            title,
            author,
            year
        }
    }

    pub fn create_book() -> Book {
        let title = read_input("Enter the title of the book: ");
        let author = read_input("Enter the author's name: ");
        let year_str = read_input("Enter the year of the book: ");

        let year = year_str.parse::<i32>().unwrap_or(0); // Defaulting to 0 if parsing fails

        Book::new(title, author, year)
    }
}


pub async fn get_questions() -> Result<Vec<Question>, reqwest::Error> {
    let url = "https://opentdb.com/api.php?amount=10";

    let response = reqwest::get(url).await?;

    let mut quiz_objects : Vec<Question> = Vec::new();
    if response.status().is_success() {
        let body = response.text().await?;
        let api_response: ApiResponse = serde_json::from_str(&body)
            .expect("Failed to parse JSON"); // Replace with better error handling

        for question in api_response.results {
            let mut quiz_object = Question { question: question.question, correct_answer: question.correct_answer, incorrect_answers: question.incorrect_answers };
            quiz_objects.push(quiz_object)
        }
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }
    Ok(quiz_objects)
}


pub fn start_quizz(questions : Vec<Question>){
    let mut score = 0;
    println!("Welcome to the rusty quizz!");
    for question in questions{
        let mut all_questions: Vec<String> = Vec::new();
        for answer in question.incorrect_answers{
            all_questions.push(answer);
        }
        all_questions.push(question.correct_answer.clone());


        match all_questions.len() {
            2 => {
                println!("{}", question.question);
                println!("1.{:?}  2.{:?}", all_questions[0], all_questions[1]);
            },
        
            3 => {
                println!("{}", question.question);
                println!("1.{:?}  2.{:?}  3.{:?}", all_questions[0], all_questions[1], all_questions[2]);
            },
        
            4 => {
                println!("{}", question.question);
                println!("1.{:?}  2.{:?}  3.{:?}  4.{:?}", all_questions[0], all_questions[1], all_questions[2], all_questions[3]);
            },
        
            _ => {println!("{}", all_questions.len())} // You might want to handle other cases or do nothing
        }
        let mut answer = read_input("Your answer: ");
        if answer == question.correct_answer{
            println!("Correct aswer");
            score += 1
        } else {
            println!("Incorrect aswer");
        }
    }
    println!("{}/10 correct answers", score)
}









pub struct Library {
    name: String,
    books: Vec<Book>,
}

impl Library {
    pub fn new(name: String) -> Library {
        Library {
            name,
            books: Vec::new(),
        }
    }

    pub fn create_library(&mut self) {
        let name = read_input("Hello new library owner! \n Enter the name of your brand new library: ");
        self.name = name;  // Update the name of the current instance
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn remove_book(&mut self) {
        let name = read_input("Enter the name of the book: ");
        let mut removed = false;
        self.books.retain(|book| {
            if book.title == name {
                println!("Successfully removed {}!", name);
                removed = true;
                false
            } else {
                true
            }
        });

        if !removed {
            println!("No book found with that name.");
        }
    }

    pub fn edit_book(&mut self){
        let name = read_input("Input the book name you want to change: ");


        let mut book_index = None;

        for (index, book) in self.books.iter().enumerate() {
            if book.title == name {
                book_index = Some(index);
                break;
            }
        }

        if let Some(index) = book_index {
            let new_name = read_input("Enter the new name of the book: ");
            let new_author = read_input("Enter the new author's name: ");
            let new_year_str = read_input("Enter the new year of the book: ");
            let new_year = new_year_str.parse::<i32>().unwrap_or(0); // Defaulting to 0 if parsing fails

            self.books[index] = Book::new(new_name, new_author, new_year);
        } else {
            println!("No book found with that name.");
        }
    }

    pub fn list_books(&self){
        if self.books.len() > 0{
            for book in &self.books{
                println!("name: {}/nAuthor: {}/nYear: {}", book.title.to_string(), book.author.to_string(), book.year.to_string())
            }
        } else {
            println!("No books in the library!")
        }
        
    }
}


fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

