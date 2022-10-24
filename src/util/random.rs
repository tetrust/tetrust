use rand::seq::SliceRandom;

pub fn random_select<T: Clone>(array: &[T]) -> T {
    let temp = array.to_vec();

    let mut rng = rand::thread_rng();

    temp.choose_multiple(&mut rng, 1).next().unwrap().to_owned()
}

pub fn shuffle<T: Clone>(array: &[T]) -> impl Iterator<Item = T> {
    let mut temp = array.to_vec();

    let mut rng = rand::thread_rng();
    temp.shuffle(&mut rng);

    temp.into_iter()
}
