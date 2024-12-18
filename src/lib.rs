use axum::extract::State;

#[derive(Clone)]
pub struct SharedState<T: SomeThingT + Clone> {
    pub name: String,
    pub something: T,
}

impl<T: SomeThingT + Clone> SharedStateT for SharedState<T> {
    fn func_one(&self) -> String {
        self.name.clone()
    }

    fn func_two(&self) -> impl SomeThingT + Clone {
        self.something.clone()
    }
}

pub trait SharedStateT {
    fn func_one(&self) -> String;
    fn func_two(&self) -> impl SomeThingT + Clone;
}

pub trait SomeThingT {
    fn does_something(&self);
}

#[derive(Clone)]
pub struct AnotherStruct {}

impl SomeThingT for AnotherStruct {
    fn does_something(&self) {
        println!("Doing somehting");
    }
}

pub fn middleware_fn<R: SomeThingT + Clone>(State(state): State<SharedState<R>>) {
    let val = state.func_two();

    val.does_something();
}
