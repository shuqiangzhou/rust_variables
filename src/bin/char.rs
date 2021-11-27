#[allow(unused,unused_imports,unused_doc_comments)]
use std::{char, ops::Add};

/// `char`类型
fn main() {

    // `char`知识梳理
    // 1.大小固定的四个字节的`utf-8`字符的[码点][也就是16进制]也就是内存为:2^32-1的内存大小
    // 2.是一个16进制编码,16进制编码即是`unicode`编码,`unicode`和utf-8的编码点是不一致的
    // 3.16进制转二进制进行二进制字节补码:比如字节:10010,补码为:00010010
    // 4.十六进制转二进制转`utf-8`编码规则
    // ```code
    // UTF-8 最大的一个特点，就是它是一种变长的编码方式。它可以使用1~4个字节表示一个符号，根据不同的符号而变化字节长度。
    //
    // UTF-8 的编码规则很简单，只有二条：
    //
    // 对于单字节的符号，字节的第一位设为0，后面7位为这个符号的 Unicode 码。因此对于英语字母，UTF-8 编码和 ASCII 码是相同的。
    //
    // 对于n字节的符号（n > 1），第一个字节的前n位都设为1，第n + 1位设为0，后面字节的前两位一律设为10。剩下的没有提及的二进制位，全部为这个符号的 Unicode 码。
    //
    // 下表总结了编码规则，字母x表示可用编码的位。
    //
    // Unicode符号范围     |        UTF-8编码方式
    // (十六进制)          |              （二进制）
    // ----------------------+---------------------------------------------
    // 0000 0000-0000 007F | 0xxxxxxx
    // 0000 0080-0000 07FF | 110xxxxx 10xxxxxx
    // 0000 0800-0000 FFFF | 1110xxxx 10xxxxxx 10xxxxxx
    // 0001 0000-0010 FFFF | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
    //
    // 跟据上表，解读 UTF-8 编码非常简单。如果一个字节的第一位是0，则这个字节单独就是一个字符；如果第一位是1，则连续有多少个1，就表示当前字符占用多少个字节。
    //
    // 下面，还是以汉字严为例，演示如何实现 UTF-8 编码。
    //
    // 严的 Unicode 是4E25（100111000100101），根据上表，可以发现4E25处在第三行的范围内（0000 0800 - 0000 FFFF），因此严的 UTF-8 编码需要三个字节，
    // 即格式是1110xxxx 10xxxxxx 10xxxxxx。然后，从严的最后一个二进制位开始，依次从后向前填入格式中的x，多出的位补0。这样就得到了，
    // 严的 UTF-8 编码是11100100 10111000 10100101，转换成十六进制就是E4B8A5。
    // 为一个unicode的编码,`unicode`是以16进制的编码,是一个整数
    //  [`char详细解说`](https://www.orchome.com/1099)类型的说明情况,以及unicode是如何进行转换的
    // ```

    

    println!(
        "unicode节点:0-0x007F:{:?},0x007F-0x07FF:{:?},0x07FF-0xFFFF:{:?},大于0xFFFF:{:?}",
        (0x007F as u32).to_be_bytes(),
        (0x07FF as u32).to_be_bytes(),
        (0xFFFF as u32).to_be_bytes(),
        (0xFFFF as u32).to_be_bytes(),
    );
    println!(
        "unicode节点:0-0x007F:{},0x007F-0x07FF:{},0x07FF-0xFFFF:{},大于0xFFFF:{}",
        0x007F as u32, 0x07FF as u32, 0xFFFF as u32, 0xFFFF as u32
    );


    //  Unicode符号范围     |        UTF-8编码方式
    // (十六进制)          |              （二进制）
    // ----------------------+---------------------------------------------
    // 0000 0000-0000 007F | 0xxxxxxx
    // 0000 0080-0000 07FF | 110xxxxx 10xxxxxx
    // 0000 0800-0000 FFFF | 1110xxxx 10xxxxxx 10xxxxxx
    // 0001 0000-0010 FFFF | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx

    let c = '严';
    let mut bin_yan: String = String::from("");
    (c as u32).to_be_bytes().map(|char| {
        if char > 0 {
            bin_yan.push_str(format!("{:0>8b}_", char).as_str());   
        }
        println!(
            "二进制内存字节补码后表现:{:0>8b},16进制的表现形式:{:x}",
            char, char
        );
    });
    let bin_yan =  bin_yan.trim_end_matches(|x|{
        x=='_'
    });
   
    // 在第三排 `4E25` 对应的`utf-8`编码为:1110xxxx 10xxxxxx 10xxxxxx,然后把`严`的二进制由低位开始替换`x`
    // 严的`unicode`二进制:`01001110_00100101`
    // 01001110_00100101
    // 1110xxxx 10xxxxxx 10xxxxxx
    // 替换结果
    // 1110-0100 10-1110_00 10-100101
    // 严的`unicode`完整的二进制编码为:`111001001011100010100101`

    println!(
        "`严`所占内存:{}字节,`严`完整的16进制:{:X},`严`unicode码点二进制完整形式:{},`utf-8`16进制形式:{:X},u32所占内存:{}字节",
        std::mem::size_of_val::<char>(&c),
        c as u32,
        bin_yan,
        0b111001001011100010100101,
        std::mem::size_of::<u32>(),
    );

    // 结论
    // 通过上一节的例子，可以看到严的 Unicode码点是4E25，UTF-8 码点是E4B8A5，两者是不一样的


}
