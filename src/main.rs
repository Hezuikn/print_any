use print_any::{print_any, print_any_dbg};

fn main() {
    print_any("https://twitter.com/hezuikn".chars());
    print_any(&&&&"your waifu loves you");
    print_any(&mut "&mut sekai".to_owned());
    print_any("based slavery".to_owned());
    print_any(42);
    print_any(None::<()>);
    print_any(Some("entirely optional text uwu"));
    print_any(Ok::<_, ()>("i wish you eternal happiness"));
    print_any(Err::<(), _>("fuck you cupcake is best girl"));
    print_any(&Err::<(), _>("https://www.youtube.com/watch?v=-ZI_PkdxMmE"));
    print_any(["https://www.youtube.com/watch?v=0_ftyQCj3Mw", "present text"]);
    print_any("");
    struct Marriage;
    print_any_dbg(Marriage);
}
