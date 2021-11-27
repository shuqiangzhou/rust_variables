#[allow(unused, dead_code)]

// 类型系统number,以下以u8和i8作为演示
fn main() {
    // number类型的运算最大位数运算规则:实际如果取值:`2u8.pow(8)-1)`或者`-2i16.pow(7)`会表现出错,次方运算会报错,因为已经算数溢出了
    // u 8,16,32,64,128->位数 0-2^(n-1) = 255

    println!("u8区间值:{}-{}", 0, 2u16.pow(8) - 1);

    // i 前缀的区间值
    println!("i前缀的区间值:{}-{}", -2i16.pow(7), 2i16.pow(7) - 1);

    println!(
        "u8 min({}),max({}),i8 min({}),max({})",
        u8::MIN,
        u8::MAX,
        i8::MIN,
        i8::MAX
    );

    // 溢出处理
    let mut u8_overflow: u8 = 255;

    // debug下会报错,但是release下是不会报错的,溢出部分会从最小值进行计算
    // let data = u8_overflow.add(3);
    // println!("release模式下此不会触发错误:{:?},预期为:0",data);

    // 使用overflow相关函数可以很友好的检测和处理溢出异常
    // overflowing_*:检查是否溢出,返回溢出多少,和是否溢出的元组数据,但是可以很友好的处理,溢出后返回的是溢出的数据
    let (num, is_overflow) = &u8_overflow.overflowing_add(2);
    println!("是否溢出:{},溢出后返回数据:{}", is_overflow, &num);

    // checked_*:检查溢出的值,如果未溢出则正常返回值,溢出返回none
    let some = &u8_overflow.checked_add(2);
    println!("溢出返回结果:{:?},溢出返回值:{:?}", some.is_none(), some);

    // saturating_*:函数返回溢出了最高的值,比如:u8的最高值,无论如何加减乘除,最高值只可能为:255
    let mut u8_min_oveflow: i8 = 125;
    let data = &u8_min_oveflow.saturating_add(1);

    println!(
        "未溢出返回结果:{},溢出返回结果:{}",
        data,
        u8_min_oveflow.saturating_add(3)
    );

    // wrapping_*:方法定义了溢出后会变化为:高位溢出变最低位累计,低位溢出最高向下减
    let mut i8_data: i8 = 126;
    println!(
        "高位溢出:{:?},低位溢出:{:?}",
        127i8.wrapping_add(2),
        -127i8.wrapping_sub(-2)
    );

    //不同类型不能进行累加

    let num_u8 = 100u8;

    let num_i8 = 100i8;

    // 不同类型不允许进行累加,否则报错
    // println!("不同类型不允许进行累加:{}",num_u8.wrapping_add(num_i8));

    //可以转换到其他高位计算
    println!(
        "可以转换到其他高位计算:{:?}",
        (num_i8 as i32).overflowing_add(num_u8 as i32)
    );

    // 强转类型:类型强转只能转换
    println!(
        "强转:{:?},强转2:{:?},强转溢出:{:?},ASSIC转换测试:{:?}",
        "123哈哈".parse::<i32>().unwrap_or(-1),
        "哈123哈".parse::<i32>().unwrap_or(-1),
        "128".parse::<u8>().unwrap_or(0),
        "a".parse::<u8>().unwrap_or(0),
    );

    // 对数据的二进制形式的0或者1的统计
    println!(
        "零的个数:{},一的个数:{},二进制共有多少零开头:{},零的个数:{},一的个数:{}",
        0b0010_0110u8.count_ones(),
        0b0001_0110u8.count_zeros(),
        0b0001_0110u8.leading_zeros(),
        10u8.count_ones(),
        10u8.count_zeros()
    );
}
