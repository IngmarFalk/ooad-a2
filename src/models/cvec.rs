use std::slice::SliceIndex;

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
    T: std::fmt::Display,
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

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.values.iter()
    }
}

impl<T> std::fmt::Display for CVec<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.len;
        let brackets = self.iter().map(|_| "{}").collect::<Vec<&str>>();
        let buf = brackets.join(";");

        f.write_fmt(format_args!(buf, ..self.values,))
    }
}
