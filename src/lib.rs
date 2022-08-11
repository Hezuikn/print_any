#![allow(incomplete_features)]
#![feature(specialization, box_syntax)]

use std::any::type_name;

use single_borrow::SingleBorrow;

#[track_caller]
pub fn print_any_dbg<T>(any: T) {
    print!("{} {:?} ", std::panic::Location::caller(), type_name::<T>());
    print_any(any);
}

pub fn print_any<T>(any: T) {
    print_any_nw(any);
    println!();
}

pub fn print_any_nw<T>(any: T) {
    let f = impl_print_any(any).unwrap_or_else(|_| format!("/type/{:?}", type_name::<T>()));
    print!("{f}");
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
enum not_the_actual_type {}
impl std::fmt::Display for not_the_actual_type {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}
impl Iterator for not_the_actual_type {
    type Item = not_the_actual_type;
    fn next(&mut self) -> Option<Self::Item> {
        unreachable!()
    }
}

fn impl_print_any<T>(t: T) -> Result<String, ()> {
    //display
    use specialize::constrain;
    #[allow(unused_braces)]
    if let Some(x) = constrain!(ref [{&t}: std::fmt::Display] = not_the_actual_type) {
        let x = format!("{x}");
        if x.is_empty() {
            return Ok(format!("/empty/"));
        } else {
            return Ok(x);
        }
    }

    //option / result
    trait MaybeOwnedOption {
        type Inner;
        fn maybe_owned_option(self) -> Result<Option<Self::Inner>, Box<Self>>;
    }
    impl<T> MaybeOwnedOption for T {
        default type Inner = not_the_actual_type;
        default fn maybe_owned_option(self) -> Result<Option<Self::Inner>, Box<Self>> {
            Err(box self)
        }
    }
    impl<Inner> MaybeOwnedOption for Option<Inner> {
        type Inner = Inner;
        fn maybe_owned_option(self) -> Result<Option<Self::Inner>, Box<Self>> {
            Ok(self)
        }
    }

    trait MaybeBorrowedOption {
        type Inner;
        fn maybe_borrowed_option(&self) -> Option<&Option<Self::Inner>>;
    }
    impl<T> MaybeBorrowedOption for T {
        default type Inner = not_the_actual_type;
        default fn maybe_borrowed_option(&self) -> Option<&Option<Self::Inner>> {
            None
        }
    }
    impl<Inner> MaybeBorrowedOption for Option<Inner> {
        type Inner = Inner;
        fn maybe_borrowed_option(&self) -> Option<&Option<Self::Inner>> {
            Some(self)
        }
    }

    let t = match t.maybe_owned_option() {
        Ok(x) => {
            return Ok(match x {
                Some(x) => {
                    format!("sum: {}", impl_print_any(x)?)
                }
                None => {
                    format!("noone!")
                }
            });
        }
        Err(bx) => match (*bx).single_borrow().maybe_borrowed_option() {
            Some(x) => {
                return Ok(match x {
                    Some(x) => {
                        format!("sum: {}", impl_print_any(x)?)
                    }
                    None => {
                        format!("noone!")
                    }
                });
            }
            None => *bx,
        },
    };

    //

    trait MaybeOwnedResult {
        type O;
        type E;
        fn maybe_owned_result(self) -> Result<Result<Self::O, Self::E>, Box<Self>>;
    }
    impl<T> MaybeOwnedResult for T {
        default type O = not_the_actual_type;
        default type E = not_the_actual_type;
        default fn maybe_owned_result(self) -> Result<Result<Self::O, Self::E>, Box<Self>> {
            Err(box self)
        }
    }
    impl<O, E> MaybeOwnedResult for Result<O, E> {
        type O = O;
        type E = E;
        fn maybe_owned_result(self) -> Result<Result<Self::O, Self::E>, Box<Self>> {
            Ok(self)
        }
    }

    trait MaybeBorrowedResult {
        type O;
        type E;
        fn maybe_borrowed_result(&self) -> Option<&Result<Self::O, Self::E>>;
    }
    impl<T> MaybeBorrowedResult for T {
        default type O = not_the_actual_type;
        default type E = not_the_actual_type;
        default fn maybe_borrowed_result(&self) -> Option<&Result<Self::O, Self::E>> {
            None
        }
    }
    impl<O, E> MaybeBorrowedResult for Result<O, E> {
        type O = O;
        type E = E;
        fn maybe_borrowed_result(&self) -> Option<&Result<Self::O, Self::E>> {
            Some(self)
        }
    }

    let mut t = match t.maybe_owned_result() {
        Ok(x) => {
            return Ok(match x {
                Ok(x) => {
                    format!("ok: {}", impl_print_any(x)?)
                }
                Err(x) => {
                    format!("err: {}", impl_print_any(x)?)
                }
            });
        }
        Err(bx) => match (*bx).single_borrow().maybe_borrowed_result() {
            Some(x) => {
                return Ok(match x {
                    Ok(x) => {
                        format!("ok: {}", impl_print_any(x)?)
                    }
                    Err(x) => {
                        format!("err: {}", impl_print_any(x)?)
                    }
                });
            }
            None => *bx,
        },
    };

    //iter
    #[allow(unused_braces)]
    if let Some(x) = constrain!(ref mut [{&mut t}: Iterator] = not_the_actual_type) {
        return Ok(format!("${}", impl_print_any(Vec::<_>::from_iter(x))?));
    }

    #[allow(unused_braces)]
    if let Some(x) = constrain!(ref [{t.single_borrow()}: Iterator + Clone] = not_the_actual_type) {
        return Ok(format!(
            "${}",
            impl_print_any(Vec::<_>::from_iter(x.clone()))?
        ));
    }

    //debug
    #[allow(unused_braces)]
    if let Some(x) = constrain!(ref [{&t}: std::fmt::Debug] = not_the_actual_type) {
        return Ok(format!("dbg: {x:#?}"));
    }

    return Err(());
}
