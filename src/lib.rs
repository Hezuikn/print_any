#![allow(incomplete_features)]
#![feature(specialization)]

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

fn impl_print_any<T>(mut t: T) -> Result<String, ()> {
    let b = t.single_borrow();

    //display
    use specialize::constrain;
    if let Some(x) = constrain!(ref [b: std::fmt::Display] = not_the_actual_type) {
        let x = format!("{x}");
        if x.is_empty() {
            return Ok(format!("/empty/"));
        } else {
            return Ok(x);
        }
    }

    //option / result
    {
        trait MaybeOption<'a> {
            type Inner;
            fn maybe_option(self) -> Option<&'a Option<Self::Inner>>;
        }
        impl<'a, T> MaybeOption<'a> for T {
            default type Inner = not_the_actual_type;
            default fn maybe_option(self) -> Option<&'a Option<Self::Inner>> {
                None
            }
        }
        impl<'a, T> MaybeOption<'a> for &'a Option<T> {
            type Inner = T;
            fn maybe_option(self) -> Option<&'a Option<Self::Inner>> {
                Some(self)
            }
        }
        if let Some(x) = b.maybe_option() {
            return Ok(match x {
                None => format!("noone!"),
                Some(x) => {
                    format!("sum: {}", impl_print_any(x)?)
                }
            });
        }
        //
        trait MaybeResult<'a> {
            type Ok;
            type Err;
            fn maybe_result(self) -> Option<&'a Result<Self::Ok, Self::Err>>;
        }
        impl<'a, T> MaybeResult<'a> for T {
            default type Ok = not_the_actual_type;
            default type Err = not_the_actual_type;
            default fn maybe_result(self) -> Option<&'a Result<Self::Ok, Self::Err>> {
                None
            }
        }
        impl<'a, O, E> MaybeResult<'a> for &'a Result<O, E> {
            type Ok = O;
            type Err = E;
            fn maybe_result(self) -> Option<&'a Result<O, E>> {
                Some(self)
            }
        }
        if let Some(x) = b.maybe_result() {
            return Ok(match x {
                Ok(x) => {
                    format!("ok: {}", impl_print_any(x)?)
                }
                Err(x) => {
                    format!("err: {}", impl_print_any(x)?)
                }
            });
        }
    }

    //iter
    let m = &mut t;
    if let Some(x) = constrain!(ref mut [m: Iterator] = not_the_actual_type) {
        return Ok(format!("${}", impl_print_any(Vec::<_>::from_iter(x))?))
    }

    let b = t.single_borrow();

    //debug
    if let Some(x) = constrain!(ref [b: std::fmt::Debug] = not_the_actual_type) {
        return Ok(format!("dbg: {x:#?}"));
    }

    return Err(());
}
