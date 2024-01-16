mod db;
use db::Db;
mod library;
use std::io;
use std::thread;



#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "mongodb+srv://admin:admin@lukacluster.cf5yzeq.mongodb.net/?retryWrites=true&w=majority";
    let db = Db::new(uri).await?;

    let options = "
    0 - Exit
    1 - List all books
    2 - Add book
    3 - Delete book
    4 - Play quizz
    ";

    loop {
        let choice = read_input(options);
        match choice.as_str() {
            "0" => break,
            "1" => {
                let books = db.get_all_books().await?;
                for book in books{
                    println!("Title: {}   Author: {}  Year: {}", book.title, book.author, book.year)
                }
            },
            "2" => {
                let inserted_book = db.add_book(library::Book::create_book()).await?;
                println!("{:?}", inserted_book)
            },
            "3" => {
                let book_title = "Book Title to Delete"; 
                db.remove_book(book_title).await?;
               
            },
            "4" => {
                let response = library::get_questions().await.unwrap();
                library::start_quizz(response);
            },
            _ => println!("Invalid input!")
        }
    }
    Ok(())
}



fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}


