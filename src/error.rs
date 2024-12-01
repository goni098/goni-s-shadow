#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Invocation(#[from] grammers_client::InvocationError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Authorization(#[from] grammers_client::client::bots::AuthorizationError),

    #[error(transparent)]
    SignIn(#[from] Box<grammers_client::SignInError>),

    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    Env(#[from] dotenv::Error),

    #[error(transparent)]
    Var(#[from] std::env::VarError),
}

pub type Rs<A> = Result<A, Error>;
