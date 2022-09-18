use itertools::Itertools;

fn main() {
    dice_prop(2, |x: i32|x <= 2);
}

fn dice_prop<T>(num_dice: usize, comparer: T) -> f32 where T: Fn(i32) -> bool {
    let counter: i32 = 0;
    let perms = (1..=6).permutations(num_dice).unique();
    for perm in perms {
        let count: i32 = perm.iter().sum();
        println!("{:?}, {:?}",perm,  comparer(count))
    }
    counter as f32
}
