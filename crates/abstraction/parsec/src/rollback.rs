use crate::*;

pub trait ParseFromWithRollback<Context>: ParseFrom<Context>
where
    Context: ParseContext + ?Sized,
{
    type Error;
    fn parse_from_with_rollback_when_no_error<'a>(
        stream: &mut Context,
    ) -> Result<Option<Self>, <Self as ParseFromWithRollback<Context>>::Error>;

    fn try_parse_from_with_rollback<'a>(stream: &mut Context) -> Option<Self>;
}

impl<Context, P> ParseFromWithRollback<Context> for P
where
    Context: ParseContext + ?Sized,
    P: ParseFrom<Context>,
{
    type Error = <P as ParseFrom<Context>>::Error;
    fn parse_from_with_rollback_when_no_error<'a>(
        stream: &mut Context,
    ) -> Result<Option<Self>, <P as ParseFrom<Context>>::Error> {
        let state = stream.save_state();
        let result = Self::parse_from_without_guaranteed_rollback(stream);
        match result {
            // rollback for no pattern
            Ok(None) => stream.rollback(state),
            _ => (),
        }
        result
    }

    fn try_parse_from_with_rollback<'a>(stream: &mut Context) -> Option<Self> {
        let state = stream.save_state();
        let result = Self::parse_from_without_guaranteed_rollback(stream);
        match result {
            Ok(Some(patt)) => Some(patt),
            Ok(None) | Err(_) => {
                stream.rollback(state);
                None
            }
        }
    }
}
