use std::collections::VecDeque;

use num::complex::Complex32;

use crate::{
    mandelbrot::{Mandelbrot, rect_from_position},
    range::Range,
};

pub struct BoundaryScanner<'a> {
    pub(crate) mandelbrot: &'a Mandelbrot,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) width_range: Range,
    pub(crate) height_range: Range,
    pub(crate) real_range: Range,
    pub(crate) imaginary_range: Range,
    pub(crate) data: Vec<usize>,
    pub(crate) queued: Vec<bool>,
    pub(crate) loaded: Vec<bool>,
    pub(crate) queue: VecDeque<usize>,
}

impl<'a> BoundaryScanner<'a> {
    pub fn new(mandelbrot: &'a Mandelbrot, start: usize, end: usize) -> Self {
        let width = mandelbrot.width;
        let height = end - start;
        let size = width * height;
        let queue_size = (width + height) * 2;

        let rect = rect_from_position(&mandelbrot.position, &mandelbrot.zoom);

        Self {
            mandelbrot,
            start,
            end,
            width_range: Range::new(0.0, mandelbrot.width as f32),
            height_range: Range::new(0.0, mandelbrot.height as f32),
            real_range: Range::new(rect.start.x, rect.end.x),
            imaginary_range: Range::new(rect.start.y, rect.end.y),
            data: vec![0; size],
            queued: vec![false; size],
            loaded: vec![false; size],
            queue: VecDeque::with_capacity(queue_size),
        }
    }

    #[inline]
    fn local_index(&self, index: usize) -> usize {
        index - self.start * self.mandelbrot.width
    }

    fn add_queue(&mut self, index: usize) {
        let local_index = self.local_index(index);
        if self.queued[local_index] {
            return;
        }
        self.queued[local_index] = true;
        self.queue.push_back(index);
    }

    fn load(&mut self, index: usize) -> usize {
        let local_index = self.local_index(index);
        if self.loaded[local_index] {
            return self.data[local_index];
        }

        let x = (index % self.mandelbrot.width) as f32;
        let y = (index / self.mandelbrot.width) as f32;

        let c = Complex32::new(
            Range::scale(&self.width_range, x, &self.real_range),
            Range::scale(&self.height_range, y, &self.imaginary_range),
        );

        let (_, result) = self.mandelbrot.iterate(&c);
        self.loaded[local_index] = true;
        self.data[local_index] = result;
        result
    }

    fn scan(&mut self, index: usize) {
        let width = self.mandelbrot.width;
        let x = index % width;
        let y = index / width;
        let center = self.load(index);
        let ll = x > 0;
        let rr = x + 1 < width;
        let uu = y > self.start;
        let dd = y + 1 < self.end;
        let l = ll && self.load(index - 1) != center;
        let r = rr && self.load(index + 1) != center;
        let u = uu && self.load(index - width) != center;
        let d = dd && self.load(index + width) != center;
        if l {
            self.add_queue(index - 1);
        }
        if r {
            self.add_queue(index + 1);
        }
        if u {
            self.add_queue(index - width);
        }
        if d {
            self.add_queue(index + width);
        }
        if (uu && ll) && (l || u) {
            self.add_queue(index - width - 1);
        }
        if (uu && rr) && (r || u) {
            self.add_queue(index - width + 1);
        }
        if (dd && ll) && (l || d) {
            self.add_queue(index + width - 1);
        }
        if (dd && rr) && (r || d) {
            self.add_queue(index + width + 1);
        }
    }

    pub fn run(&mut self) -> &[usize] {
        let width = self.mandelbrot.width;

        for y in self.start..self.end {
            self.add_queue(y * width);
            self.add_queue(y * width + (width - 1));
        }
        for x in 1..width - 1 {
            self.add_queue(self.start * width + x);
            self.add_queue((self.end - 1) * width + x);
        }

        while let Some(index) = self.queue.pop_front() {
            self.scan(index);
        }

        for index in self.start * width..self.end * width - 1 {
            let local_index = self.local_index(index);
            if self.loaded[local_index] && !self.loaded[local_index + 1] {
                self.data[local_index + 1] = self.data[local_index];
                self.loaded[local_index + 1] = true;
            }
        }

        &self.data
    }
}
