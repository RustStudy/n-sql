// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub enum  CommentType {
    Block,
    Line
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde_derive", derive(Deserialize, Serialize))]
pub struct Comment {
    pub r#type: CommentType,
    pub content: String
}