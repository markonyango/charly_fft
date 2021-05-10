use rug::{float::Constant, Assign, Complex, Float};

pub struct Fft {
    pub data: Vec<Complex>,
    n: usize,
    sign: i8,
    precision: u32,
}

impl Fft {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            n: 0,
            sign: 1,
            precision: 200,
        }
    }

    pub fn run(&mut self) {
      if self.data.len().is_power_of_two() == false {
          panic!("Length of input data must be a power of 2!");
      }

      self.bit_reversal();
      self.danielson_lanczos();
    }

    pub fn asign_data(&mut self, data: Vec<Complex>) {
        self.data = data;
        self.n = self.data.len();
    }

    fn danielson_lanczos(&mut self) {
        let mut w: Complex;
        let mut L = 2;

        loop {
            for k in 0..L / 2 {
                let mut kth: Float = Float::with_val(self.precision, 2 * k / L);
                kth = kth * Float::with_val(self.precision, Constant::Pi);
                if self.sign < 0 {
                    kth = kth * -1;
                }
                w = Complex::with_val(self.precision, (kth.cos_ref(), kth.sin_ref()));

                for j in 0..self.n / L {
                    let mut tao: Complex = Complex::new(self.precision);
                    let incomplete = &w * &self.data[j * L + k + L / 2];
                    tao.assign(incomplete);

                    let incomplete = &self.data[j * L + k] - tao.clone();
                    self.data[j * L + k + L / 2].assign(incomplete);

                    let incomplete = &self.data[j * L + k] + tao.clone();
                    self.data[j * L + k].assign(incomplete);
                }
            }

            L = L + L;
            if L > self.n {
                break;
            }
        }
    }

    pub fn bit_reversal(&mut self) {
      todo!();
    }
}

#[cfg(test)]
mod tests {
    use rug::Complex;

    #[test]
    fn can_create_fft() {
        let fft = super::Fft::new();
        assert!(fft.data.len() == 0);
    }

    #[test]
    fn can_assign_data() {
        let data = vec![
          Complex::with_val(53, (1.152922e-02,0)),
          Complex::with_val(53, (4.611686e-02,0)),
          Complex::with_val(53, (9.684541e-02,0)),
          Complex::with_val(53, (1.420399e-01,0)),
          Complex::with_val(53, (1.633459e-01,0)),
          Complex::with_val(53, (1.568121e-01,0)),
          Complex::with_val(53, (1.306767e-01,0)),
          Complex::with_val(53, (9.707415e-02,0)),
          Complex::with_val(53, (6.552505e-02,0)),
          Complex::with_val(53, (4.077114e-02,0)),
          Complex::with_val(53, (2.364726e-02,0)),
          Complex::with_val(53, (1.289851e-02,0)),
          Complex::with_val(53, (6.664228e-03,0)),
          Complex::with_val(53, (3.280851e-03,0)),
          Complex::with_val(53, (1.546687e-03,0)),
          Complex::with_val(53, (7.011647e-04,0)),
          Complex::with_val(53, (3.067596e-04,0)),
          Complex::with_val(53, (1.299217e-04,0)),
          Complex::with_val(53, (5.341225e-05,0)),
          Complex::with_val(53, (2.136490e-05,0)),
          Complex::with_val(53, (8.332311e-06,0)),
          Complex::with_val(53, (3.174214e-06,0)),
          Complex::with_val(53, (1.183116e-06,0)),
          Complex::with_val(53, (4.320946e-07,0)),
          Complex::with_val(53, (1.548339e-07,0)),
          Complex::with_val(53, (5.450153e-08,0)),
          Complex::with_val(53, (1.886591e-08,0)),
          Complex::with_val(53, (6.428385e-09,0)),
          Complex::with_val(53, (2.158101e-09,0)),
          Complex::with_val(53, (7.144058e-10,0)),
          Complex::with_val(53, (2.333726e-10,0)),
          Complex::with_val(53, (7.528147e-11,0)),
        ];

        let mut fft = super::Fft::new();
        fft.asign_data(data);
        fft.danielson_lanczos();
        println!("Danielson Lanczos step");
        println!("{:?}", fft.data);

        assert_eq!(fft.data[0], (1, -1));
    }
}

