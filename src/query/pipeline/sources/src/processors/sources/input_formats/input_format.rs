//  Copyright 2022 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use std::sync::Arc;

use common_exception::Result;
use common_meta_types::UserStageInfo;
use common_pipeline_core::Pipeline;
use common_settings::Settings;
use opendal::Operator;

use crate::processors::sources::input_formats::input_context::InputContext;
use crate::processors::sources::input_formats::input_split::SplitInfo;

#[async_trait::async_trait]
pub trait InputFormat: Send + Sync {
    async fn get_splits(
        &self,
        files: &[String],
        stage_info: &UserStageInfo,
        op: &Operator,
        settings: &Arc<Settings>,
    ) -> Result<Vec<Arc<SplitInfo>>>;

    fn exec_copy(&self, ctx: Arc<InputContext>, pipeline: &mut Pipeline) -> Result<()>;

    fn exec_stream(&self, ctx: Arc<InputContext>, pipeline: &mut Pipeline) -> Result<()>;
}
