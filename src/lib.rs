#![feature(futures_api)]

use core::pin::Pin;
use futures::{prelude::*, task::*};

fn step(v: u128) -> u128 {
    if v % 2 == 0 {
        v / 2
    } else {
        v * 3 + 1
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComputationStatus {
    finished: bool,
    start: u128,
    highest: u128,
    value: u128,
    n: u128,
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
    pub fn new(start: u128) -> Self {
        Self {
            status: ComputationStatus {
                finished: false,
                start,
                highest: start,
                value: start,
                n: 0,
            },
        }
    }
}

impl Stream for Computation {
    type Item = ComputationStatus;

    fn poll_next(mut self: Pin<&mut Self>, _waker: &Waker) -> Poll<Option<Self::Item>> {
        if self.status.value == 1 && !self.status.finished {
            self.status.finished = true;
            return Poll::Ready(Some(self.status.clone()));
        }

        if self.status.value < 1 || self.status.finished {
            return Poll::Ready(None);
        }

        self.status = self.status.clone().step();

        Poll::Ready(Some(self.status.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn test() {
        let fixtures = vec![
            (
                ComputationStatus {
                    finished: true,
                    start: 1,
                    highest: 1,
                    value: 1,
                    n: 0,
                },
                1,
            ),
            (
                ComputationStatus {
                    finished: true,
                    start: 9,
                    highest: 52,
                    value: 1,
                    n: 19,
                },
                9,
            ),
            (
                ComputationStatus {
                    finished: true,
                    start: 670617279,
                    highest: 966616035460,
                    value: 1,
                    n: 986,
                },
                670617279,
            ),
        ];

        for (expectation, input) in fixtures {
            let result = block_on(Computation::new(input).collect::<Vec<_>>())
                .last()
                .cloned()
                .unwrap();

            assert_eq!(expectation, result);
        }
    }
}
