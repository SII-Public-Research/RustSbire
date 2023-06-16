#![feature(async_fn_in_trait)]

/// `Arg` is the type passed to the task; to pass multiple arguments, use a tuple.
pub trait Component<Arg> {
    type Error;

    async fn run(arg: Arg) -> Result<(), Self::Error>;
}
