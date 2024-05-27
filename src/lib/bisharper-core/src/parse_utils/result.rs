use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BParseResult<T, E> {
    output: Option<T>,
    errs: Vec<E>,
}

impl<T, E> BParseResult<T, E> {
    pub fn new(output: Option<T>, errs: Vec<E>) -> BParseResult<T, E> {
        BParseResult { output, errs }
    }

    pub fn has_output(&self) -> bool {
        self.output.is_some()
    }

    pub fn has_errors(&self) -> bool {
        !self.errs.is_empty()
    }

    pub fn output(&self) -> Option<&T> {
        self.output.as_ref()
    }

    pub fn errors(&self) -> impl ExactSizeIterator<Item = &E> + DoubleEndedIterator {
        self.errs.iter()
    }

    pub fn into_output(self) -> Option<T> {
        self.output
    }

    pub fn into_errors(self) -> Vec<E> {
        self.errs
    }

    pub fn into_output_errors(self) -> (Option<T>, Vec<E>) {
        (self.output, self.errs)
    }

    pub fn into_result(self) -> Result<T, Vec<E>> {
        if self.errs.is_empty() {
            self.output.ok_or(self.errs)
        } else {
            Err(self.errs)
        }
    }

    #[track_caller]
    pub fn unwrap(self) -> T
        where
            E: fmt::Debug,
    {
        if self.has_errors() {
            panic!(
                "called `ParseResult::unwrap` on a parse result containing errors: {:?}",
                &self.errs
            )
        } else {
            self.output.expect("parser generated no errors or output")
        }
    }
}

impl<T, E> From<Result<T, E>> for BParseResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(output) => BParseResult::new(Some(output), Vec::new()),
            Err(err) => BParseResult::new(None, vec![err]),
        }
    }
}

impl<T, E> From<Result<T, Vec<E>>> for BParseResult<T, E> {
    fn from(result: Result<T, Vec<E>>) -> Self {
        match result {
            Ok(output) => BParseResult::new(Some(output), Vec::new()),
            Err(errs) => BParseResult::new(None, errs),
        }
    }
}