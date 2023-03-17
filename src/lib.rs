#![feature(async_fn_in_trait)]

/// Composant par défaut, A est le type de variable passée à la tâche (peut être un tuple, notamment).
pub trait Component<A> {
    type Error;

    async fn run(arg: A) -> Result<(), Self::Error>;
}
