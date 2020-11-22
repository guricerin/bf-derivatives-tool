#[derive(Debug)]
pub enum Token {
    RShift,
    LShift,
    Inc,
    Dec,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
    Ignore,
}
