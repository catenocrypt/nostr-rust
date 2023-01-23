// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Options {
    /// Wait for connection
    pub wait_for_connection: Arc<AtomicBool>,
    /// Wait for the msg to be sent
    pub wait_for_send: Arc<AtomicBool>,
    /// POW difficulty (for all events)
    pub difficulty: Arc<AtomicU8>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            wait_for_connection: Arc::new(AtomicBool::new(false)),
            wait_for_send: Arc::new(AtomicBool::new(false)),
            difficulty: Arc::new(AtomicU8::new(0)),
        }
    }
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn wait_for_connection(self, wait: bool) -> Self {
        Self {
            wait_for_connection: Arc::new(AtomicBool::new(wait)),
            ..self
        }
    }

    pub(crate) fn get_wait_for_connection(&self) -> bool {
        self.wait_for_connection.load(Ordering::SeqCst)
    }

    pub fn wait_for_send(self, wait: bool) -> Self {
        Self {
            wait_for_send: Arc::new(AtomicBool::new(wait)),
            ..self
        }
    }

    pub(crate) fn get_wait_for_send(&self) -> bool {
        self.wait_for_send.load(Ordering::SeqCst)
    }

    pub fn difficulty(self, difficulty: u8) -> Self {
        Self {
            difficulty: Arc::new(AtomicU8::new(difficulty)),
            ..self
        }
    }

    pub(crate) fn get_difficulty(&self) -> u8 {
        self.difficulty.load(Ordering::SeqCst)
    }

    pub fn update_opts(&self, new_opts: Options) {
        let _ = self
            .wait_for_connection
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| {
                Some(new_opts.get_wait_for_connection())
            });
        let _ = self
            .wait_for_send
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| {
                Some(new_opts.get_wait_for_send())
            });
        let _ = self
            .difficulty
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| {
                Some(new_opts.get_difficulty())
            });
    }
}
