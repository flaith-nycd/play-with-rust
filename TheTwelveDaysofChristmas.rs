/**
* What we want:
* found in https://www.poetryfoundation.org/poems/42913/the-twelve-days-of-christmas

       The first day of Christmas,
       My true love sent to me
       A partridge in a pear tree.

       The second day of Christmas,
       My true love sent to me
       Two turtle doves, and
       A partridge in a pear tree.

       The third day of Christmas,
       My true love sent to me
       Three French hens,
       Two turtle doves, and
       A partridge in a pear tree.

       The fourth day of Christmas,
       My true love sent to me
       Four colly birds,
       Three French hens,
       Two turtle doves, and
       A partridge in a pear tree.

       The fifth day of Christmas,
       My true love sent to me
       Five gold rings,
       Four colly birds,
       Three French hens,
       Two turtle doves, and
       A partridge in a pear tree.

       The sixth day of Christmas,
       My true love sent to me
       Six geese a-laying,
       Five gold rings,
       Four colly birds,
       Three French hens,
       Two turtle doves, and
       A partridge in a pear tree.

       The seventh day of Christmas,
       My true love sent to me
       Seven swans a-swimming,
       Six geese a-laying,
       Five gold rings,
       Four colly birds,
       Three French hens,
       Two turtle doves, and
       A partridge in a pear tree.

       The eighth day of Christmas,
       My true love sent to me
       Eight maids a-milking,
       Seven swans a-swimming,
       Six geese a-laying,
       Five gold rings,
       Four colly birds,
       Three French hens,
       Two turtle doves, and
       A partridge in a pear tree.

       The ninth day of Christmas,
       My true love sent to me
       Nine drummers drumming,
       Eight maids a-milking,
       Seven swans a-swimming,
       Six geese a-laying,
       Five gold rings,
       Four colly birds,
       Three French hens,
       Two turtle doves, and
       A partridge in a pear tree.

       The tenth day of Christmas,
       My true love sent to me
       Ten pipers piping,
       Nine drummers drumming,
       Eight maids a-milking,
       Seven swans a-swimming,
       Six geese a-laying,
       Five gold rings,
       Four colly birds,
       Three French hens,
       Two turtle doves, and
       A partridge in a pear tree.

       The eleventh day of Christmas
       My true love sent to me
       Eleven ladies dancing,
       Ten pipers piping,
       Nine drummers drumming,
       Eight maids a-milking,
       Seven swans a-swimming,
       Six geese a-laying,
       Five gold rings,
       Four colly birds,
       Three French hens,
       Two turtle doves, and
       A partridge in a pear tree.

       The twelfth day of Christmas
       My true love sent to me
       Twelve fiddlers fiddling,
       Eleven ladies dancing,
       Ten pipers piping,
       Nine drummers drumming,
       Eight maids a-milking,
       Seven swans a-swimming,
       Six geese a-laying,
       Five gold rings,
       Four colly birds,
       Three French hens,
       Two turtle doves, and
       A partridge in a pear tree.
 */

fn main() {
    let array_days = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let array_lines = ["Twelve fiddlers fiddling,", "Eleven ladies dancing,", "Ten pipers piping,", "Nine drummers drumming,",
        "Eight maids a-milking,", "Seven swans a-swimming,", "Six geese a-laying,", "Five gold rings,",
        "Four colly birds,", "Three French hens,", "Two turtle doves, and", "A partridge in a pear tree."];

    let total_lines = array_lines.len();    // 12
    let mut index = total_lines;

    // From version 1.53.0, no need to use .iter()
    // for element in array_days.iter() {
    for element in array_days {
        index -= 1;
        println!("The {} day of Christmas,", element);
        println!("My true love sent to me");

        // This yields values from index (inclusive) to total_lines (exclusive) in steps of one.
        // for x in 0..10 : result from 0 to 9
        // so here, from index to total_lines - 1 (11 to 11)
        for line in index..total_lines {
            println!("{}", array_lines[line]);
        }
        println!();
    }
}
