//                 scope -----v
fn imil_say(sentence: String) {
    println!("{}", sentence);
}

fn main() {
    let msg = String::from("RFS rules !");

    imil_say(msg);

    // Say it again loud !
    imil_say(msg);
    // ^-- le problème est le même que dans rfs_05.
    // msg est moved (perte de l'ownership) dans la fonction imil_say() lors du premier appel.
    // la variable msg ne peut donc plus être utilisée pour le second appel de la fonction.
}

//  🦉 uggla   master  …  rfs  rfs_06  problem  cargo build
//    Compiling problem v0.1.0 (/home/uggla/workspace/rust/rfs/rfs_06/problem)
// error[E0382]: use of moved value: `msg`
//   --> src/main.rs:12:14
//    |
// 7  |     let msg = String::from("RFS rules !");
//    |         --- move occurs because `msg` has type `String`, which does not implement t
// he `Copy` trait
// 8  |
// 9  |     imil_say(msg);
//    |              --- value moved here
// ...
// 12 |     imil_say(msg);
//    |              ^^^ value used here after move
//
// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `problem` due to previous error
