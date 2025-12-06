pub enum Queue {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum QueueType {
    Direct,
    Compute,
    Transfer,
}
