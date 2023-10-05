// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the sequencer-example-l2 repository.

// You should have received a copy of the MIT License
// along with the sequencer-example-l2 repository. If not, see <https://mit-license.org/>.

use clap::ValueEnum;
use strum_macros::EnumIter;

pub const INITIAL_BALANCE: u64 = 9999;

#[derive(ValueEnum, Clone, Copy, Debug, EnumIter)]
#[value(rename_all = "verbatim")]
pub enum SeedIdentity {
    Bob = 0,
    Alice = 1,
    Charlie = 2,
}
