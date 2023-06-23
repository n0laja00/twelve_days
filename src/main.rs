
fn main() {
    println!("Welcome to twelve days of Christmas!"); 
    let events: [&str; 12] = [
        "On the first day of Christmas
        My true love sent to me
        A partridge in a pear tree",
        "On the second day of Christmas
        My true love sent to me
        Two turtle doves and
        A partridge in a pear tree",
        "On the third day of Christmas
        My true love sent to me
        Three French hens
        Two turtle doves and
        A partridge in a pear tree",
        "On the forth day of Christmas
        My true love sent to me
        Four calling birds
        Three French hens
        Two turtle doves and
        A partridge in a pear tree",
        "On the fifth day of Christmas
        My true love sent to me
        Five golden rings
        Four calling birds
        Three French hens
        Two turtle doves and
        A partridge in a pear tree",
        "On the sixth day of Christmas
        My true love sent to me
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three French hens
        Two turtle doves and
        A partridge in a pear tree",
        "On the seventh day of Christmas
        My true love sent to me
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three French hens
        Two turtle doves and
        A partridge in a pear tree",
        "On the eighth day of Christmas
        My true love sent to me
        Eight maids a-milking
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three French hens
        Two turtle doves and
        A partridge in a pear tree",
        "On the ninth day of Christmas
        My true love sent to me
        Nine ladies dancing
        Eight maids a-milking
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three French hens
        Two turtle doves and
        A partridge in a pear tree",
        "On the tenth day of Christmas
        My true love sent to me
        Ten lords a-leaping
        Nine ladies dancing
        Eight maids a-milking
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three French hens
        Two turtle doves and
        A partridge in a pear tree",
        "On the eleventh day of Christmas
        My true love sent to me
        Eleven pipers piping
        Ten lords a-leaping
        Nine ladies dancing
        Eight maids a-milking
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three French hens
        Two turtle doves and
        A partridge in a pear tree",
        "On the twelfth day of Christmas
        My true love sent to me
        Twelve drummers drumming
        Eleven pipers piping
        Ten lords a-leaping
        Nine ladies dancing
        Eight maids a-milking
        Seven swans a-swimming
        Six geese a-laying
        Five golden rings
        Four calling birds
        Three French hens
        Two turtle doves and
        A partridge in a pear tree"
    ];
    let days: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 ];

    //print all events using element...
    for element in events {
        println!("{element}");
    };

    //Statically typed number of days....
    for number in 0..12 {
        println!("{} {}", days[number], events[number] );
    };

    //Get every number from index 0 until the end of the days array. 
    for number in 0..days.len() {
        println!("{} {}", days[number], events[number] );
    };
}
