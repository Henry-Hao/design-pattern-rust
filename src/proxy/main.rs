trait Image {
    fn display(&self);
}

struct RealImage {
    filename: String
}

struct ProxyImage {
    filename: String,
    image: RealImage
}

impl RealImage {
    fn new(filename: String) -> Self {
        Self {
            filename
        }
    }
}

impl Image for RealImage {
    fn display(&self) {
        println!("Real image: {}", self.filename);
    }
}

impl ProxyImage {
    fn new(filename: String) -> Self {
        Self {
            image: RealImage::new(filename.clone()),
            filename,
        }
    }
}

impl Image for ProxyImage {
    fn display(&self) {
        self.image.display()
    }
}

fn main() {
    let real = RealImage::new("Real.img".to_owned());
    real.display();
    let proxy = ProxyImage::new("Proxy.img".to_owned());
    proxy.display();
}
