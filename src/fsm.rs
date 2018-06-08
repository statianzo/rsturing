#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    Idle,
    TimeInput,
    Cook,
    Pause,
    Done,
}

enum Event {
    DigitPressed,
    StopPressed,
    StartPressed,
    TimerElapsed,
}

struct Microwave {
    state: State,
}

impl Microwave {
    fn dispatch(&mut self, event: Event) {
        use self::Event::*;
        use self::State::*;

        self.state = match (self.state, event) {
            (Idle, DigitPressed) => TimeInput,
            (TimeInput, DigitPressed) => TimeInput,
            (TimeInput, StartPressed) => Cook,
            (TimeInput, StopPressed) => Idle,
            (Cook, TimerElapsed) => Done,
            (Cook, StopPressed) => Pause,
            (Pause, StartPressed) => Cook,
            (Pause, StopPressed) => Idle,
            (Done, StopPressed) => Idle,
            _ => self.state,
        }
    }
}

#[test]
fn test_fsm() {
    use self::Event::*;
    use self::State::*;

    let mut mic = Microwave { state: Idle };
    // Punch in numbers
    mic.dispatch(DigitPressed);
    mic.dispatch(DigitPressed);
    mic.dispatch(DigitPressed);
    assert_eq!(mic.state, TimeInput);

    // Start microwave
    mic.dispatch(StartPressed);
    assert_eq!(mic.state, Cook);

    // Check if food is hot
    mic.dispatch(StopPressed);
    assert_eq!(mic.state, Pause);

    // Continue cooking
    mic.dispatch(StartPressed);
    assert_eq!(mic.state, Cook);

    // Timer finished
    mic.dispatch(TimerElapsed);
    assert_eq!(mic.state, Done);

    // Clear "done" message
    mic.dispatch(StopPressed);
    assert_eq!(mic.state, Idle);
}
