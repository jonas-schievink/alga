// Copyright 2013-2014 The Num-rs Developers.
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

#![allow(missing_docs)]

use ops::{Op, Additive};

use structure::LoopApprox;
use structure::Loop;
use structure::MonoidApprox;
use structure::Monoid;

pub trait GroupApprox<O: Op>
    : LoopApprox<O>
    + MonoidApprox<O>
{}

impl GroupApprox<Additive> for i8   {}
impl GroupApprox<Additive> for i16  {}
impl GroupApprox<Additive> for i32  {}
impl GroupApprox<Additive> for i64  {}

pub trait Group<O: Op>
    : GroupApprox<O>
    + Loop<O>
    + Monoid<O>
{}

impl Group<Additive> for i8   {}
impl Group<Additive> for i16  {}
impl Group<Additive> for i32  {}
impl Group<Additive> for i64  {}
