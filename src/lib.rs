// std

// crates

// local

//! Min is a minfication library for HTML, CSS, and JS. Min can only minify CSS currently.
//!
//! [GitHub](https://github.com/EthanJustice/min)
//! | [Lib.rs](https://lib.rs/crates/min)
//! | [Docs.rs](https://docs.rs/min/latest)

pub mod css;
pub mod html;
pub mod js;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
