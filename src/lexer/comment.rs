// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

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