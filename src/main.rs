struct FilterCondition {
    desired_value: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.desired_value
    }
}

fn custom_filter(collection: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut filtered = Vec::new();
    for item in collection {
        if condition.is_match(item) {
            filtered.push(*item);
        }
    }
    filtered
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 2, 3, 2];

    let condition = FilterCondition { desired_value: 2 };

    let filtered_numbers = custom_filter(&numbers, &condition);

    println!("Orijinal Koleksiyon: {:?}", numbers);
    println!("Filtrelenmiş Koleksiyon: {:?}", filtered_numbers);
}
