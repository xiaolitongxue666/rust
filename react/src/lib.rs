//! # React - 响应式单元
//!
//! Input 单元存值，Compute 单元依赖其他单元计算。set_value 时沿依赖图传播，仅当值变化时触发回调。
//!
//! ## 考点
//! - 闭包、Fn/FnMut 存储
//! - 图遍历、拓扑序更新
//! - Cell::last_value 判断是否需触发回调

use std::collections::HashMap;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct Cell<T: Copy> {
    value: T,
    last_value: T,
    dependents: Vec<ComputeCellId>,
}

struct ComputeCell<'a, T: Copy> {
    cell: Cell<T>,
    dependencies: Vec<CellId>,
    f: Box<dyn 'a + Fn(&[T]) -> T>,
    callbacks_issued: usize,
    callbacks: HashMap<CallbackId, Box<dyn 'a + FnMut(T)>>,
}

impl<T: Copy> Cell<T> {
    fn new(initial: T) -> Self {
        Cell {
            value: initial,
            last_value: initial,
            dependents: Vec::new(),
        }
    }
}

impl<'a, T: Copy> ComputeCell<'a, T> {
    fn new<F: 'a + Fn(&[T]) -> T>(initial: T, dependencies: Vec<CellId>, f: F) -> Self {
        ComputeCell {
            cell: Cell::new(initial),
            dependencies,
            f: Box::new(f),
            callbacks_issued: 0,
            callbacks: HashMap::new(),
        }
    }
}

pub struct Reactor<'a, T: Copy> {
    inputs: Vec<Cell<T>>,
    computes: Vec<ComputeCell<'a, T>>,
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            inputs: Vec::new(),
            computes: Vec::new(),
        }
    }

    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.inputs.push(Cell::new(initial));
        InputCellId(self.inputs.len() - 1)
    }

    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        for &dep in dependencies {
            match dep {
                CellId::Input(InputCellId(id)) => {
                    if id >= self.inputs.len() {
                        return Err(dep);
                    }
                }
                CellId::Compute(ComputeCellId(id)) => {
                    if id >= self.computes.len() {
                        return Err(dep);
                    }
                }
            }
        }
        let new_id = ComputeCellId(self.computes.len());
        for &dep in dependencies {
            match dep {
                CellId::Input(InputCellId(id)) => self.inputs[id].dependents.push(new_id),
                CellId::Compute(ComputeCellId(id)) => {
                    self.computes[id].cell.dependents.push(new_id)
                }
            }
        }
        let inputs: Vec<_> = dependencies
            .iter()
            .map(|&id| self.value(id).unwrap())
            .collect();
        let initial = compute_func(&inputs);
        self.computes.push(ComputeCell::new(
            initial,
            dependencies.to_vec(),
            compute_func,
        ));
        Ok(new_id)
    }

    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(InputCellId(id)) => self.inputs.get(id).map(|c| c.value),
            CellId::Compute(ComputeCellId(id)) => self.computes.get(id).map(|c| c.cell.value),
        }
    }

    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        let InputCellId(id) = id;
        let deps = match self.inputs.get_mut(id) {
            Some(c) => {
                c.value = new_value;
                c.dependents.clone()
            }
            None => return false,
        };
        for &d in &deps {
            self.update_dependent(d);
        }
        for d in deps {
            self.fire_callbacks(d);
        }
        true
    }

    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let ComputeCellId(id) = id;
        self.computes.get_mut(id).map(|c| {
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
        let ComputeCellId(cell) = cell;
        match self.computes.get_mut(cell) {
            Some(c) => match c.callbacks.remove(&callback) {
                Some(_) => Ok(()),
                None => Err(RemoveCallbackError::NonexistentCallback),
            },
            None => Err(RemoveCallbackError::NonexistentCell),
        }
    }

    fn update_dependent(&mut self, id: ComputeCellId) {
        let ComputeCellId(id) = id;
        let (new_value, dependents) = {
            let c = &self.computes[id];
            let inputs: Vec<_> = c
                .dependencies
                .iter()
                .map(|&dep| self.value(dep).unwrap())
                .collect();
            let new_val = (c.f)(&inputs);
            (new_val, c.cell.dependents.clone())
        };
        if let Some(c) = self.computes.get_mut(id) {
            if c.cell.value != new_value {
                c.cell.value = new_value;
            } else {
                return;
            }
        }
        for d in dependents {
            self.update_dependent(d);
        }
    }

    fn fire_callbacks(&mut self, id: ComputeCellId) {
        let ComputeCellId(id) = id;
        let dependents = match self.computes.get_mut(id) {
            Some(c) => {
                if c.cell.value == c.cell.last_value {
                    return;
                }
                for cb in c.callbacks.values_mut() {
                    cb(c.cell.value);
                }
                c.cell.last_value = c.cell.value;
                c.cell.dependents.clone()
            }
            None => return,
        };
        for d in dependents {
            self.fire_callbacks(d);
        }
    }
}

impl<T: Copy + PartialEq> Default for Reactor<'_, T> {
    fn default() -> Self {
        Self::new()
    }
}
