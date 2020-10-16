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
        cos_buffer: [f32; SAMPLE_BUFFER_SIZE],
        samples_recorded: usize,
    }

    #[init(spawn = [measure])]
    fn init(ctx: init::Context) -> init::LateResources {
        let channels = rtt_target::rtt_init! {
            up: {
                0: {
                    size: 1024
                    name: "Log"
                }
                1: {
                    size: 1024
                    name: "Debug"
                }
                2: {
                    size: 1
                    name: "Data"
                }
            }
        };
        let mut info = channels.up.0;
        let data = channels.up.2;
        rtt_target::set_print_channel(channels.up.1);
        ctx.spawn.measure().unwrap();

        init::LateResources {
            sin_buffer: [0.0; SAMPLE_BUFFER_SIZE],
            cos_buffer: [0.0; SAMPLE_BUFFER_SIZE],
            samples_recorded: 0,
        }
    }

    #[idle(resources = [])]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            compiler_fence(Ordering::SeqCst);
        }
    }

    #[task(resources = [sin_buffer, cos_buffer, samples_recorded] )]
    fn measure(ctx: measure::Context) {
        static mut DAC_PHASE: u32 = 0;

        let measure::Context {
            resources:
                measure::Resources {
                    mut sin_buffer,
                    mut cos_buffer,
                    mut samples_recorded,
                },
            ..
        } = ctx;

        let sinf = 1.0f32;
        let cosf = 1.0f32;
        samples_recorded.lock(|samples_recorded| {
            sin_buffer.lock(|sin_buffer| sin_buffer[*samples_recorded] = sinf);
            cos_buffer.lock(|cos_buffer| cos_buffer[*samples_recorded] = cosf);
        });
    }

    #[task(
        resources = [
            samples_recorded,
            sin_buffer,
            cos_buffer,
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
