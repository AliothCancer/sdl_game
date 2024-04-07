


#[derive(Debug,Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharStatistic{
    Speed(StatEdit),
    Hp(StatEdit)
}


#[derive(Debug,Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatEdit{
    Increase(u16),
    Decrease(u16)
}