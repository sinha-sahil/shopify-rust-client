use std::fmt;

#[derive(Debug, Clone)]
pub enum DateFilter {
    Exact(String),
    Before(String),
    After(String),
    OnOrBefore(String),
    OnOrAfter(String),
    Range(String, String),
}

impl fmt::Display for DateFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateFilter::Exact(d) => write!(f, "{}", d),
            DateFilter::Before(d) => write!(f, "<{}", d),
            DateFilter::After(d) => write!(f, ">{}", d),
            DateFilter::OnOrBefore(d) => write!(f, "<={}", d),
            DateFilter::OnOrAfter(d) => write!(f, ">={}", d),
            DateFilter::Range(from, to) => write!(f, "{}..{}", from, to),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NumericFilter<T: fmt::Display> {
    Exact(T),
    GreaterThan(T),
    LessThan(T),
    GreaterOrEqual(T),
    LessOrEqual(T),
}

impl<T: fmt::Display> fmt::Display for NumericFilter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumericFilter::Exact(v) => write!(f, "{}", v),
            NumericFilter::GreaterThan(v) => write!(f, ">{}", v),
            NumericFilter::LessThan(v) => write!(f, "<{}", v),
            NumericFilter::GreaterOrEqual(v) => write!(f, ">={}", v),
            NumericFilter::LessOrEqual(v) => write!(f, "<={}", v),
        }
    }
}
