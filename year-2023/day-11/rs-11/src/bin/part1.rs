use rs_2023_11::*;

fn main() {
    let file = *INPUT_1;
    let image = GalaxyImage::<2>::new(file);
    let result = image.product_sum();
    println!("{result}");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first_test() {
        let file = rs_2023_11::static_read("example2.txt");
        let image = GalaxyImage::<2>::new(file);
        let result = image.product_sum();
        assert_eq!(result, 374);
    }
}
