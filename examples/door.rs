use door::*;
use tracing::{debug, trace};
use tracing_subscriber;
use typestate::typestate;

// NB: `#[typestate]` injects a diagram of the state machine in the docs here.

/// Simple state machine with two states and one self-transition, as rendered
/// above.
///
/// `typestate` requires an initial state and a final state. Here we arbtirarily
/// selected `Opened` for both.
///
/// # Example
///
/// Let's create an [`Opened`] door, close it, ring the door bell 3 times, open
/// it again, and finally stop the state machine.
///
/// ```rust
/// let door = Door::<Opened>::initial();
/// let door = door.close();
/// for _ in 0..=2 {
///     door.ring_bell();
/// }
/// let door = door.open();
/// door.r#final();
/// ```
///
/// The execution of the example is reflected in this [`mod@tracing`] output:
///
/// ```
/// DEBUG initial: door: created automaton output=Door { state: Opened }
/// DEBUG close: door: closing self=Door { state: Opened }
/// TRACE door: dropping
/// DEBUG ring_bell: door: 🔔 self=Door { state: Closed }
/// DEBUG ring_bell: door: 🔔 self=Door { state: Closed }
/// DEBUG ring_bell: door: 🔔 self=Door { state: Closed }
/// DEBUG open: door: opening self=Door { state: Closed }
/// TRACE door: dropping
/// DEBUG r#final: door: automaton self-destructing in 3, 2, ... 💥 self=Door { state: Opened }
/// TRACE door: dropping
/// ```
///
/// [`initial`]: OpenedState::initial
/// [`close`]: OpenedState::close
/// [`final`]: OpenedState::final
/// [`open`]: ClosedState::open
/// [`ring_bell`]: ClosedState::ring_bell
#[typestate]
pub mod door {
    /// Automaton with some internal state.
    #[automaton]
    #[derive(Debug)]
    pub struct Door;

    /// An internal state.
    #[state]
    #[derive(Debug)]
    pub struct Opened;

    /// Transition functions for [`Door<Opened>`].
    pub trait Opened {
        /// Initial state for [`Door`].
        fn initial() -> Opened;

        /// Transition from [`Opened`] to [`Closed`].
        fn close(self) -> Closed;

        /// Final state for [`Door`].
        fn r#final(self);
    }

    /// An internal state.
    #[state]
    #[derive(Debug)]
    pub struct Closed;

    /// Transition functions for [`Door<Closed>`].
    pub trait Closed {
        fn open(self) -> Opened;
        fn ring_bell(&self);
    }
}

impl OpenedState for Door<Opened> {
    #[tracing::instrument]
    fn initial() -> Self {
        let door = Self { state: Opened };
        debug!(output = ?door, "created automaton");
        door
    }

    #[tracing::instrument]
    fn close(self) -> Door<Closed> {
        debug!("closing");
        Door::<Closed> { state: Closed }
    }

    #[tracing::instrument]
    fn r#final(self) {
        debug!("automaton self-destructing in 3, 2, ... 💥");
    }
}

impl ClosedState for Door<Closed> {
    #[tracing::instrument]
    fn open(self) -> Door<Opened> {
        debug!("opening");
        Door::<Opened> { state: Opened }
    }

    #[tracing::instrument]
    fn ring_bell(&self) {
        debug!("🔔");
    }
}

impl<T: DoorState> Drop for Door<T> {
    fn drop(&mut self) {
        trace!("dropping");
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .without_time()
        .compact()
        .init();

    let door = Door::<Opened>::initial();
    let door = door.close();
    for _ in 0..=2 {
        door.ring_bell();
    }
    let door = door.open();
    door.r#final()
}
