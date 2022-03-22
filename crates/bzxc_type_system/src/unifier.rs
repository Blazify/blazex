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
    pub(crate) fn unify(&mut self, constraints: Vec<Constraint>) -> Substitution {
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

    pub fn unify_one(&mut self, constraint: Constraint) -> Substitution {
        match (constraint.0, constraint.1) {
            (Type::Fun(params, ret1), Type::Fun(args, ret2)) => {
                let mut constraints = vec![];

                for i in 0..params.len() {
                    constraints.push(Constraint(
                        params.get(i).unwrap().clone(),
                        args.get(i).unwrap().clone(),
                    ));
                }

                constraints.push(Constraint(*ret1.clone(), *ret2.clone()));

                self.unify(constraints)
            }
            (Type::Class(ty1), Type::Class(ty2)) => self.unify_one(Constraint(*ty1, *ty2)),
            (Type::Array(ty1, _), Type::Array(ty2, _)) => self.unify_one(Constraint(*ty1, *ty2)),
            (Type::Object(tree1), Type::Object(tree2)) => {
                let main_tree;
                let other_tree;

                if tree1.len() > tree2.len() {
                    main_tree = tree1;
                    other_tree = tree2;
                } else {
                    main_tree = tree2;
                    other_tree = tree1;
                };

                let a = main_tree.get("%alignment%");
                let b = other_tree.get("%alignment%");

                let mut constr = vec![];
                for (name, ty1) in &other_tree {
                    if name.as_str() == "%alignment%" {
                        continue;
                    }
                    let mut ty2 = main_tree.get(name);

                    if ty2.is_none() {
                        ty2 = self
                            .methods
                            .get(a.unwrap_or_else(|| b.unwrap()))
                            .unwrap_or_else(|| self.methods.get(b.unwrap()).unwrap())
                            .get(name);
                    }

                    constr.push(Constraint(ty1.clone(), ty2.unwrap().clone()));
                }

                self.unify(constr)
            }
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

    pub fn unify_var(&mut self, tvar: i32, ty: Type) -> Substitution {
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
                    let methods = self.methods.clone();
                    for (x, class) in methods {
                        for (y, method) in class {
                            if method == Type::Var(tvar) {
                                self.methods
                                    .get_mut(&x.clone())
                                    .unwrap()
                                    .insert(y.clone(), ty.clone());
                            }
                        }
                    }
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
                    .contains(&true)
                    | self.occurs(tvar, *r.clone())
            }
            Type::Array(arr, _) => self.occurs(tvar, *arr),
            Type::Class(klass) => self.occurs(tvar, *klass),
            Type::Object(obj) => obj
                .iter()
                .map(|(_, ty)| self.occurs(tvar, ty.clone()))
                .collect::<Vec<bool>>()
                .contains(&true),
            Type::Var(tvar2) => tvar == tvar2,
            _ => false,
        }
    }
}
