#[deny(missing_docs)]

/// `Range<T>`'s with constant step sizes
pub mod step_range {
    macro_rules! declare_step_range {
        ($name: ident, $t: ty) => {
            /// A range with a constant step size
            #[derive(Debug, Clone)]
            pub struct $name<const STEP: $t> {
                start: $t,
                stop: $t,
                state: $t,
            }

            impl<const STEP: $t> $name<STEP> {
                /// Creates a new StepRange
                pub fn new(start: $t, stop: $t) -> Self {
                    Self {
                        start,
                        stop,
                        state: start,
                    }
                }

                /// The start of the range
                pub fn start(&self) -> $t {
                    self.start
                }

                /// The end of the range
                pub fn stop(&self) -> $t {
                    self.stop
                }
            }

            impl<const STEP: $t> Iterator for $name<STEP> {
                type Item = $t;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.start > self.stop {
                        None
                    } else {
                        let current_state = self.state;
                        self.state += STEP;
                        Some(current_state)
                    }
                }
            }
        };
    }

    declare_step_range!(StepRangeU8, u8);
    declare_step_range!(StepRangeU16, u16);
    declare_step_range!(StepRangeU32, u32);
    declare_step_range!(StepRangeU64, u64);
    declare_step_range!(StepRangeU128, u128);
    declare_step_range!(StepRangeUSize, usize);
    declare_step_range!(StepRangeI8, i8);
    declare_step_range!(StepRangeI16, i16);
    declare_step_range!(StepRangeI32, i32);
    declare_step_range!(StepRangeI64, i64);
    declare_step_range!(StepRangeI128, i128);
    declare_step_range!(StepRangeISize, isize);
}

#[cfg(test)]
mod tests {

    #[test]
    fn readme_step_range_examples() {
        use crate::step_range::StepRangeU64;

        // The odd positive integers from 1 to 1000 the traditional way.
        let odd_numbers_std = (1..1000).step_by(2);

        // The odd positive integers from 1 to 1000 with a constant step size.
        let odd_numbers = StepRangeU64::<2>::new(1, 1000);

        let ans = odd_numbers.zip(odd_numbers_std).all(|(a, b)| a == b);
        assert!(ans);
    }
}
