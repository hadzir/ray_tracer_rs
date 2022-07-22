use crate::F;

pub struct VAnimator {
    frame_count: usize,
}
pub struct Frame {
    current: usize,
    count: usize,
}

pub struct LinearScale {
    domain: (F, F),
    range: Vec<F>,
}
impl LinearScale {
    pub fn new() -> Self {
        Self {
            domain: (0.0, 1.0),
            range: vec![0.0, 1.0],
        }
    }
    pub fn with_range(mut self, range: Vec<F>) -> Self {
        self.range = range;
        self
    }
    pub fn with_domain(mut self, start: F, end: F) -> Self {
        self.domain = (start, end);
        self
    }
    pub fn scale(&self, input: F) -> F {
        self.interpolate(self.normalize(input))
    }
    fn normalize(&self, input: F) -> F {
        //Takes an arbitrary input float and converts it into a value between [0,1)
        let cimput = input.clamp(self.domain.0, self.domain.1);
        let progress = cimput - self.domain.0;
        let distance = self.domain.1 - self.domain.0;
        progress / distance
    }
    fn interpolate(&self, normalized_input: F) -> F {
        let slice_count = self.range.len() - 1;
        let slice_width = 1.0 / slice_count as F;
        //hack to avoid outofbounds index for I=1.0
        let mut offset_key = 0;
        if normalized_input == 1.0 {
            offset_key = 1;
        }

        let current_slice = (normalized_input * (slice_count as F).floor()) as usize - offset_key;
        let current_slice_progess =
            (normalized_input - slice_width * current_slice as F) / slice_width;

        //0-------------------1
        //.----|----|----|----.     ->Slice count = 4, slice width = / 1/4
        //.------->I.               ->Normalized input = 0.4
        //.                         Current slice idx = 1
        //.----|-->                 current slice progress CSP = I-SW*CSI
        //.currenct slice progress, normalized = CSP / SW

        current_slice_progess * (self.range[current_slice + 1] - self.range[current_slice])
            + self.range[current_slice]
    }
}

impl Frame {
    pub fn new(count: usize, current: usize) -> Self {
        Self { current, count }
    }
    pub fn currentf(&self) -> F {
        self.current as F
    }
    pub fn current(&self) -> usize {
        self.current as usize
    }
    pub fn filename(&self, path: &str, name: &str, suffix: &str) -> String {
        format!("{}/{}{:06}{}", path, name, self.current, suffix)
    }
    pub fn linear_scale(&self) -> LinearScale {
        LinearScale::new().with_domain(0.0, self.count as F)
    }
}
impl VAnimator {
    pub fn new(frame_count: usize) -> Self {
        Self { frame_count }
    }
    pub fn animate(&self, animate: fn(frame: Frame)) {
        for current_frame in 0..self.frame_count {
            animate(Frame::new(self.frame_count, current_frame));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::*;

    #[test]
    fn scaling_works1() {
        let ls = LinearScale::new().with_range(vec![100.0, 200.0]);
        assert_zeq!(ls.scale(0.0), 100.0);
        assert_zeq!(ls.scale(0.5), 150.0);
        assert_zeq!(ls.scale(1.0), 200.0);
    }
    #[test]
    fn scaling_works2() {
        let ls = LinearScale::new()
            .with_domain(0.0, 10.0)
            .with_range(vec![100.0, 200.0]);
        assert_zeq!(ls.scale(0.0), 100.0);
        assert_zeq!(ls.scale(5.0), 150.0);
        assert_zeq!(ls.scale(10.0), 200.0);
    }
    #[test]
    fn scaling_works3() {
        let ls = LinearScale::new().with_range(vec![0.0, 100.0, 200.0, 100.0, 0.0]);
        assert_zeq!(ls.scale(0.0), 0.0);
        assert_zeq!(ls.scale(0.05), 20.0);
        assert_zeq!(ls.scale(0.10), 40.0);
        assert_zeq!(ls.scale(0.15), 60.0);
        assert_zeq!(ls.scale(0.20), 80.0);
        assert_zeq!(ls.scale(0.25), 100.0);
        assert_zeq!(ls.scale(0.50), 200.0);
        assert_zeq!(ls.scale(0.75), 100.0);
        assert_zeq!(ls.scale(100.0), 0.0);
    }
    #[test]
    fn scaling_works_for_frame1() {
        let f = Frame::new(100, 0);
        let ls = f.linear_scale().with_range(vec![100.0, 200.0]);
        assert_zeq!(ls.scale(f.currentf()), 100.0);
    }
    #[test]
    fn scaling_works_for_frame2() {
        let f = Frame::new(100, 50);
        let ls = f.linear_scale().with_range(vec![100.0, 200.0]);
        assert_zeq!(ls.scale(f.currentf()), 150.0);
    }
    #[test]
    fn scaling_works_for_frame3() {
        let f = Frame::new(100, 100);
        let ls = f.linear_scale().with_range(vec![100.0, 200.0]);
        assert_zeq!(ls.scale(f.currentf()), 200.0);
    }
}
