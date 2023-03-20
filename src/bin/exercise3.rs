// create a struct Novel with title, author, genre (all String)

// create a struct NonFiction with title, author, topic (all String)

// create a trait Book

// implement Book trait for Novel
   // define get_summary which takes in &self
       // prints "<Book Title> is a <Book Genre> written by <Book Author>"

// implement Book trait for NonFiction
   // define get_summary which takes in &self
       // prints "<Book Title> is a <Book Topic> written by <Book Author>"

// define fuction wihch takes in a generic that that implements Book trait
   // call get_summary with the book

   fn main() {
    // create book_1 of instance Novel
   
    // create book_2 of instance NonFiction
 
    // call print_book_summary with book_1
    // call print_book_summary with book_2
 }
 