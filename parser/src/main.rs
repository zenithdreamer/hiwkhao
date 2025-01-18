use parser::Parser;

fn main() {
    let input = r"23+8
25 * 0
5NUM^ 3.0
x=5
10*x
x=y
x!=5
(2+5)
x = list[2]
x[0] + x[1]
";
    let mut parser = Parser::new(vec![]);
    let mut result = parser.parse_file_pretty(input);

    //println!("{:?}", result);

    //parser = Parser::new(vec![]);
    //result = parser.parse_file(input);

    //println!("{:?}", result);
}
