#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(const_fn)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate panic_abort;
extern crate stm32f103xx;
extern crate typenum;

use rtfm::{app, Threshold};
use typenum::consts::U1;

app! { //~ error bound `*const (): core::marker::Send` is not satisfied
    device: stm32f103xx,

    resources: {
        static TOKEN: Option<Threshold<U1>> = None;
    },

    idle: {
        resources: [TOKEN],
    },

    tasks: {
        exti0: {
            interrupt: EXTI0,
            resources: [TOKEN],
        },
    }
}

fn init(_ctxt: init::Context) {}

fn idle(_ctxt: idle::Context) -> ! {
    loop {}
}

fn exti0(_ctxt: exti0::Context) {}
