use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct InputCell<T> {
    value: T,
    dependents: Vec<ComputeCellId>,
    prev_val: T,
}

impl<T: Copy> InputCell<T> {
    pub fn new(value: T) -> InputCell<T> {
        Self {
            value,
            dependents: Vec::new(),
            prev_val: value,
        }
    }
}

pub struct ComputeCell<'a, T> {
    cell: InputCell<T>,
    compute_function: Box<dyn 'a + Fn(&[T]) -> T>,
    dependencies: Vec<CellId>,
    callbacks_issued: usize,
    callbacks: HashMap<CallbackId, Box<dyn 'a + FnMut(T)>>,
}

impl<'a, T: Copy> ComputeCell<'a, T> {
    pub fn new<F: 'a + Fn(&[T]) -> T>(initial: T, dependencies: Vec<CellId>, comp_func: F) -> Self {
        Self {
            cell: InputCell::new(initial),
            compute_function: Box::new(comp_func),
            dependencies,
            callbacks_issued: 0,
            callbacks: HashMap::new(),
        }
    }
}

pub struct Reactor<'a, T: Copy> {
    input_cells: Vec<InputCell<T>>,
    compute_cells: Vec<ComputeCell<'a, T>>,
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input_cells: Vec::new(),
            compute_cells: Vec::new(),
        }
    }

    pub fn create_input(&mut self, _initial: T) -> InputCellId {
        self.input_cells.push(InputCell::new(_initial));
        InputCellId(self.input_cells.len() - 1)
    }

    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let cell_id = ComputeCellId(self.compute_cells.len());

        for &d in dependencies {
            match d {
                CellId::Input(InputCellId(x)) => {
                    if x >= self.input_cells.len() {
                        return Err(d);
                    }
                }

                CellId::Compute(ComputeCellId(x)) => {
                    if x >= cell_id.0 {
                        return Err(d);
                    }
                }
            }
        }

        for &d in dependencies {
            match d {
                CellId::Input(InputCellId(x)) => self.input_cells[x].dependents.push(cell_id),

                CellId::Compute(ComputeCellId(x)) => {
                    self.compute_cells[x].cell.dependents.push(cell_id)
                }
            }
        }

        let initial = compute_func(
            &dependencies
                .iter()
                .map(|&id| self.value(id).unwrap())
                .collect::<Vec<_>>(),
        );

        self.compute_cells.push(ComputeCell::new(
            initial,
            dependencies.to_vec(),
            compute_func,
        ));

        Ok(cell_id)
    }

    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(InputCellId(x)) => self.input_cells.get(x).map(|c| c.value),

            CellId::Compute(ComputeCellId(x)) => self.compute_cells.get(x).map(|c| c.cell.value),
        }
    }

    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        self.input_cells
            .get_mut(id.0)
            .map(|cell| {
                cell.value = new_value;

                cell.dependents.clone()
            })
            .map(|dependents| {
                self.update_dependents(dependents.clone());

                self.update_callbacks(dependents.clone());
            })
            .is_some()
    }

    fn update_dependents(&mut self, dependents: Vec<ComputeCellId>) {
        for &d in dependents.iter() {
            let inputs: Vec<_> = self.compute_cells[d.0]
                .dependencies
                .iter()
                .map(|&id| self.value(id).unwrap())
                .collect();

            let computed_value = (self.compute_cells[d.0].compute_function)(&inputs);

            self.compute_cells[d.0].cell.value = computed_value;

            self.update_dependents(self.compute_cells[d.0].cell.dependents.clone());
        }
    }

    fn update_callbacks(&mut self, dependents: Vec<ComputeCellId>) {
        for d in dependents {
            if let Some(c) = self.compute_cells.get_mut(d.0) {
                if c.cell.value == c.cell.prev_val {
                    return;
                }

                for callback in c.callbacks.values_mut() {
                    callback(c.cell.value);
                }

                c.cell.prev_val = c.cell.value;
            }

            self.update_callbacks(self.compute_cells[d.0].cell.dependents.clone())
        }
    }

    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        self.compute_cells.get_mut(id.0).map(|c| {
            c.callbacks_issued += 1;
            let cbid = CallbackId(c.callbacks_issued);
            c.callbacks.insert(cbid, Box::new(callback));

            cbid
        })
    }

    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        match self.compute_cells.get_mut(cell.0) {
            Some(c) => match c.callbacks.remove(&callback) {
                Some(_) => Ok(()),

                None => Err(RemoveCallbackError::NonexistentCallback),
            },

            None => Err(RemoveCallbackError::NonexistentCell),
        }
    }
}
