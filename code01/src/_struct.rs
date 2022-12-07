#[derive(Debug)]
enum State {
    New,
    Delete,
}

#[derive(Debug)]
struct Gues {
    name: String,
    verifed: bool,
}

impl Gues {
    fn new(name: String, verifed: bool) -> Gues {
        Gues { name, verifed }
    }

    fn delete(gues: Gues) {
        let mut vec = Vec::new();
        vec.push(gues);
    }
}

#[derive(Debug)]
struct GuesState {
    state: Option<State>,
}

pub fn ex_struct() {
    let mut gues = Vec::new();
    let state = State::New;
    let _ = match state {
        State::New => {
            let new_gues = Gues {
                name: "".to_string(),
                verifed: false,
            };
            gues.push(new_gues);
            println!("Create gues state succes!");
        }
        State::Delete => {
            gues.pop();
            println!("Delete state succes!");
        }
    };
    let gues_state = Box::new(GuesState { state: Some(state) });
}
