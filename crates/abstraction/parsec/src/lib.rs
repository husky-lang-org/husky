mod rollback;
mod separated_list;
mod seq;
#[cfg(test)]
mod tests;

pub use rollback::*;
pub use separated_list::*;

use std::ops::DerefMut;
#[cfg(test)]
use tests::*;

pub trait ParseStream {
    type State;
    fn save_state(&self) -> Self::State;
    fn rollback(&mut self, state: Self::State);
}

pub trait StreamWrapper: std::ops::DerefMut
where
    Self::Target: ParseStream,
{
}

impl<Wrapper> ParseStream for Wrapper
where
    Wrapper: StreamWrapper,
    Wrapper::Target: ParseStream,
{
    type State = <<Wrapper as std::ops::Deref>::Target as ParseStream>::State;

    fn save_state(&self) -> Self::State {
        self.deref().save_state()
    }

    fn rollback(&mut self, state: Self::State) {
        self.deref_mut().rollback(state)
    }
}

pub trait ParseContext: ParseStream {
    fn parse<P: ParseFrom<Self>>(&mut self) -> Result<Option<P>, P::Error>;
    fn parse_expected<P: ParseFrom<Self>, Error>(
        &mut self,
        err: impl FnOnce(Self::State) -> Error,
    ) -> Result<P, Error>
    where
        Error: From<P::Error>;

    /// returns an optional and the rest of the stream,
    ///
    /// guarantees that stream state is not changed if result is Ok(None)
    fn parse_into<P: ParseFrom<Self>>(self) -> Result<(Option<P>, Self), P::Error>
    where
        Self: Sized;
}

impl<T> ParseContext for T
where
    T: ParseStream,
{
    fn parse<P: ParseFrom<Self>>(&mut self) -> Result<Option<P>, P::Error> {
        P::parse_from_with_rollback(self)
    }

    fn parse_expected<P: ParseFrom<Self>, Error>(
        &mut self,
        err: impl FnOnce(Self::State) -> Error,
    ) -> Result<P, Error>
    where
        Error: From<P::Error>,
    {
        let saved_state = self.save_state();
        match P::parse_from_with_rollback(self)? {
            Some(output) => Ok(output),
            None => Err(err(saved_state)),
        }
    }

    fn parse_into<P: ParseFrom<Self>>(mut self) -> Result<(Option<P>, Self), P::Error> {
        let optional = P::parse_from_with_rollback(&mut self)?;
        Ok((optional, self))
    }
}

pub trait ParseFrom<Context>: Sized
where
    Context: ParseContext + ?Sized,
{
    type Error;
    /// no guarantee on stream state other than Ok(Some(_))
    fn parse_from_without_guaranteed_rollback<'a>(
        ctx: &mut Context,
    ) -> Result<Option<Self>, Self::Error>;
}
