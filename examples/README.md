Examples are ported from [_Type-Driven Development with Idris_](https://www.manning.com/books/type-driven-development-with-idris) by Edwin Brady.

# [`door`](./door.rs)

Simple state machine with two states, `Opened` and `Closed`.

![`door` state machine diagram](../generated/Door.dot.png)


# [`vending_machine`](./vending_machine.rs)

Vending machine with four states:

1. `CoinsAndChocolates`
1. `NoCoinsButChocolates`
1. `CoinsButNoChocolates`
1. `NoCoinsNorChocolates`

States with coins denote the number of coins that have been inserted and can either be exchanged for a chocolate or returned to the user. A vending machine may only vend a chocolate if there is at least one coin and one chocolate to facilitate the exchange. After vending a chocolate the vending machine could be in any state, hence the `VendingResult` enum.

Unlike in Idris, there's not enough static context associated with the states for the compiler to deduce the next state, and consequently there's a risk that implementations of the `vend` transition won't preserve the desired contract, namely that the number of coins and chocolates are each decremented. It might be possible to provide more context to the compiler, e.g. with const generics, to encode the number of coins and chocolates, potentially with a type-level unary encoding like Idris' `Nat`s.

![`vending_machine` state machine diagram](../generated/VendingMachine.dot.png)
