pub trait FromEnv: Sized {
    fn from_env() -> Result<Self, String>;
}
