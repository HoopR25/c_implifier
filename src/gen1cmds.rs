//  A C-like for loop.
#[macro_export]
macro_rules! cfor
{
    ($i: stmt, $cond: expr, $to_do:expr, $code: block) =>
    {
        $i
        while($cond)
        {
            
            $code;
            $to_do;
        }
    };
}
pub use cfor;

//  A shortened version of the read_line() function in std::io::stdin(). Really simple, fast and easy.
#[macro_export]
macro_rules! readln {
    ($str: expr) => {
        $str = String::new();
        std::io::stdin().read_line(&mut $str).expect("Failed to read line.");
    };
}
pub use readln;

//  i++ in Rust. You write ipp!(); and put the int int the curly brackets, it increases the value by 1.
#[macro_export]
macro_rules! ipp {
    ($int: expr) => {
        $int += 1;
    };
}
pub use ipp;

//  c++ endl implemented as a macro in Rust.
#[macro_export]
macro_rules! endl {
	() => {
		println!("");
	};
}
pub use endl;

//  An easier way to convert to int from a string (that may be an input). Expects (string, int). Resets the string as an empty String, so you don't have to.
pub trait StrToInt{
    fn to_int(&mut self) -> i32;
} impl StrToInt for String
{
    fn to_int(&mut self) -> i32 {
        let int: i32 = self.trim().parse().unwrap();
        int
    }
}

//  The same as StrToInt, but it converts the string to u8, a much better variable type for bools, and it's potencial is really seen with the ToBool function.
pub trait StrToU8{
    fn to_u8(&mut self) -> u8;
} impl StrToU8 for String
{
    fn to_u8(&mut self) -> u8 {
        let int: u8 = self.trim().parse().unwrap();
        int
    }
}

//  An easier way to convert to bool from an int. Expects (int, bool). It sees if the int is 1 (then the bool is true) or not (then the bool is false).
pub trait ToBool {
    fn to_bool(self) -> bool;
} impl ToBool for u8 {
    fn to_bool(self) -> bool {
        match self {
            0 => false,
            1 => true,
            _ => panic!("The value is not valid!"),
        }
    }
}

//  A simple function to see if an integer is a prime number or not. You can use it on i32 variables.
pub trait IsPrimeNum {
    fn is_prime(self) -> bool;
} impl IsPrimeNum for i32 {
    fn is_prime (self) -> bool {
            if self < 1 {
                return false;
            }
            cfor!(let mut i: i32 = 2, i <= (self/2), i += 1,
            {
                if self % i == 0 {
                    return false;
                }

            });
            return true;
    }
}


//  Checks if a String is numeric and returns a boolean value (true or false)
pub trait IsNumeric {
    fn is_numeric(&self) -> bool;
} impl IsNumeric for String {
    fn is_numeric(&self) -> bool {
        self.parse::<f64>().is_ok()
    }
}
