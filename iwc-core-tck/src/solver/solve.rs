use iwc_core_constraints::Constraint;

impl super::Solver {
    pub(crate) fn take_constraints(&mut self) -> Vec<Constraint> {
        std::mem::take(&mut self.context.volatile.constraints)
    }

    pub(crate) fn step(&mut self, constraints: Vec<Constraint>) -> Vec<Constraint> {
        for constraint in constraints {
            match constraint {
                Constraint::ClassAssertion(_) => continue,
                Constraint::UnifyDeep(u_name, t_name) => {
                    let u_ty = self.unification_solved.get(&u_name);
                    let t_ty = self.unification_solved.get(&t_name);
                    match (u_ty, t_ty) {
                        (Some(u_ty), Some(t_ty)) => {
                            self.context.unify(*u_ty, *t_ty);
                        }
                        (None, Some(t_ty)) => {
                            self.unification_solved.insert(u_name, *t_ty);
                        }
                        (Some(u_ty), None) => {
                            self.unification_solved.insert(t_name, *u_ty);
                        }
                        (None, None) => {
                            self.unification_unsolved.push((u_name, t_name));
                        }
                    }
                }
                Constraint::UnifySolve(name, ty) => {
                    self.unification_solved.insert(name, ty);
                }
                Constraint::UnifyError(error) => {
                    self.unification_errors.push(error);
                }
            }
        }

        let mut constraints = self.take_constraints();

        self.unification_unsolved.retain(|(u_name, t_name)| {
            let u_ty = self.unification_solved.get(u_name);
            let t_ty = self.unification_solved.get(t_name);
            if u_ty.is_some() || t_ty.is_some() {
                constraints.push(Constraint::UnifyDeep(*u_name, *t_name));
                false
            } else {
                true
            }
        });

        constraints
    }

    pub fn solve(&mut self) {
        let mut constraints = self.take_constraints();

        loop {
            constraints = self.step(constraints);

            if constraints.is_empty() {
                break;
            }
        }
    }
}
