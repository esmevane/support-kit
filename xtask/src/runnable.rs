pub trait Runnable {
    fn run(&self);
}

impl<T> Runnable for T
where
    T: Fn(),
{
    fn run(&self) {
        self();
    }
}

impl<T> Runnable for Option<T>
where
    T: Runnable,
{
    fn run(&self) {
        if let Some(runnable) = self {
            runnable.run();
        }
    }
}

impl<T, E> Runnable for Result<T, E>
where
    T: Runnable,
    E: std::error::Error,
{
    fn run(&self) {
        match self {
            Ok(runnable) => runnable.run(),
            Err(error) => panic!("{}", error),
        }
    }
}

impl<T> Runnable for [T]
where
    T: Runnable,
{
    fn run(&self) {
        for runnable in self {
            runnable.run();
        }
    }
}

impl<T> Runnable for Vec<T>
where
    T: Runnable,
{
    fn run(&self) {
        for runnable in self {
            runnable.run();
        }
    }
}
