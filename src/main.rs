// Rust Fundamentals - Study Exercises
// Covers core concepts from variables through concurrency.

use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread;

// ---------------------------------------------------------------------------
// Structs and Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle({}×{})", self.width, self.height)
    }
}

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rect(Rectangle),
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rect(rect) => rect.area(),
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle(r) => format!("Circle with radius {:.2}", r),
            Shape::Rect(rect) => format!("{}", rect),
            Shape::Triangle { base, height } => {
                format!("Triangle with base {:.2} and height {:.2}", base, height)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Traits and Generics
// ---------------------------------------------------------------------------

trait Summary {
    fn summarize(&self) -> String;

    fn headline(&self) -> String {
        String::from("(Read more...)")
    }
}

#[derive(Debug)]
struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {} — {}...", self.title, self.author, &self.content[..self.content.len().min(50)])
    }

    fn headline(&self) -> String {
        format!("Breaking: {}", self.title)
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    body: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.body)
    }
}

/// Print any type that implements Summary — demonstrates trait bounds.
fn print_summary(item: &impl Summary) {
    println!("  Headline : {}", item.headline());
    println!("  Summary  : {}", item.summarize());
}

/// Return the larger of two items that implement PartialOrd + Copy.
fn largest<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

// ---------------------------------------------------------------------------
// Ownership and Borrowing
// ---------------------------------------------------------------------------

/// Takes ownership of a String and returns it back.
fn take_and_return(s: String) -> String {
    println!("  Took ownership of: {}", s);
    s
}

/// Borrows a string slice immutably.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

/// Borrows a vector mutably and appends an element.
fn push_value(vec: &mut Vec<i32>, value: i32) {
    vec.push(value);
}

// ---------------------------------------------------------------------------
// Lifetimes
// ---------------------------------------------------------------------------

/// Returns the longer of two string slices. The lifetime annotation tells
/// the compiler that the returned reference lives at least as long as both
/// inputs.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

#[derive(Debug)]
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce(&self, announcement: &str) -> &'a str {
        println!("  Announcement: {}", announcement);
        self.text
    }
}

// ---------------------------------------------------------------------------
// Error Handling
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum ParseError {
    Empty,
    NotANumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "input was empty"),
            ParseError::NotANumber(s) => write!(f, "\"{}\" is not a valid number", s),
        }
    }
}

fn parse_number(input: &str) -> Result<i64, ParseError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseError::Empty);
    }
    trimmed
        .parse::<i64>()
        .map_err(|_| ParseError::NotANumber(trimmed.to_string()))
}

fn find_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().copied().find(|n| n % 2 == 0)
}

// ---------------------------------------------------------------------------
// Pattern Matching
// ---------------------------------------------------------------------------

fn classify_number(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0 => "zero",
        1..=100 => "small positive",
        _ => "large positive",
    }
}

fn describe_option(opt: Option<i32>) -> String {
    match opt {
        Some(n) if n > 0 => format!("positive value: {}", n),
        Some(0) => "zero".to_string(),
        Some(n) => format!("negative value: {}", n),
        None => "nothing".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Concurrency
// ---------------------------------------------------------------------------

fn concurrent_sum(data: &[i32], num_threads: usize) -> i32 {
    let chunk_size = (data.len() + num_threads - 1) / num_threads;
    let shared: Arc<Vec<i32>> = Arc::new(data.to_vec());
    let mut handles = Vec::new();

    for i in 0..num_threads {
        let data_ref = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(data_ref.len());
            if start >= data_ref.len() {
                return 0;
            }
            data_ref[start..end].iter().sum::<i32>()
        }));
    }

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

fn mutex_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let result = *counter.lock().unwrap();
    result
}

// ---------------------------------------------------------------------------
// Main — runs all demonstrations
// ---------------------------------------------------------------------------

