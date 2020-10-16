#![no_std]
#![no_main]

mod types {
    pub type R = f32;
}

const SAMPLE_BUFFER_SIZE: usize = 16;

use core::sync::atomic::{compiler_fence, Ordering};

use types::R;

#[rtic::app(device = stm32l4xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        sin_buffer: [R; SAMPLE_BUFFER_SIZE],
    }

    #[init(spawn = [measure])]
    fn init(ctx: init::Context) -> init::LateResources {
        init::LateResources {
            sin_buffer: [0.0; SAMPLE_BUFFER_SIZE],
        }
    }

    #[idle(resources = [])]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            compiler_fence(Ordering::SeqCst);
        }
    }

    #[task(resources = [sin_buffer] )]
    fn measure(ctx: measure::Context) {
        static mut DAC_PHASE: u32 = 0;

        let measure::Context {
            resources: measure::Resources { mut sin_buffer },
            ..
        } = ctx;

        let sinf = 1.0f32;
        sin_buffer.lock(|sin_buffer| sin_buffer[0] = sinf);
    }

    #[task(
        resources = [
            sin_buffer,
        ],
        binds = EXTI0, priority = 2
    )]
    fn conversion_done(ctx: conversion_done::Context) {
        // Do some stuff with buffers
    }

    extern "C" {
        fn EXTI4();
    }
};
