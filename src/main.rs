type State = usize;

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Blank,
    Zero,
    One,
}

#[derive(Debug)]
enum Move {
    Stay,
    Left,
    Right,
}

struct Transition {
    start: State,
    end: State,
    read: Cell,
    write: Cell,
    operation: Move,
}

struct Machine {
    initial: State,
    done: State,
    transitions: Vec<Transition>,
}

type Tape = Vec<Cell>;

fn run(machine: Machine, code: Tape) -> Tape {
    let mut position = 0;
    let mut state = machine.initial;
    let mut tape = code.to_vec();
    tape.resize(10, Cell::Blank);

    while state != machine.done {
        let transition;
        {
            let cell = tape.get(position).unwrap_or(&Cell::Blank);
            transition = machine
                .transitions
                .iter()
                .find(|t| t.start == state && &t.read == cell)
                .expect("Missing transition");
        }
        state = transition.end;
        tape[position] = transition.write.clone();
        position = match transition.operation {
            Move::Left => position - 1,
            Move::Right => position + 1,
            Move::Stay => position,
        };
    }

    return tape;
}

#[test]
fn test_build() {
    let machine = Machine {
        initial: 0,
        done: 2,
        transitions: vec![
            Transition {
                start: 0,
                end: 0,
                read: Cell::Zero,
                write: Cell::Zero,
                operation: Move::Right,
            },
            Transition {
                start: 0,
                end: 0,
                read: Cell::One,
                write: Cell::One,
                operation: Move::Right,
            },
            Transition {
                start: 0,
                end: 1,
                read: Cell::Blank,
                write: Cell::Blank,
                operation: Move::Left,
            },
            Transition {
                start: 1,
                end: 1,
                read: Cell::One,
                write: Cell::Zero,
                operation: Move::Left,
            },
            Transition {
                start: 1,
                end: 2,
                read: Cell::Zero,
                write: Cell::One,
                operation: Move::Stay,
            },
            Transition {
                start: 1,
                end: 2,
                read: Cell::Blank,
                write: Cell::One,
                operation: Move::Stay,
            },
        ],
    };

    let result = run(machine, vec![Cell::One, Cell::One, Cell::Zero, Cell::One]);
    println!("{:?}", result);

    let mut expected = vec![Cell::One, Cell::One, Cell::One, Cell::Zero];
    expected.resize(10, Cell::Blank);
    assert_eq!(result, expected);
}
