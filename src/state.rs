use std::sync::Mutex;
use crate::models::Book;

pub struct AppState {
    books: Mutex<Vec<Book>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            books: Mutex::new(Vec::new()),
        }
    }

    pub fn create_book(&self, book: Book) {
        let mut books = self.books.lock().unwrap();
        books.push(book);
    }

    pub fn get_book(&self, book_id: u32) -> Option<Book> {
        let books = self.books.lock().unwrap();
        books.iter().cloned().find(|b| b.id == book_id)
    }

    pub fn update_book(&self, book_id: u32, book: Book) -> Option<Book> {
        let mut books = self.books.lock().unwrap();
        let pos = books.iter().position(|x| x.id == book_id)?;
        books[pos] = book.clone();
        Some(book)
    }

    pub fn delete_book(&self, book_id: u32) -> bool {
        let mut books = self.books.lock().unwrap();
        let index = books.iter().position(|x| x.id == book_id);
        index.map(|idx| books.remove(idx)).is_some()
    }
}
