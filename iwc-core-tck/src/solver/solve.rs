use std::collections::HashMap;

use iwc_core_ast::ty::{TypeIdx, pretty::pretty_print_ty};

use super::{Constraint, Solver};

type Unifications = HashMap<usize, TypeIdx>;

type Unsolved = Vec<(usize, usize)>;

impl Solver {
    fn take_constraints(&mut self) -> Vec<Constraint> {
        std::mem::take(&mut self.context.volatile.constraints)
    }

    pub fn solve(&mut self) -> anyhow::Result<()> {
        let mut constraints = self.take_constraints();
        let mut unifications = Unifications::new();
        let mut unsolved = Unsolved::new();

        loop {
            for constraint in constraints {
                match constraint {
                    Constraint::ClassAssertion(_) => continue,
                    Constraint::UnifyDeep(u_name, t_name) => {
                        let u_ty = unifications.get(&u_name);
                        let t_ty = unifications.get(&t_name);
                        match (u_ty, t_ty) {
                            (Some(u_ty), Some(t_ty)) => {
                                self.context.unify(*u_ty, *t_ty)?;
                            }
                            (None, Some(t_ty)) => {
                                unifications.insert(u_name, *t_ty);
                            }
                            (Some(u_ty), None) => {
                                unifications.insert(t_name, *u_ty);
                            }
                            (None, None) => {
                                unsolved.push((u_name, t_name));
                            }
                        }
                    }
                    Constraint::UnifySolve(name, ty) => {
                        unifications.insert(name, ty);
                    }
                }
            }

            constraints = self.take_constraints();

            unsolved.retain(|(u_name, t_name)| {
                let u_ty = unifications.get(u_name);
                let t_ty = unifications.get(t_name);
                if u_ty.is_some() || t_ty.is_some() {
                    constraints.push(Constraint::UnifyDeep(*u_name, *t_name));
                    false
                } else {
                    true
                }
            });

            if constraints.is_empty() {
                break;
            }
        }

        for (name, ty) in unifications {
            println!("?{} ~ {}", name, pretty_print_ty(&self.context.volatile.type_arena, ty));
        }

        Ok(())
    }
}
