pub trait BFCell: Clone + Copy + Eq + PartialEq {
    fn zero() -> Self;
    fn one() -> Self;

    fn add(&mut self, rhs: Self);
    fn sub(&mut self, rhs: Self);

    fn iszero(self) -> bool;

    fn input(&mut self, bfinput: &mut impl std::io::Read) -> Result<(), std::io::Error>;
    fn output(self, bfoutput: &mut impl std::io::Write) -> Result<(), std::io::Error>;
}
