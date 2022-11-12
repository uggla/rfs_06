// Ici on utilise une ref pour pas copier.

fn imil_say(sentence: &mut u32) {
    //                ^- on prend une ref mutable
    *sentence += 1;
    // ^-- * pour déréférencer pour utiliser la valeur.
    println!("{}", sentence);
}

fn main() {
    let mut msg = 123;
    //   ^-- ici on ne copie plus donc il faut déclarer la variable mutable.
    imil_say(&mut msg);
    //       ^-- on utilise la ref mutable.

    // Say it again loud !
    imil_say(&mut msg);
    //       ^-- on utilise la ref mutable.
}

//  🦉 uggla   master  …  rfs  rfs_06  problem_rfs_07_bis  cargo run
//    Compiling problem_rfs_07_bis v0.1.0 (/home/uggla/workspace/rust/rfs/rfs_06/problem_rfs
// _07_bis)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.41s
//      Running `target/debug/problem_rfs_07_bis`
// 124
// 125 <--- c'est bien incrémenté une 2ème fois
