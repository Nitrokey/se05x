// Copyright (C) 2023 Nitrokey GmbH
// SPDX-License-Identifier: LGPL-3.0-only

#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

extern crate delog;
delog::generate_macros!();

mod macros;

pub mod se050;
pub mod t1;
