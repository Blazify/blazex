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

use crate::constraint::Constraint;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Substitution(pub BTreeMap<Type, Type>);
impl Substitution {
    pub fn empty() -> Self {
        Substitution(BTreeMap::new())
    }

    pub fn apply(&self, constraints: Vec<Constraint>) -> Vec<Constraint> {
        constraints
            .iter()
            .map(|constraint| {
                Constraint(
                    self.apply_ty(constraint.0.clone()),
                    self.apply_ty(constraint.1.clone()),
                )
            })
            .collect()
    }

    pub fn apply_ty(&self, ty: Type) -> Type {
        self.0.iter().fold(ty.clone(), |result, solution| {
            let (ty, solution_type) = solution;
            if let Type::Var(tvar) = ty {
                self.substitute_tvar(result, *tvar, solution_type.clone())
            } else {
                unreachable!();
            }
        })
    }

    fn substitute_tvar(&self, ty: Type, tvar: i32, sol_ty: Type) -> Type {
        match ty {
            Type::Fun(params, ret) => Type::Fun(
                params
                    .iter()
                    .map(|x| self.substitute_tvar(x.clone(), tvar, sol_ty.clone()))
                    .collect(),
                box self.substitute_tvar(*ret.clone(), tvar, sol_ty.clone()),
            ),
            Type::Array(ty, size) => Type::Array(box self.substitute_tvar(*ty, tvar, sol_ty), size),
            Type::Var(tvar2) => {
                if tvar == tvar2 {
                    sol_ty
                } else {
                    ty
                }
            }
            _ => ty,
        }
    }

    pub fn compose(&mut self, other: Substitution) -> Substitution {
        let mut self_substituded: BTreeMap<Type, Type> = self
            .0
            .clone()
            .into_iter()
            .map(|(k, s)| (k, other.apply_ty(s)))
            .collect();
        self_substituded.extend(other.0);
        Substitution(self_substituded)
    }
}
