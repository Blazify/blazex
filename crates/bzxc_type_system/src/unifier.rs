use std::collections::BTreeMap;

use bzxc_shared::Type;

use crate::{constraint::Constraint, substitution::Substitution, TypeSystem};

impl TypeSystem {
    pub fn unify(&self, constraints: Vec<Constraint>) -> Substitution {
        if constraints.is_empty() {
            Substitution::empty()
        } else {
            let mut it = constraints.into_iter();
            let mut subst = self.unify_one(it.next().unwrap());
            let subst_tail = subst.apply(it.collect());
            let subst_tail: Substitution = self.unify(subst_tail);
            subst.compose(subst_tail)
        }
    }

    pub fn unify_one(&self, constraint: Constraint) -> Substitution {
        match (constraint.0, constraint.1) {
            (Type::Fun(params, ret1), Type::Fun(args, ret2)) => {
                let mut constraints = vec![];

                for i in 0..params.len() {
                    constraints.push(Constraint(
                        params.get(i - 1).unwrap().clone(),
                        args.get(i - 1).unwrap().clone(),
                    ));
                }

                constraints.push(Constraint(*ret1.clone(), *ret2.clone()));

                self.unify(constraints)
            }
            (Type::Var(tvar), ty) => self.unify_var(tvar, ty),
            (ty, Type::Var(tvar)) => self.unify_var(tvar, ty),
            (Type::Array(box Type::Var(tvar)), Type::Array(ty)) => self.unify_var(tvar, *ty),
            (Type::Array(ty), Type::Array(box Type::Var(tvar))) => self.unify_var(tvar, *ty),
            (a, b) => {
                if a == b {
                    Substitution::empty()
                } else {
                    panic!("Cannot unify {:#?} with {:#?}", a, b)
                }
            }
        }
    }

    pub fn unify_var(&self, tvar: i32, ty: Type) -> Substitution {
        match ty.clone() {
            Type::Var(tvar2) => {
                if tvar == tvar2 {
                    Substitution::empty()
                } else {
                    Substitution(BTreeMap::from([(Type::Var(tvar), ty)]))
                }
            }
            _ => {
                if self.occurs(tvar, ty.clone()) {
                    panic!("circular type")
                } else {
                    Substitution(BTreeMap::from([(Type::Var(tvar), ty)]))
                }
            }
        }
    }

    fn occurs(&self, tvar: i32, ty: Type) -> bool {
        match ty {
            Type::Fun(p, r) => {
                p.iter()
                    .map(|x| self.occurs(tvar, x.clone()))
                    .collect::<Vec<bool>>()
                    .contains(&false)
                    | self.occurs(tvar, *r.clone())
            }
            Type::Var(tvar2) => tvar == tvar2,
            _ => false,
        }
    }
}
