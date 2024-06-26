/// Represents the minimum value for a roll to succeed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Obstacle(pub usize);

impl From<String> for Obstacle {
    /// Given in the form `Ob|ob{Obstacle Level}`.
    /// No `name` or `checks` field are accepted.
    fn from(value: String) -> Self {
        let quantity = value[2..].parse::<usize>().unwrap_or(1);
        Obstacle(quantity)
    }
}
