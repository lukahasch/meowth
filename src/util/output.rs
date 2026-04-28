use iced::Task;

#[derive(Debug)]
pub enum Output<T>
where
    T: Send,
{
    Message(T),
    Task(Task<T>),
    Batch(Vec<Output<T>>),
}

impl<T> Output<T>
where
    T: Send + 'static,
{
    pub fn message(message: T) -> Self {
        Output::Message(message)
    }

    pub fn none() -> Self {
        Output::Batch(vec![])
    }

    pub fn task(task: Task<T>) -> Self {
        Output::Task(task)
    }

    pub fn future(future: impl Future<Output = T> + Send + 'static) -> Self {
        Self::Task(Task::future(future))
    }

    pub fn map<U, F>(self, f: F) -> Output<U>
    where
        U: Send + 'static,
        F: Fn(T) -> U + Send + Clone + 'static,
    {
        match self {
            Output::Message(message) => Output::Message(f(message)),
            Output::Task(task) => Output::Task(task.map(f)),
            Output::Batch(batch) => Output::Batch(
                batch
                    .into_iter()
                    .map(|output| output.map(f.clone()))
                    .collect(),
            ),
        }
    }

    pub fn flatten(self) -> Output<T> {
        match self {
            Output::Batch(mut batch) if batch.len() == 1 => batch.pop().unwrap().flatten(),
            Output::Batch(batch) => Output::Batch(
                batch
                    .into_iter()
                    .flat_map(|output| output.into_iter())
                    .map(Self::flatten)
                    .collect(),
            ),
            output => output,
        }
    }

    /// Applays f to Self::Message and returns a Task for every Self::Task
    pub fn realise(self, f: &mut impl FnMut(T) -> Output<T>) -> Option<Task<T>> {
        match self {
            Output::Message(message) => f(message).realise(f),
            Output::Task(task) => Some(task),
            Output::Batch(batch) => Some(Task::batch(
                batch.into_iter().filter_map(|output| output.realise(f)),
            )),
        }
    }
}

impl<T> IntoIterator for Output<T>
where
    T: Send,
{
    type Item = Output<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Output::Batch(batch) => batch.into_iter(),
            output => vec![output].into_iter(),
        }
    }
}
