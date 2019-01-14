use core::fmt::Display;

use crate::{Compat, Context, Fail};

/// Extension methods for `Result`.
pub trait ResultExt<T, E> {
    /// Wraps the error in `Compat` to make it compatible with older error
    /// handling APIs that expect `std::error::Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() {
    /// #    tests::run_test();
    /// # }
    /// #
    /// # #[cfg(not(all(feature = "std", feature = "derive")))] mod tests { pub fn run_test() { } }
    /// #  
    /// # #[cfg(all(feature = "std", feature = "derive"))] mod tests {
    /// use std::error::Error;
    /// # use std::fmt;
    /// #
    /// # extern crate failure;
    /// #
    /// # use crate::tests::failure::ResultExt;
    /// #
    /// # #[derive(Debug)]
    /// struct CustomError;
    ///
    /// impl Error for CustomError {
    ///     fn description(&self) -> &str {
    ///         "My custom error message"
    ///     }
    ///
    ///     fn cause(&self) -> Option<&Error> {
    ///         None
    ///     }
    /// }
    /// #
    /// # impl fmt::Display for CustomError {
    /// #     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    /// #         write!(f, "{}", self.description())
    /// #     }
    /// # }
    /// #
    /// # pub fn run_test() {
    ///
    /// let x = (|| -> Result<(), failure::Error> {
    ///     Err(CustomError).compat()?
    /// })().with_context(|e| {
    ///     format!("An error occured: {}", e)
    /// }).unwrap_err();
    ///
    /// let x = format!("{}", x);
    ///
    /// assert_eq!(x, "An error occured: My custom error message");
    /// # }
    ///
    /// # }
    /// ```
    fn compat(self) -> Result<T, Compat<E>>;

    /// Wraps the error type in a context type.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(all(feature = "std", feature = "derive"))]
    /// # #[macro_use] extern crate failure;
    /// #
    /// # #[cfg(all(feature = "std", feature = "derive"))]
    /// # #[macro_use] extern crate failure_derive;
    /// #
    /// # fn main() {
    /// #    tests::run_test();
    /// # }
    /// #
    /// # #[cfg(not(all(feature = "std", feature = "derive")))] mod tests { pub fn run_test() { } }
    /// #
    /// # #[cfg(all(feature = "std", feature = "derive"))] mod tests {
    /// #
    /// # use failure::{self, ResultExt};
    /// #
    /// #[derive(Fail, Debug)]
    /// #[fail(display = "")]
    /// struct CustomError;
    /// #
    /// # pub fn run_test() {
    ///  
    /// let x = (|| -> Result<(), failure::Error> {
    ///     Err(CustomError)?
    /// })().context(format!("An error occured")).unwrap_err();
    ///
    /// let x = format!("{}", x);
    ///
    /// assert_eq!(x, "An error occured");
    /// # }
    ///
    /// # }
    /// ```
    fn context<D>(self, context: D) -> Result<T, Context<D>>
    where
        D: Display + Send + Sync + 'static;

    /// Wraps the error type in a context type generated by looking at the
    /// error value.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(all(feature = "std", feature = "derive"))]
    /// # #[macro_use] extern crate failure;
    /// #
    /// # #[cfg(all(feature = "std", feature = "derive"))]
    /// # #[macro_use] extern crate failure_derive;
    /// #
    /// # fn main() {
    /// #    tests::run_test();
    /// # }
    /// #
    /// # #[cfg(not(all(feature = "std", feature = "derive")))] mod tests { pub fn run_test() { } }
    /// #
    /// # #[cfg(all(feature = "std", feature = "derive"))] mod tests {
    /// #
    /// # use failure::{self, ResultExt};
    /// #
    /// #[derive(Fail, Debug)]
    /// #[fail(display = "My custom error message")]
    /// struct CustomError;
    /// #
    /// # pub fn run_test() {
    ///
    /// let x = (|| -> Result<(), failure::Error> {
    ///     Err(CustomError)?
    /// })().with_context(|e| {
    ///     format!("An error occured: {}", e)
    /// }).unwrap_err();
    ///
    /// let x = format!("{}", x);
    ///
    /// assert_eq!(x, "An error occured: My custom error message");
    /// # }
    ///
    /// # }
    /// ```
    fn with_context<F, D>(self, f: F) -> Result<T, Context<D>>
    where
        F: FnOnce(&E) -> D,
        D: Display + Send + Sync + 'static;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: Fail,
{
    fn compat(self) -> Result<T, Compat<E>> {
        self.map_err(|err| err.compat())
    }

    fn context<D>(self, context: D) -> Result<T, Context<D>>
    where
        D: Display + Send + Sync + 'static,
    {
        self.map_err(|failure| failure.context(context))
    }

    fn with_context<F, D>(self, f: F) -> Result<T, Context<D>>
    where
        F: FnOnce(&E) -> D,
        D: Display + Send + Sync + 'static,
    {
        self.map_err(|failure| {
            let context = f(&failure);
            failure.context(context)
        })
    }
}

with_std! {
    use crate::Error;

    impl<T> ResultExt<T, Error> for Result<T, Error> {
        fn compat(self) -> Result<T, Compat<Error>> {
            self.map_err(|err| err.compat())
        }

        fn context<D>(self, context: D) -> Result<T, Context<D>> where
            D: Display + Send + Sync + 'static
        {
            self.map_err(|failure| failure.context(context))
        }

        fn with_context<F, D>(self, f: F) -> Result<T, Context<D>> where
            F: FnOnce(&Error) -> D,
            D: Display + Send + Sync + 'static
        {
            self.map_err(|failure| {
                let context = f(&failure);
                failure.context(context)
            })
        }
    }
}
