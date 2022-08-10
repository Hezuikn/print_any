```rust
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
```

<pre>
$dbg: [
    'h',
    't',
    't',
    'p',
    's',
    ':',
    '/',
    '/',
    't',
    'w',
    'i',
    't',
    't',
    'e',
    'r',
    '.',
    'c',
    'o',
    'm',
    '/',
    'h',
    'e',
    'z',
    'u',
    'i',
    'k',
    'n',
]
your waifu loves you
&mut sekai
based slavery
42
noone!
sum: entirely optional text uwu
ok: i wish you eternal happiness
err: fuck you cupcake is best girl
err: <a>https://www.youtube.com/watch?v=-ZI_PkdxMmE</a>
dbg: [
    "<a>https://www.youtube.com/watch?v=0_ftyQCj3Mw</a>",
    "present text",
]
/empty/
src/main.rs:17:5 "print_any::main::Marriage" /type/"print_any::main::Marriage"
</pre>
