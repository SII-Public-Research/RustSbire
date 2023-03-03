#![feature(async_fn_in_trait)]

/// Composant par défaut, A est le type de variable passée à la tâche (peut être un tuple, notamment).
pub trait Component<A> {
    async fn run(arg: A);
}
