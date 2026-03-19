use core::error;

struct BumpAllocator {
    start: usize,
    end: usize,
    current: usize,
}

impl BumpAllocator {
    fn new() -> Self {
        return Self{
            start:0,
            end:0,
            current:0
        };
    }
    fn init(&mut self,start: usize ,end: usize) {
       self.start = start;
       self.end = end;
       self.current = start;
    }
    fn allocate(&mut self, size: usize, alignment: usize) -> Result<usize, &str> {
        let aligned = if self.current % alignment != 0 {
            ((((self.current as f64) / (alignment as f64)).floor() + 1.0) * alignment as f64)
                as usize
        } else {
            self.current
        };

        println!("{}", aligned);
        if aligned + size > self.end {
            return Err("Heap Space not available");
        }
        self.current = aligned + size;

        return Ok(aligned);
    }
    fn reset(&mut self) {
        self.current = self.start;
    }
}

fn main() {
    let mut allocator = BumpAllocator::new();
    allocator.init(1000, 2000);
}
