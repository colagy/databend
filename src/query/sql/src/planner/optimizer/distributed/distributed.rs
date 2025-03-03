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

use std::sync::Arc;

use common_catalog::table_context::TableContext;
use common_exception::Result;

use crate::optimizer::property::require_property;
use crate::optimizer::Distribution;
use crate::optimizer::RelExpr;
use crate::optimizer::RequiredProperty;
use crate::optimizer::SExpr;
use crate::plans::Exchange;

pub fn optimize_distributed_query(ctx: Arc<dyn TableContext>, s_expr: &SExpr) -> Result<SExpr> {
    let required = RequiredProperty {
        distribution: Distribution::Any,
    };
    let mut result = require_property(ctx, &required, s_expr)?;
    let rel_expr = RelExpr::with_s_expr(&result);
    let physical_prop = rel_expr.derive_physical_prop()?;
    let root_required = RequiredProperty {
        distribution: Distribution::Serial,
    };
    if !root_required.satisfied_by(&physical_prop) {
        // Manually enforce serial distribution.
        result = SExpr::create_unary(Exchange::Merge.into(), result);
    }

    Ok(result)
}
