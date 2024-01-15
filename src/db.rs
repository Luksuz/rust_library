// Assuming `Book` is correctly defined in `crate::library`
use crate::library::Book;


use mongodb::{
    results::InsertOneResult,
    bson::{doc, from_document},
    error::Result,
    Client, Collection,
};
use futures::stream::StreamExt;

pub struct Db {
    client: Client,
}

impl Db {
    // Specify the MongoDB error type for the Result
    pub async fn new(uri: &str) -> mongodb::error::Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        Ok(Db { client })
    }

    pub fn books_collection(&self) -> Collection<Book> {
        self.client.database("rustyLibrary").collection("books")
    }

    pub async fn get_all_books(&self) -> mongodb::error::Result<Vec<Book>>{
        let collection = self.books_collection();
        let mut cursor = collection.find(None, None).await?;
    
        let mut books = Vec::new();
        while let Some(result) = cursor.next().await {
            let doc = result?; // Handle the Result
            books.push(doc);
        }
    
        Ok(books)
    }
    
    // Specify the MongoDB error type for the Result
    pub async fn add_book(&self, new_book: Book) -> mongodb::error::Result<mongodb::results::InsertOneResult> {
        let collection = self.books_collection();
        let book = collection.insert_one(new_book, None).await?;
        Ok(book)
    }
    
    // Specify the MongoDB error type for the Result
    pub async fn remove_book(&self, book_title: &str) -> mongodb::error::Result<()> {
        let collection = self.books_collection();
        collection.delete_one(doc! {"title": book_title}, None).await?;
        Ok(())
    }
}
