enum State {
    New
}

pub fn ex_enums () {
    let state = State::New;
}

fn some_enum (some: &State) {
    match some {
        State::New => todo!()
    }
}