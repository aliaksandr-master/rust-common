// types:
// i8, i16, i32, i64, i128
// u8, u16, i32, i64, i128
// usize, isize - длина укозателя по сути. зависит от архитектуры 64 или 32
// f32, f64 - floating types
// String, - байтовы массив, и длинна массив, может содержать нулевые символы/ ПО УМОЛЧАНИЮ UTF8
// &str - указатель на слайс строки

// OsString - редко юзается. компилится в разный стрин в зависимости от ОС
// &OsStr -указатель на слайс OsString

// ; - the end of exp
// no ; - return the value


pub fn calc_sum(a: f32, b: f32) -> f32 {
    a + b // return the value
//    (a + b) as f32 // return the casted typed value
//    return a + b; // return the value
}
