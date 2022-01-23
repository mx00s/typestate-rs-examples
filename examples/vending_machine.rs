use nonzero_biguint::*;
use num_bigint::BigUint;
use num_traits::One;
use tracing::{debug, instrument, trace};
use typestate::typestate;
use vending_machine::*;

// NB: `#[typestate]` injects a diagram of the state machine in the docs here.

#[typestate]
pub mod vending_machine {
    use super::nonzero_biguint::NonZeroBigUint;
    use num_bigint::BigUint;

    /// Automaton with some internal state.
    #[automaton]
    #[derive(Clone, Debug)]
    pub struct VendingMachine;

    #[state]
    #[derive(Clone, Debug)]
    pub struct CoinsAndChocolates {
        pub coins: NonZeroBigUint,
        pub chocolates: NonZeroBigUint,
    }
    pub trait CoinsAndChocolates {
        fn insert_coin(self) -> CoinsAndChocolates;
        fn vend(self) -> VendResult;
        fn get_coins(self) -> NoCoinsButChocolates;
    }

    #[derive(Clone, Debug)]
    pub enum VendResult {
        CoinsAndChocolates,
        NoCoinsButChocolates,
        CoinsButNoChocolates,
        NoCoinsNorChocolates,
    }

    #[state]
    #[derive(Clone, Debug)]
    pub struct NoCoinsButChocolates {
        pub chocolates: NonZeroBigUint,
    }
    pub trait NoCoinsButChocolates {
        fn insert_coin(self) -> CoinsAndChocolates;
        fn get_coins(self) -> NoCoinsButChocolates;
        fn refill(self, bars: BigUint) -> NoCoinsButChocolates;
    }

    #[state]
    #[derive(Clone, Debug)]
    pub struct CoinsButNoChocolates {
        pub coins: NonZeroBigUint,
    }
    pub trait CoinsButNoChocolates {
        fn insert_coin(self) -> CoinsButNoChocolates;
        fn get_coins(self) -> NoCoinsNorChocolates;
    }

    #[state]
    #[derive(Clone, Debug)]
    pub struct NoCoinsNorChocolates;
    pub trait NoCoinsNorChocolates {
        fn initial() -> NoCoinsNorChocolates;
        fn insert_coin(self) -> CoinsButNoChocolates;
        fn get_coins(self) -> NoCoinsNorChocolates;
        fn refill(self, bars: NonZeroBigUint) -> NoCoinsButChocolates;
        fn r#final(self);
    }
}

impl CoinsAndChocolatesState for VendingMachine<CoinsAndChocolates> {
    #[instrument]
    fn insert_coin(self) -> VendingMachine<CoinsAndChocolates> {
        let result = VendingMachine {
            state: CoinsAndChocolates {
                coins: self.state.coins.increment(),
                ..self.state
            },
        };
        debug!(?result, "inserted coin");
        result
    }

    #[instrument]
    fn vend(self) -> VendResult {
        let result = match (
            self.state.coins.decrement(),
            self.state.chocolates.decrement(),
        ) {
            (None, None) => {
                trace!("last coin and chocolate left!");
                VendResult::NoCoinsNorChocolates(VendingMachine {
                    state: NoCoinsNorChocolates,
                })
            }
            (None, Some(chocolates)) => {
                trace!(?chocolates, "last coin left!");
                VendResult::NoCoinsButChocolates(VendingMachine {
                    state: NoCoinsButChocolates { chocolates },
                })
            }
            (Some(coins), None) => {
                trace!(?coins, "last chocolate left!");
                VendResult::CoinsButNoChocolates(VendingMachine {
                    state: CoinsButNoChocolates { coins },
                })
            }
            (Some(coins), Some(chocolates)) => {
                trace!(?coins, ?chocolates, "not the last coin nor chocolate left!");
                VendResult::CoinsAndChocolates(VendingMachine {
                    state: CoinsAndChocolates { coins, chocolates },
                })
            }
        };
        debug!(?result, "vended a chocolate");
        result
    }

    #[instrument]
    fn get_coins(self) -> VendingMachine<NoCoinsButChocolates> {
        let result = VendingMachine {
            state: NoCoinsButChocolates {
                chocolates: self.state.chocolates,
            },
        };
        debug!(?result, coins = ?self.state.coins, "collected coins");
        result
    }
}

impl NoCoinsButChocolatesState for VendingMachine<NoCoinsButChocolates> {
    #[instrument]
    fn insert_coin(self) -> VendingMachine<CoinsAndChocolates> {
        let result = VendingMachine {
            state: CoinsAndChocolates {
                coins: NonZeroBigUint::one(),
                chocolates: self.state.chocolates,
            },
        };
        debug!(?result, "inserted coin");
        result
    }

