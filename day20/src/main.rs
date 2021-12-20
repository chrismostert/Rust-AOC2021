struct ImageProcessor {
    image: Vec<Vec<bool>>,
    algorithm: Vec<bool>,
    void_val: bool,
}

impl ImageProcessor {
    fn new(algorithm_input: &str, image_input: &str) -> Self {
        ImageProcessor {
            algorithm: Self::parse_row(algorithm_input),
            image: Self::parse_image(image_input),
            void_val: false,
        }
    }

    fn parse_row(input: &str) -> Vec<bool> {
        input.chars().fold(Vec::default(), |mut acc, c| {
            match c {
                '#' => acc.push(true),
                '.' => acc.push(false),
                _ => unreachable!(),
            }
            acc
        })
    }

    fn parse_image(image_input: &str) -> Vec<Vec<bool>> {
        image_input.lines().fold(Vec::default(), |mut acc, row| {
            acc.push(Self::parse_row(row));
            acc
        })
    }

    fn get_pixel(&self, x: isize, y: isize) -> bool {
        if let Some(row) = self.image.get(y as usize) {
            if let Some(&val) = row.get(x as usize) {
                return val;
            }
        }
        self.void_val
    }

    fn get_new_value(&self, x: isize, y: isize) -> bool {
        let vals = (y - 1..=y + 1)
            .scan(0, |_, y| {
                Some((x - 1..=x + 1).scan(0, move |_, x| Some(self.get_pixel(x, y) as usize)))
            })
            .flatten();
        let exps = (0..=8).rev().scan(0, |_, x| Some(2usize.pow(x)));
        self.algorithm[vals.zip(exps).fold(0, |acc, (val, exp)| acc + val * exp)]
    }

    fn process(&mut self) {
        let mut next_image = vec![vec![false; self.image[0].len() + 2]; self.image.len() + 2];

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
