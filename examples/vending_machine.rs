#![feature(nonzero_ops)]

use std::num::NonZeroUsize;

use tracing::{debug, instrument, trace};
use typestate::typestate;
use vending_machine::*;

// NB: `#[typestate]` injects a diagram of the state machine in the docs here.

// TODO: Prevent overflows, either by implementing a Error and Result types or
// switching to an arbitrary precision unsized integer.

#[typestate]
pub mod vending_machine {
    use std::num::NonZeroUsize;

    /// Automaton with some internal state.
    #[automaton]
    #[derive(Clone, Debug)]
    pub struct VendingMachine;

    #[state]
    #[derive(Clone, Debug)]
    pub struct CoinsAndChocolates {
        pub coins: NonZeroUsize,
        pub chocolates: NonZeroUsize,
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
        pub chocolates: NonZeroUsize,
    }
    pub trait NoCoinsButChocolates {
        fn insert_coin(self) -> CoinsAndChocolates;
        fn get_coins(self) -> NoCoinsButChocolates;
        fn refill(self, bars: NonZeroUsize) -> NoCoinsButChocolates;
    }

    #[state]
    #[derive(Clone, Debug)]
    pub struct CoinsButNoChocolates {
        pub coins: NonZeroUsize,
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
        fn refill(self, bars: NonZeroUsize) -> NoCoinsButChocolates;
        fn r#final(self);
    }
}

const ONE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };

impl CoinsAndChocolatesState for VendingMachine<CoinsAndChocolates> {
    #[instrument]
    fn insert_coin(self) -> VendingMachine<CoinsAndChocolates> {
        let result = VendingMachine {
            state: CoinsAndChocolates {
                coins: unsafe { self.state.coins.unchecked_add(1usize) },
                chocolates: self.state.chocolates,
            },
        };
        debug!(?result, "inserted coin");
        result
    }

    #[instrument]
    fn vend(self) -> VendResult {
        let result = match (self.state.coins.get(), self.state.chocolates.get()) {
            (1, 1) => {
                trace!("last coin and chocolate left!");
                VendResult::NoCoinsNorChocolates(VendingMachine {
                    state: NoCoinsNorChocolates,
                })
            }
            (1, chocolates) => {
                trace!("last coin left!");
                VendResult::NoCoinsButChocolates(VendingMachine {
                    state: NoCoinsButChocolates {
                        chocolates: unsafe { NonZeroUsize::new_unchecked(chocolates - 1) },
                    },
                })
            }
            (coins, 1) => {
                trace!("last chocolate left!");
                VendResult::CoinsButNoChocolates(VendingMachine {
                    state: CoinsButNoChocolates {
                        coins: unsafe { NonZeroUsize::new_unchecked(coins - 1) },
                    },
                })
            }
            (coins, chocolates) => {
                trace!("not the last coin nor chocolate left!");
                VendResult::CoinsAndChocolates(VendingMachine {
                    state: CoinsAndChocolates {
                        coins: unsafe { NonZeroUsize::new_unchecked(coins - 1) },
                        chocolates: unsafe { NonZeroUsize::new_unchecked(chocolates - 1) },
                    },
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
        let coins = self.state.coins;
        debug!(?result, coins, "collected coins");
        result
    }
}

impl NoCoinsButChocolatesState for VendingMachine<NoCoinsButChocolates> {
    #[instrument]
    fn insert_coin(self) -> VendingMachine<CoinsAndChocolates> {
        let result = VendingMachine {
            state: CoinsAndChocolates {
                coins: ONE,
                chocolates: self.state.chocolates,
            },
        };
        debug!(?result, "inserted coin");
        result
    }

    #[instrument]
    fn get_coins(self) -> VendingMachine<NoCoinsButChocolates> {
        let result = VendingMachine {
            state: NoCoinsButChocolates {
                chocolates: self.state.chocolates,
            },
        };
        debug!(?result, "collected 0 coins");
        result
    }

    #[instrument]
    fn refill(self, bars: std::num::NonZeroUsize) -> VendingMachine<NoCoinsButChocolates> {
        let result = VendingMachine {
            state: NoCoinsButChocolates {
                chocolates: unsafe { self.state.chocolates.unchecked_add(bars.get()) },
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
            state: CoinsButNoChocolates { coins: ONE },
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
    fn refill(self, bars: std::num::NonZeroUsize) -> VendingMachine<NoCoinsButChocolates> {
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
                coins: unsafe { self.state.coins.unchecked_add(1usize) },
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
        let coins = self.state.coins;
        debug!(?result, coins, "collected coins");
        result
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
    let vm = vm.refill(unsafe { NonZeroUsize::new_unchecked(3) });
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
