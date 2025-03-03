// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashSet;

use super::column_stat::ColumnStatSet;
use crate::plans::Scalar;
use crate::IndexType;

pub type ColumnSet = HashSet<IndexType>;
pub type TableSet = HashSet<IndexType>;

#[derive(Default, Clone)]
pub struct RequiredProperty {
    pub distribution: Distribution,
}

impl RequiredProperty {
    pub fn satisfied_by(&self, physical: &PhysicalProperty) -> bool {
        self.distribution.satisfied_by(&physical.distribution)
    }
}

#[derive(Default, Clone, Debug)]
pub struct Statistics {
    // We can get the precise row count of a table in databend,
    // which information is useful to optimize some queries like `COUNT(*)`.
    pub precise_cardinality: Option<u64>,
    /// Statistics of columns, column index -> column stat
    pub column_stats: ColumnStatSet,
    /// Statistics info is accurate
    pub is_accurate: bool,
}

#[derive(Default, Clone, Debug)]
pub struct RelationalProperty {
    /// Output columns of a relational expression
    pub output_columns: ColumnSet,

    /// Outer references of a relational expression
    pub outer_columns: ColumnSet,

    /// Used columns of a relational expression
    pub used_columns: ColumnSet,

    // TODO(leiysky): introduce upper bound of cardinality to
    // reduce error in estimation.
    pub cardinality: f64,
    pub statistics: Statistics,
}

#[derive(Default, Clone)]
pub struct PhysicalProperty {
    pub distribution: Distribution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Distribution {
    Any,
    Random,
    Serial,
    Broadcast,
    Hash(Vec<Scalar>),
}

impl Default for Distribution {
    // Only used for `RequiredProperty`
    fn default() -> Self {
        Self::Any
    }
}

impl Distribution {
    /// Check if required distribution is satisfied by given distribution.
    pub fn satisfied_by(&self, distribution: &Distribution) -> bool {
        // (required, delivered)
        match (&self, distribution) {
            (Distribution::Any, _)
            | (Distribution::Random, _)
            | (Distribution::Serial, Distribution::Serial)
            | (Distribution::Broadcast, Distribution::Broadcast) => true,

            // TODO(leiysky): this is actually broken by https://github.com/datafuselabs/databend/pull/7451
            // , would be fixed later.
            // (Distribution::Hash(ref keys), Distribution::Hash(ref other_keys)) => keys
            //     .iter()
            //     .all(|key| other_keys.iter().any(|other_key| key == other_key)),
            _ => false,
        }
    }
}
