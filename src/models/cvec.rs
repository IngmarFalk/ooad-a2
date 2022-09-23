#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct CVec<T>
where
    T: std::fmt::Display,
{
    pub values: Vec<T>,
    pub len: usize,
}

impl<T> CVec<T>
where
    T: std::fmt::Display + PartialEq + Clone,
{
    pub fn new() -> CVec<T> {
        CVec {
            values: Vec::new(),
            len: 0,
        }
    }

    pub fn push(&self, val: T) {
        self.values.push(val);
    }

    pub fn remove(&self, val: T) -> Option<T> {
        match self.index_of(val) {
            Some(i) => Some(self.values.remove(i)),
            None => None,
        }
    }

    pub fn index_of(&self, val: T) -> Option<usize> {
        self.iter().position(|&v| v == val)
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.values.iter()
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.values.clone()
    }
}

impl<T> std::fmt::Display for CVec<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.values.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{},", album))
        })
    }
}
