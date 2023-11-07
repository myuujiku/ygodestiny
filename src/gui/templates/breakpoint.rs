use adw::{Breakpoint, BreakpointBin, BreakpointCondition, BreakpointConditionLengthType};
use relm4::{adw::traits::BreakpointBinExt, prelude::*};

pub fn default() -> Breakpoint {
    let condition = BreakpointCondition::new_length(
        BreakpointConditionLengthType::MaxWidth,
        560.0,
        adw::LengthUnit::Sp,
    );

    Breakpoint::new(condition)
}

pub fn default_bin() -> BreakpointBin {
    let bin = BreakpointBin::new();
    bin.add_breakpoint(default());

    bin
}