    #[instrument]
    fn get_coins(self) -> VendingMachine<NoCoinsButChocolates> {
        let result = self;
        debug!(?result, "collected 0 coins");
        result
    }

    #[instrument]
    fn refill(self, bars: BigUint) -> VendingMachine<NoCoinsButChocolates> {
        let result = VendingMachine {
            state: NoCoinsButChocolates {
                chocolates: self.state.chocolates + bars,
            },
        };
        debug!(?result, "restocked with chocolates");
        result
    }
}

impl NoCoinsNorChocolatesState for VendingMachine<NoCoinsNorChocolates> {
    #[instrument]
    fn initial() -> VendingMachine<NoCoinsNorChocolates> {
        let result = VendingMachine {
            state: NoCoinsNorChocolates,
        };
        debug!(?result, "created a vending machine");
        result
    }

    #[instrument]
    fn insert_coin(self) -> VendingMachine<CoinsButNoChocolates> {
        let result = VendingMachine {
            state: CoinsButNoChocolates {
                coins: NonZeroBigUint::one(),
            },
        };
        debug!(?result, "inserted coin");
        result
    }

    #[instrument]
    fn get_coins(self) -> VendingMachine<NoCoinsNorChocolates> {
        let result = self;
        debug!(?result, "collected 0 coins");
        result
    }

    #[instrument]
    fn refill(self, bars: NonZeroBigUint) -> VendingMachine<NoCoinsButChocolates> {
        let result = VendingMachine {
            state: NoCoinsButChocolates { chocolates: bars },
        };
        debug!(?result, "restocked with chocolates");
        result
    }

    #[instrument]
    fn r#final(self) {
        debug!("reached final state");
    }
}

impl CoinsButNoChocolatesState for VendingMachine<CoinsButNoChocolates> {
    #[instrument]
    fn insert_coin(self) -> VendingMachine<CoinsButNoChocolates> {
        let result = VendingMachine {
            state: CoinsButNoChocolates {
                coins: self.state.coins.increment(),
            },
        };
        debug!(?result, "inserted coin");
        result
    }

    #[instrument]
    fn get_coins(self) -> VendingMachine<NoCoinsNorChocolates> {
        let result = VendingMachine {
            state: NoCoinsNorChocolates,
        };
        debug!(?result, coins = ?self.state.coins, "collected coins");
        result
    }
}

mod nonzero_biguint {
    use num_bigint::BigUint;
    use num_traits::{CheckedSub, One, Zero};
    use std::ops::{Add, Mul};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct NonZeroBigUint(BigUint);

    impl NonZeroBigUint {
        pub fn new(n: BigUint) -> Option<Self> {
            n.try_into().ok()
        }

        pub fn decrement(&self) -> Option<Self> {
            match self.inner().checked_sub(&BigUint::one()) {
                None => unreachable!(),
                Some(n) if n.is_zero() => None,
                Some(n) => Some(Self(n)),
            }
        }

        pub fn increment(&self) -> Self {
            Self(self.0.clone() + BigUint::one())
        }

        pub fn inner(&self) -> &BigUint {
            &self.0
        }
    }

    impl TryFrom<BigUint> for NonZeroBigUint {
        type Error = ();
        fn try_from(n: BigUint) -> Result<Self, Self::Error> {
            (!n.is_zero()).then(|| NonZeroBigUint(n)).ok_or(())
        }
    }

    impl TryFrom<usize> for NonZeroBigUint {
        type Error = ();
        fn try_from(n: usize) -> Result<Self, Self::Error> {
            BigUint::from(n).try_into()
        }
    }

    impl One for NonZeroBigUint {
        fn one() -> Self {
            Self(BigUint::one())
        }
    }

    impl Add<BigUint> for NonZeroBigUint {
        type Output = Self;
        fn add(self, other: BigUint) -> Self {
            Self(self.inner() + other)
        }
    }

    impl Mul for NonZeroBigUint {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            Self(self.inner() * other.inner())
        }
    }
}

fn main() -> Result<(), ()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .without_time()
        .compact()
        .init();

    let vm = VendingMachine::<NoCoinsNorChocolates>::initial();
    let vm = vm.insert_coin();
    let vm = vm.insert_coin();
    let vm = vm.get_coins();
    let vm = vm.get_coins();
    let vm = vm.refill(3.try_into().unwrap());
    let vm = vm.insert_coin();
    let vm = match vm.vend() {
        VendResult::NoCoinsButChocolates(vm) => vm,
        _ => return Err(()),
    };
    let vm = vm.insert_coin();
    let vm = vm.insert_coin();
    let vm = match vm.vend() {
        VendResult::CoinsAndChocolates(vm) => vm,
        _ => return Err(()),
    };
    let vm = match vm.vend() {
        VendResult::NoCoinsNorChocolates(vm) => vm,
        _ => return Err(()),
    };
    vm.r#final();
    Ok(())
}
