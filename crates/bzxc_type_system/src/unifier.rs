/*
 * Copyright 2020 to 2021 BlazifyOrg
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *    http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/
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
            (Type::Array(box Type::Var(tvar), _), Type::ElementType(ty)) => {
                self.unify_var(tvar, *ty)
            }
            (Type::ElementType(ty), Type::Array(box Type::Var(tvar), _)) => {
                self.unify_var(tvar, *ty)
            }
            (Type::ElementType(ty), Type::Var(tvar)) => self.unify_var(tvar, Type::ElementType(ty)),
            (Type::Var(tvar), Type::ElementType(ty)) => self.unify_var(tvar, Type::ElementType(ty)),
            (Type::Var(tvar), ty) => self.unify_var(tvar, ty),
            (ty, Type::Var(tvar)) => self.unify_var(tvar, ty),
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
