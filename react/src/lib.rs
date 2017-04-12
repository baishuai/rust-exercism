use std::collections::HashMap;

#[allow(unused_variables)]
// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = usize;


type ComputeFunc<T> = Fn(&[T]) -> T;

struct Computation<T> {
    inputs: Vec<CellID>,
    compute_func: Box<ComputeFunc<T>>,
}

enum CellType<T> {
    Input,
    Compute(Computation<T>),
}

struct Cell<'a, T> {
    value: T,
    last_value: T,
    deps: Vec<CellID>,
    kind: CellType<T>,
    callbacks: HashMap<CallbackID, Box<FnMut(T) -> () + 'a>>,
}

impl<'a, T: Copy + PartialEq> Cell<'a, T> {
    fn new_input(initial: T) -> Self {
        Cell {
            value: initial,
            last_value: initial,
            deps: Vec::new(),
            kind: CellType::Input,
            callbacks: HashMap::new(),
        }
    }

    //    fn new_compute<F: Fn(&[T]) -> T + 'a>(initial: T,
    //                                          dependencies: &[CellID],
    //                                          compute_func: Box<F>)
    //                                          -> Self {
    //        Cell {
    //            value: initial,
    //            last_value: initial,
    //            deps: Vec::new(),
    //            kind: CellType::Compute(Computation {
    //                inputs: dependencies.into(),
    //                compute_func: Box::new(compute_func),
    //            }),
    //            callbacks: HashMap::new(),
    //        }
    //    }
}


pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    cells: Vec<Cell<'a, T>>,
    callback_id_counter: CallbackID,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            cells: Vec::new(),
            callback_id_counter: 0
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        // let id = self.cells.len();
        // self.cells.push(initial);
        // id
        let cell = Cell::new_input(initial);
        self.cells.push(cell);
        self.cells.len() - 1
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // Return an Err (and you can change the error type) if any dependency doesn't exist.
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(&mut self,
                                                      dependencies: &[CellID],
                                                      compute_func: F)
                                                      -> Result<CellID, ()> {
        let dep_values = dependencies.iter()
            .flat_map(|&d| self.value(d))
            .collect::<Vec<_>>();
        if dep_values.len() != dependencies.len() {
            Err(())
        } else {
            let value = compute_func(dep_values.as_slice());

            self.cells.push(
                Cell {
                    value: value,
                    last_value: value,
                    deps: Vec::new(),
                    kind: CellType::Compute(Computation {
                        inputs: dependencies.into(),
                        compute_func: Box::new(compute_func)
                    }),
                    callbacks: HashMap::new()
                }
            );
            let id = self.cells.len() - 1;
            for &dep_id in dependencies {
                self.cells[dep_id].deps.push(id)
            }
            Ok(id)
        }
    }


    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        self.cells.get(id).map(|&ref c| c.value)
    }

    // Sets the value of the specified input cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist, or the
    // specified cell is a compute cell, since compute cells cannot have their values directly set.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        if self.cells.len() <= id {
            Err(()) //invalid cellid
        } else if let CellType::Compute(_) = self.cells[id].kind {
            Err(())
        } else {
            self.set_and_propagate_value(id, new_value);

            let dependent = self.cells[id].deps.clone();
            for dep_id in dependent {
                self.fire_callbacks_id_needed(dep_id);
            }
            Ok(()) //"Cannot set_value on compute cells.
        }
    }

    fn compute_value(&mut self, id: CellID) {
        let new_value = if let CellType::Compute(ref computation) = self.cells[id].kind {
            let dep_values = computation.inputs.iter()
                .flat_map(|&d| self.value(d))
                .collect::<Vec<_>>();
            let ref f = computation.compute_func;
            Some(f(&dep_values))
        } else {
            None
        };
        self.set_and_propagate_value(id, new_value.unwrap())
    }

    fn set_and_propagate_value(&mut self, id: CellID, new_value: T) {
        self.cells[id].value = new_value;
        let deps = self.cells[id].deps.clone();
        for dep_id in deps {
            self.compute_value(dep_id)
        }
    }

    fn fire_callbacks_id_needed(&mut self, id: CellID) {
        let dependent = self.cells.get_mut(id).map(|c| {
            if c.value == c.last_value {
                Vec::new()
            } else {
                for cb in c.callbacks.values_mut() {
                    cb(c.value);
                }
                c.last_value = c.value;
                c.deps.clone()
            }
        }).unwrap();

        for dep_id in dependent {
            self.fire_callbacks_id_needed(dep_id);
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(&mut self,
                                                id: CellID,
                                                callback: F)
                                                -> Result<CallbackID, ()> {
        match self.cells.get_mut(id) {
            Some(cell) => {
                self.callback_id_counter += 1;
                cell.callbacks.insert(self.callback_id_counter, Box::new(callback));
                Ok(self.callback_id_counter)
            }
            None => {
                Err(())
            }
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Return an Err (and you can change the error type) if either the cell or callback
    // does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        match self.cells.get_mut(cell) {
            Some(c) => c.callbacks.remove(&callback).map(|_| ()).ok_or(()),
            None => Err(()),
        }
    }
}
