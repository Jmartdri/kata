#[allow(dead_code)]
mod booklib {
    use std::{collections::HashMap, ops::Deref};

    #[derive(Debug, PartialEq)]
    pub struct Book {
        pub title: String,
        pub price: f32,
    }

    impl Book {
        pub fn new(title: String) -> Self {
            Book {
                title: title,
                price: 8.0,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct BookSet {
        pub book: Book,
        pub count: usize,
    }

    #[derive(Debug, PartialEq)]
    pub struct BookCart {
        pub bookset: Vec<BookSet>,
    }

    impl Deref for BookCart {
        type Target = Vec<BookSet>;
        fn deref(&self) -> &Self::Target {
            &self.bookset
        }
    }

    impl BookCart {
        pub fn new() -> Self {
            BookCart {
                bookset: Vec::new(),
            }
        }

        pub fn add(&mut self, book: Book) {
            let it = self.bookset.iter_mut();
            let mut exist = false;
            for item in it {
                if item.book == book {
                    item.count += 1;
                    exist = true;
                    break;
                }
            }
            if !exist {
                self.bookset.push(BookSet {
                    book: book,
                    count: 1,
                });
            }
        }
    }

    fn book_nb_copy(bookset: &BookCart) -> HashMap<u8, usize> {
        //get number of book'type in cart
        let count_book_type = bookset.len();
        //hold count of book peer type
        let mut bookset_counter: HashMap<u8, usize> = HashMap::with_capacity(count_book_type);
        for (k, v) in bookset.iter().enumerate() {
            bookset_counter.insert(k as u8, v.count);
        }
        return bookset_counter;
    }

    pub fn calculate_price<'a>(bookcart: &BookCart) -> f32 {
        let bookset = &*bookcart;
        let mut bookset_counter = book_nb_copy(bookset);
        let total_price = calculate_price_with_discount(&mut bookset_counter);
        return total_price;
    }

    fn decrease_counter(counter: &mut HashMap<u8, usize>) {
        let it = counter.iter_mut();
        for (_, v) in it {
            *v -= 1;
        }

        counter.retain(|_, v| *v > 0);
    }

    fn calculate_price_with_discount(bookset_counter: &mut HashMap<u8, usize>) -> f32 {
        let size = bookset_counter.len();

        match size {
            5 | 4 | 3 | 2 => {
                let discount = match size {
                    5 => 0.75,
                    4 => 0.80,
                    3 => 0.90,
                    2 => 0.95,
                    _ => panic!("Discount for buying {} different book not configured", size),
                };
                let price_of_couple: f32 = 8.0 * size as f32 * discount;
                decrease_counter(bookset_counter);
                return price_of_couple + calculate_price_with_discount(bookset_counter);
            }
            1 => {
                return 8.0 * *bookset_counter.values().next().unwrap() as f32;
            }
            0 => {
                return 0.0;
            }
            _ => panic!("Can't handle more five book's type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::booklib::*;

    #[test]
    fn it_works() {
        let b = Book::new("I".to_owned());
        assert_eq!(
            Book {
                title: "I".to_owned(),
                price: 8.0
            },
            b
        );
    }

    #[test]
    fn test_add_basket_book() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("II".to_owned()));
        bookcart.add(Book::new("I".to_owned()));

        let expected = BookCart {
            bookset: vec![
                BookSet {
                    book: Book {
                        title: "I".to_owned(),
                        price: 8.0,
                    },
                    count: 2,
                },
                BookSet {
                    book: Book {
                        title: "II".to_owned(),
                        price: 8.0,
                    },
                    count: 1,
                },
            ],
        };
        assert_eq!(expected, bookcart);
    }

    #[test]
    fn test_empty_cart() {
        let bookcart = BookCart::new();
        assert_eq!(0.0, calculate_price(&bookcart));
    }

    #[test]
    fn test_not_empty_cart() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        assert_ne!(0.0, calculate_price(&bookcart));
    }

    #[test]
    fn test_one_item_in_cart() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        assert_eq!(8.0, calculate_price(&bookcart));
    }

    #[test]
    fn test_111() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        assert_eq!(24.0, calculate_price(&bookcart));
    }

    #[test]
    fn test_1_2() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("II".to_owned()));
        assert_eq!(8.0 * 2.0 * 0.95, calculate_price(&bookcart));
    }

    #[test]
    fn test_11_2() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("II".to_owned()));
        assert_eq!((8.0 * 2.0 * 0.95) + 8.0, calculate_price(&bookcart));
    }

    #[test]
    fn test_111_2() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("II".to_owned()));
        assert_eq!(
            (8.0 * 2.0 * 0.95) + 8.0 * 2 as f32,
            calculate_price(&bookcart)
        );
    }

    #[test]
    fn test_111_22() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("II".to_owned()));
        bookcart.add(Book::new("II".to_owned()));
        assert_eq!((8.0 * 4.0 * 0.95) + 8.0, calculate_price(&bookcart));
    }

    #[test]
    fn test_1_2_3() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("II".to_owned()));
        bookcart.add(Book::new("III".to_owned()));
        assert_eq!(8.0 * 3.0 * 0.90, calculate_price(&bookcart));
    }

    #[test]
    fn test_1_2_33() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("II".to_owned()));
        bookcart.add(Book::new("III".to_owned()));
        bookcart.add(Book::new("III".to_owned()));
        bookcart.add(Book::new("III".to_owned()));
        assert_eq!((8.0 * 3.0 * 0.90) + (8.0 * 2.0), calculate_price(&bookcart));
    }

    #[test]
    fn test_1_2_3_4() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("II".to_owned()));
        bookcart.add(Book::new("III".to_owned()));
        bookcart.add(Book::new("IV".to_owned()));
        assert_eq!(8.0 * 4.0 * 0.80, calculate_price(&bookcart));
    }

    #[test]
    fn test_1_2_3_4_5() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("II".to_owned()));
        bookcart.add(Book::new("III".to_owned()));
        bookcart.add(Book::new("IV".to_owned()));
        bookcart.add(Book::new("V".to_owned()));
        assert_eq!(8.0 * 5.0 * 0.75, calculate_price(&bookcart));
    }

    #[test]
    fn test_1x6_2_3x3_4_5x3() {
        let mut bookcart = BookCart::new();
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));
        bookcart.add(Book::new("I".to_owned()));

        bookcart.add(Book::new("II".to_owned()));

        bookcart.add(Book::new("III".to_owned()));
        bookcart.add(Book::new("III".to_owned()));
        bookcart.add(Book::new("III".to_owned()));

        bookcart.add(Book::new("IV".to_owned()));

        bookcart.add(Book::new("V".to_owned()));
        bookcart.add(Book::new("V".to_owned()));
        bookcart.add(Book::new("V".to_owned()));
        assert_eq!(
            (8.0 * 5.0 * 0.75) + (8.0 * 3.0 * 0.90) * 2.0 + (8.0 * 4.0),
            calculate_price(&bookcart)
        );
    }
}
