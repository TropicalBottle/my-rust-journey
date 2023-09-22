pub struct Book {
    author: String,
    title: String,
    year: u32,
    isbn: String,
}

impl Book {
    pub fn new(author: String, title: String, year: u32, isbn: String) -> Self {
        Book {
            author,
            title,
            year,
            isbn,
        }
    }
    
    ///Get the different informations about the book to print
    pub fn get_book(&self) {
        println!(
            "{}, a {} book by {}, {}",
            self.title, self.year, self.author, self.isbn
        );
    }

    pub fn get_author(&self) -> String {
        self.author.clone()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_year(&self) -> u32 {
        self.year
    }

    pub fn get_isbn(&self) -> String {
        self.isbn.clone()
    }

    pub fn edit_author(&mut self, author: String) {
        self.author = author;
    }

    pub fn edit_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn edit_year(&mut self, year: u32) {
        self.year = year;
    }

    pub fn edit_isbn(&mut self, isbn: String) {
        self.isbn = isbn;
    }
}
