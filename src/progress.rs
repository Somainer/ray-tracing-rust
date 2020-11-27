use std::io::Write;

pub trait ProgressLike: Sized {
    fn length(&self) -> usize;
    fn percentage(&self) -> usize;

    fn print(&self) -> std::io::Result<()> {
        let mut out = std::io::stderr();
        let completed_length = self.percentage() * self.length() / 100;
        let rest_length = self.length() - completed_length;
        let string = format!("\r{}%[{}{}]\r",
               self.percentage(),
                "=".repeat(completed_length),
                " ".repeat(rest_length)
        );
        out.write_all(string.as_bytes())?;
        out.flush()
    }
}

pub struct Progress {
    length: usize,
    percentage: usize
}

impl ProgressLike for Progress {
    fn length(&self) -> usize {
        self.length
    }

    fn percentage(&self) -> usize {
        self.percentage
    }
}

impl Progress {
    pub fn new(length: usize) -> Progress {
        Self {
            length,
            percentage: 0
        }
    }

    fn start(&self) {
        self.print().unwrap()
    }

    pub fn update_progress(&mut self, percentage: usize) {
        self.percentage = percentage;
        self.print().unwrap();
    }

    pub fn done(&mut self) {
        self.update_progress(100);
        println!("Done");
    }
}

pub struct IterProgress<T: std::iter::ExactSizeIterator> {
    iter: T,
    consumed: usize,
    cached_length: usize
}

impl<T: ExactSizeIterator> Iterator for IterProgress<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        if let Some(_) = next {
            self.consumed += 1;
            self.print().ok()?;
        }
        next
    }
}

impl<T: ExactSizeIterator> std::iter::ExactSizeIterator for IterProgress<T> {
    fn len(&self) -> usize { self.iter.len() }
}

impl<T: ExactSizeIterator> ProgressLike for IterProgress<T> {
    fn length(&self) -> usize {
        self.cached_length.min(100)
    }

    fn percentage(&self) -> usize {
        (self.consumed as f64 / self.cached_length as f64 * 100.0) as usize
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
        }
    }
}
