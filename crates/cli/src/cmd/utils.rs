/// Common trait for all cli commands
pub trait Cmd: clap::Parser + Sized {
    type Output;
    fn run(self) -> eyre::Result<Self::Output>;
}

#[async_trait::async_trait]
pub trait AsyncCmd: clap::Parser + Sized {
    type Output;

    async fn run(self) -> eyre::Result<Self::Output>;
}
