use super::{Constraint, Solver};

impl Solver {
    pub(crate) fn take_constraints(&mut self) -> Vec<Constraint> {
        std::mem::take(&mut self.context.volatile.constraints)
    }

    pub(crate) fn step(&mut self, constraints: Vec<Constraint>) -> anyhow::Result<Vec<Constraint>> {
        for constraint in constraints {
            match constraint {
                Constraint::ClassAssertion(_) => continue,
                Constraint::UnifyDeep(u_name, t_name) => {
                    let u_ty = self.unifications.get(&u_name);
                    let t_ty = self.unifications.get(&t_name);
                    match (u_ty, t_ty) {
                        (Some(u_ty), Some(t_ty)) => {
                            self.context.unify(*u_ty, *t_ty)?;
                        }
                        (None, Some(t_ty)) => {
                            self.unifications.insert(u_name, *t_ty);
                        }
                        (Some(u_ty), None) => {
                            self.unifications.insert(t_name, *u_ty);
                        }
                        (None, None) => {
                            self.unsolved_deep.push((u_name, t_name));
                        }
                    }
                }
                Constraint::UnifySolve(name, ty) => {
                    self.unifications.insert(name, ty);
                }
            }
        }

        let mut constraints = self.take_constraints();

        self.unsolved_deep.retain(|(u_name, t_name)| {
            let u_ty = self.unifications.get(u_name);
            let t_ty = self.unifications.get(t_name);
            if u_ty.is_some() || t_ty.is_some() {
                constraints.push(Constraint::UnifyDeep(*u_name, *t_name));
                false
            } else {
                true
            }
        });

        Ok(constraints)
    }

    pub fn solve(&mut self) -> anyhow::Result<()> {
        let mut constraints = self.take_constraints();

        loop {
            constraints = self.step(constraints)?;

            if constraints.is_empty() {
                break;
            }
        }

        Ok(())
    }
}