fn main() {
    println!("=== Rust Fundamentals ===\n");

    // --- Variables and types ------------------------------------------------
    println!("-- Variables and Types --");
    let name = "Rust";
    let year: u32 = 2015;
    let pi: f64 = 3.14159;
    let active = true;
    let mut count = 0;
    count += 1;
    println!("  name={}, year={}, pi={:.2}, active={}, count={}", name, year, pi, active, count);

    let tup: (i32, f64, char) = (42, 6.28, 'R');
    let (x, y, z) = tup;
    println!("  tuple: ({}, {:.2}, '{}')", x, y, z);

    let arr = [1, 2, 3, 4, 5];
    println!("  array sum: {}", arr.iter().sum::<i32>());

    // --- Ownership and borrowing -------------------------------------------
    println!("\n-- Ownership and Borrowing --");
    let s1 = String::from("hello");
    let s2 = take_and_return(s1);
    // s1 is no longer valid here — ownership was moved
    println!("  Got it back: {}", s2);

    let sentence = "hello world foo bar";
    let word = first_word(sentence);
    println!("  First word: {}", word);

    let mut numbers = vec![1, 2, 3];
    push_value(&mut numbers, 4);
    println!("  After push: {:?}", numbers);

    // --- Lifetimes ---------------------------------------------------------
    println!("\n-- Lifetimes --");
    let long = "long string";
    let result;
    {
        let short = String::from("hi");
        result = longest(long, &short);
        println!("  Longest: {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt { text: first_word(&novel) };
    println!("  Excerpt: {:?}, level={}", excerpt, excerpt.level());
    excerpt.announce("Lifetime demo");

    // --- Structs and enums -------------------------------------------------
    println!("\n-- Structs and Enums --");
    let rect = Rectangle::new(10.0, 5.0);
    println!("  {} — area={:.1}, square={}", rect, rect.area(), rect.is_square());

    let shapes: Vec<Shape> = vec![
        Shape::Circle(3.0),
        Shape::Rect(Rectangle::new(4.0, 6.0)),
        Shape::Triangle { base: 8.0, height: 3.0 },
    ];
    for shape in &shapes {
        println!("  {} — area={:.2}", shape.describe(), shape.area());
    }

    // --- Pattern matching --------------------------------------------------
    println!("\n-- Pattern Matching --");
    for n in [-5, 0, 42, 200] {
        println!("  {} → {}", n, classify_number(n));
    }
    let options: Vec<Option<i32>> = vec![Some(10), Some(0), Some(-3), None];
    for opt in options {
        println!("  {:?} → {}", opt, describe_option(opt));
    }

    // if-let shorthand
    let maybe = Some(7);
    if let Some(v) = maybe {
        println!("  if-let captured: {}", v);
    }

    // --- Error handling ----------------------------------------------------
    println!("\n-- Error Handling --");
    let inputs = ["42", "", "abc", " -7 "];
    for input in inputs {
        match parse_number(input) {
            Ok(n) => println!("  parse(\"{}\") = {}", input, n),
            Err(e) => println!("  parse(\"{}\") error: {}", input, e),
        }
    }

    let nums = [1, 3, 5, 8, 9];
    match find_even(&nums) {
        Some(n) => println!("  First even in {:?}: {}", nums, n),
        None => println!("  No even number found in {:?}", nums),
    }

    // Chaining with map / unwrap_or
    let doubled: i32 = find_even(&[1, 3, 5]).map(|n| n * 2).unwrap_or(0);
    println!("  Doubled first even (or 0): {}", doubled);

    // --- Traits and generics -----------------------------------------------
    println!("\n-- Traits and Generics --");
    let article = Article {
        title: String::from("Rust 2024 Edition"),
        author: String::from("Rust Team"),
        content: String::from("The new edition brings several improvements to the language."),
    };
    print_summary(&article);

    let tweet = Tweet {
        username: String::from("rustlang"),
        body: String::from("Rust 1.80 is out!"),
    };
    print_summary(&tweet);

    println!("  largest(3, 7) = {}", largest(3, 7));
    println!("  largest('a', 'z') = {}", largest('a', 'z'));

    // --- Concurrency -------------------------------------------------------
    println!("\n-- Concurrency --");
    let data: Vec<i32> = (1..=100).collect();
    let total = concurrent_sum(&data, 4);
    println!("  Sum of 1..=100 using 4 threads: {}", total);

    let final_count = mutex_counter();
    println!("  Mutex counter after 5 threads: {}", final_count);

    println!("\nDone.");
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle::new(4.0, 5.0);
        assert!((r.area() - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_rectangle_is_square() {
        assert!(Rectangle::new(3.0, 3.0).is_square());
        assert!(!Rectangle::new(3.0, 4.0).is_square());
    }

    #[test]
    fn test_shape_area() {
        let circle = Shape::Circle(1.0);
        assert!((circle.area() - std::f64::consts::PI).abs() < 1e-10);

        let rect = Shape::Rect(Rectangle::new(3.0, 4.0));
        assert!((rect.area() - 12.0).abs() < f64::EPSILON);

        let tri = Shape::Triangle { base: 6.0, height: 4.0 };
        assert!((tri.area() - 12.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_longest() {
        assert_eq!(longest("abc", "ab"), "abc");
        assert_eq!(longest("hi", "hello"), "hello");
    }

    #[test]
    fn test_parse_number_ok() {
        assert_eq!(parse_number("42").unwrap(), 42);
        assert_eq!(parse_number(" -7 ").unwrap(), -7);
    }

    #[test]
    fn test_parse_number_empty() {
        assert!(matches!(parse_number(""), Err(ParseError::Empty)));
        assert!(matches!(parse_number("   "), Err(ParseError::Empty)));
    }

    #[test]
    fn test_parse_number_invalid() {
        assert!(matches!(parse_number("abc"), Err(ParseError::NotANumber(_))));
    }

    #[test]
    fn test_find_even() {
        assert_eq!(find_even(&[1, 3, 5]), None);
        assert_eq!(find_even(&[1, 4, 6]), Some(4));
    }

    #[test]
    fn test_classify_number() {
        assert_eq!(classify_number(-10), "negative");
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(50), "small positive");
        assert_eq!(classify_number(200), "large positive");
    }

    #[test]
    fn test_largest() {
        assert_eq!(largest(3, 7), 7);
        assert_eq!(largest(10, 2), 10);
        assert_eq!(largest('a', 'z'), 'z');
    }

    #[test]
    fn test_concurrent_sum() {
        let data: Vec<i32> = (1..=100).collect();
        assert_eq!(concurrent_sum(&data, 4), 5050);
        assert_eq!(concurrent_sum(&data, 1), 5050);
    }

    #[test]
    fn test_mutex_counter() {
        assert_eq!(mutex_counter(), 5);
    }

    #[test]
    fn test_summary_trait() {
        let article = Article {
            title: String::from("Test"),
            author: String::from("Author"),
            content: String::from("Content here"),
        };
        assert!(article.summarize().contains("Test"));
        assert!(article.headline().contains("Breaking"));

        let tweet = Tweet {
            username: String::from("user"),
            body: String::from("hello"),
        };
        assert!(tweet.summarize().contains("@user"));
        assert_eq!(tweet.headline(), "(Read more...)");
    }
}
