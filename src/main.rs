fn main() {
    let x = 5;
    {
      let x = 99;
      print!("x: {}\n", x);
    }
    print!("x: {}\n", x);

    let mut y = 100;
    let y = 10;
    print!("y: {}\n", y);

    let meme = "Hari OM!!";
    // let meme = make_image(meme);
    print!("meme: {}\n", meme);
}
