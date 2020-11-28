use std::io::Write;
use std::time::{Instant, Duration};
use std::fmt::Debug;
use std::cmp::min;

pub trait ProgressLike: Sized {
    fn length(&self) -> usize;
    fn percentage(&self) -> usize;
    fn start_time(&self) -> Instant;

    fn print(&self) -> std::io::Result<()> {
        let mut out = std::io::stderr();
        let completed_length = self.percentage() * self.length() / 100;
        let rest_length = self.length() - completed_length;
        let string = format!("\rElapsed: {} ({}%)[{}{}]\r",
                Self::format_duration(self.start_time().elapsed()),
                self.percentage(),
                "=".repeat(completed_length),
                " ".repeat(rest_length)
        );
        out.write_all(string.as_bytes())?;
        out.flush()
    }

    fn format_duration(duration: Duration) -> String {
        if duration.as_secs() < 1 {
            format!("{}ms", duration.as_micros())
        } else {
            let total_seconds = duration.as_secs();
            let mut result = String::new();
            let seconds = total_seconds % 60;
            let hours = total_seconds / 3600;
            let minutes = total_seconds % 3600 / 60;

            if hours > 0 {
                result.push_str(format!("{}h", hours).as_str());

                if minutes > 0 || seconds > 0 {
                    result.push_str(",");
                }
            }

            if minutes > 0 {
                result.push_str(format!("{}min", minutes).as_str());

                if seconds > 0 {
                    result.push_str(",");
                }
            }

            if seconds > 0 {
                result.push_str(format!("{}s", seconds).as_str());
            }

            result
        }
    }
}

pub struct Progress {
    length: usize,
    percentage: usize,
    start_time: Instant
}

impl ProgressLike for Progress {
    fn length(&self) -> usize {
        self.length
    }

    fn percentage(&self) -> usize {
        self.percentage
    }

    fn start_time(&self) -> Instant {
        self.start_time
    }
}

impl Progress {
    pub fn new(length: usize) -> Progress {
        Self {
            length,
            percentage: 0,
            start_time: Instant::now()
        }
    }

    fn start(&self) {
        self.print().unwrap()
    }

    pub fn update_progress(&mut self, percentage: usize) {
        self.percentage = percentage;
        self.print().unwrap();
    }

    pub fn with_progress(&self, percentage: usize) -> Self {
        Self {
            length: self.length,
            percentage,
            start_time: self.start_time
        }
    }

    pub fn done(&mut self) {
        self.update_progress(100);
        println!("Done");
    }
}

pub struct IterProgress<T: std::iter::ExactSizeIterator> {
    iter: T,
    consumed: usize,
    cached_length: usize,
    start_time: Instant
}

impl<T: ExactSizeIterator> Iterator for IterProgress<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        if let Some(_) = next {
            self.consumed += 1;
            self.print().ok()?;
        } else {
            println!("\nFinished in {}.", Self::format_duration(self.start_time.elapsed()));
        }
        next
    }
}

impl<T: ExactSizeIterator> std::iter::ExactSizeIterator for IterProgress<T> {
    fn len(&self) -> usize { self.iter.len() }
}

impl<T: ExactSizeIterator> ProgressLike for IterProgress<T> {
    fn length(&self) -> usize {
        self.cached_length.min(80)
    }

    fn percentage(&self) -> usize {
        (self.consumed as f64 / self.cached_length as f64 * 100.0) as usize
    }

    fn start_time(&self) -> Instant {
        self.start_time
    }
}

pub trait ProgressIterable<T: ExactSizeIterator> {
    fn iter_progressed(self) -> IterProgress<T>;
}

impl<T: ExactSizeIterator> ProgressIterable<T> for T {
    fn iter_progressed(self) -> IterProgress<T> {
        IterProgress {
            cached_length: self.len(),
            iter: self,
            consumed: 0,
            start_time: Instant::now()
        }
    }
}
