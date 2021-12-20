struct ImageProcessor {
    image: Vec<Vec<bool>>,
    algorithm: Vec<bool>,
    void_val: bool,
}

impl ImageProcessor {
    fn new(algorithm_input: &str, image_input: &str) -> Self {
        let algorithm = Self::parse_row(algorithm_input);
        let image = Self::parse_image(image_input);
        ImageProcessor {
            algorithm,
            image,
            void_val: false,
        }
    }

    fn parse_row(input: &str) -> Vec<bool> {
        let mut res = Vec::default();
        for c in input.chars() {
            match c {
                '#' => res.push(true),
                '.' => res.push(false),
                _ => unreachable!(),
            }
        }
        res
    }

    fn parse_image(image_input: &str) -> Vec<Vec<bool>> {
        let mut res = Vec::default();
        for row in image_input.lines() {
            res.push(Self::parse_row(row));
        }
        res
    }

    fn get_pixel(&self, x: isize, y: isize) -> bool {
        if x < 0 && y < 0 {
            return self.void_val;
        }
        if let Some(row) = self.image.get(y as usize) {
            if let Some(&val) = row.get(x as usize) {
                return val;
            }
        }
        self.void_val
    }

    fn get_new_value(&self, x: isize, y: isize) -> bool {
        let idx: usize = self.get_pixel(x - 1, y - 1) as usize * 2usize.pow(8)
            + self.get_pixel(x, y - 1) as usize * 2usize.pow(7)
            + self.get_pixel(x + 1, y - 1) as usize * 2usize.pow(6)
            + self.get_pixel(x - 1, y) as usize * 2usize.pow(5)
            + self.get_pixel(x, y) as usize * 2usize.pow(4)
            + self.get_pixel(x + 1, y) as usize * 2usize.pow(3)
            + self.get_pixel(x - 1, y + 1) as usize * 2usize.pow(2)
            + self.get_pixel(x, y + 1) as usize * 2usize.pow(1)
            + self.get_pixel(x + 1, y + 1) as usize * 2usize.pow(0);
        self.algorithm[idx]
    }

    fn process(&mut self) {
        let width = self.image[0].len();
        let height = self.image.len();
        let mut next_image = vec![vec![false; width + 2]; height + 2];

        for y in 0..next_image.len() as isize {
            for x in 0..next_image[0].len() as isize {
                next_image[y as usize][x as usize] = self.get_new_value(x - 1, y - 1);
            }
        }

        self.void_val = if self.void_val {
            self.algorithm[self.algorithm.len() - 1]
        } else {
            self.algorithm[0]
        };
        self.image = next_image;
    }

    fn pixelcount(&self) -> usize {
        self.image.iter().flatten().filter(|&&x| x).count()
    }
}

fn main() {
    let (algorithm_input, image_input) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut processor = ImageProcessor::new(algorithm_input, image_input);
    for i in 0..50 {
        processor.process();
        if i == 1 {
            println!("Part 1: {}", processor.pixelcount());
        }
    }
    println!("Part 2: {}", processor.pixelcount());
}
