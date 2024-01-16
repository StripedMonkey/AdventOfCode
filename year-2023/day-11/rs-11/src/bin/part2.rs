use rs_2023_11::*;

fn main() {
    let file = *INPUT_1;
    let image = GalaxyImage::<1_000_000>::new(file);
    let result = image.product_sum();
    println!("{result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn galaxy_2x() {
        let file = rs_2023_11::static_read("example2.txt");
        let image = GalaxyImage::<2>::new(file);
        let result = image.product_sum();
        assert_eq!(result, 374);
    }

    #[test]
    fn galaxy_10x() {
        let file = rs_2023_11::static_read("example2.txt");
        let image = GalaxyImage::<10>::new(file);
        let result = image.product_sum();
        assert_eq!(result, 1030);
    }

    #[test]
    fn galaxy_100x() {
        let file = rs_2023_11::static_read("example2.txt");
        let image = GalaxyImage::<100>::new(file);
        let result = image.product_sum();
        assert_eq!(result, 8410);
    }

    #[test]
    fn galaxy_1_000_000x() {
        let file = rs_2023_11::static_read("example2.txt");
        let image = GalaxyImage::<1_000_000>::new(file);
        let result = image.product_sum();
        assert_eq!(result, 82_000_210);
    }
}
