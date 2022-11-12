// Petit ajout suite au stream 07 ou @Imilnb a été surpris en fin de stream.
// C'est le piège classique avec les types primitifs. --> IT IS A TRAP !

fn imil_say(mut sentence: u32) {
    sentence += 1;
    println!("{}", sentence);
}

fn main() {
    let msg = 123;
    // ^--- pas besoin de mut car on va copier implicitement. (cf ci-dessous)
    // dit autrement msg n'est jamais modifié, car on va en faire une copie.
    imil_say(msg);

    // Say it again loud !
    imil_say(msg);
    // ^-- ici il n'y a pas de problème d'ownership car msg est un type primitif,
    // les types primitifs sont copiés, car il n'y a pas/peu d'impact de
    // performance (on verra qu'ils implémentent le trait Copy).
    // on voit bien ici qu'il y a une copy, car au deuxième appel de la fonction
    // msg = 124 et pas 125.
}

//  🦉 uggla   master  …  rfs  rfs_06  problem_rfs_07  cargo run
//    Compiling problem_rfs_07 v0.1.0 (/home/uggla/workspace/rust/rfs/rfs_06/problem_rfs_07)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.52s
//      Running `target/debug/problem_rfs_07`
// 124
// 124  <- deuxième appel
