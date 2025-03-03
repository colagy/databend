// Copyright 2021 Datafuse Labs.
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

#![feature(allocator_api)]
#![feature(thread_local)]
#![feature(ptr_metadata)]
#![feature(result_flattening)]
#![feature(try_trait_v2)]
#![feature(thread_id_value)]
#![feature(backtrace_frames)]
#![allow(incomplete_features)]

pub mod base;
pub mod containers;
pub mod mem_allocator;
pub mod rangemap;
