extern crate futures;

use futures::prelude::*;

fn step(v: u64) -> u64 {
    if v % 2 == 0 {
        v / 2
    } else {
        v * 3 + 1
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComputationStatus {
    finished: bool,
    start: u64,
    highest: u64,
    value: u64,
    n: u64,
}

impl ComputationStatus {
    fn step(mut self) -> Self {
        let old = self.value;
        let new = step(old);
        if new > self.highest {
            self.highest = new;
        }
        self.value = new;
        self.n += 1;
        self
    }
}

pub struct Computation {
    status: ComputationStatus,
}

impl Computation {
    pub fn new(start: u64) -> Self {
        Self {
            status: ComputationStatus {
                finished: false,
                start,
                highest: start,
                value: start,
                n: 0,
            }
        }
    }
}

impl Stream for Computation {
    type Item = ComputationStatus;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.status.value == 1 && !self.status.finished {
            self.status.finished = true;
            return Ok(Async::Ready(Some(self.status.clone())));
        }

        if self.status.value < 1 || self.status.finished {
            return Ok(Async::Ready(None));
        }

        self.status = self.status.clone().step();

        Ok(Async::Ready(Some(self.status.clone())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let fixtures = [
            (ComputationStatus {
                finished: true,
                start: 1,
                highest: 1,
                value: 1,
                n: 0,
            }, 1),
            (ComputationStatus {
                finished: true,
                start: 9,
                highest: 52,
                value: 1,
                n: 19,
            }, 9),
            (ComputationStatus {
                finished: true,
                start: 670617279,
                highest: 966616035460,
                value: 1,
                n: 986,
            }, 670617279),
        ];

        for &(ref expectation, ref input) in fixtures.into_iter() {
            let result = Computation::new(*input).wait().last().unwrap().unwrap();

            assert_eq!(*expectation, result);
        }

    }
}