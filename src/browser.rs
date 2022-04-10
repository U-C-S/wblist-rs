#[derive(Debug, Clone, Copy)]
pub struct Browser<T>
where
    T: AsRef<str> + ToString,
{
    pub full_name: T,
    pub short_name: T,
    pub path: T,
}

impl<T> Browser<T>
where
    T: AsRef<str> + ToString,
{
    pub fn new(full_name: T, short_name: T, path: T) -> Self {
        Self {
            full_name,
            short_name,
            path,
        }
    }
}
