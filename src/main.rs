use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let a=20;
    let exponent=62;
    let modulo=17;
    let res=powering(a,exponent,modulo);
    println!("{}^{} mod {}={}",a,exponent,modulo,res);
}
fn powering( a:u128,mut b:u128,modulo:u32)->BigUint{
    let mut res=BigUint::one();
    let mut base =BigUint::from(a);
    let  modulos=BigUint::from(modulo);
    base%=&modulos;
    while b>0{
        if b%2==1{
            res=(res*&base)%&modulos;
        }
        base=(&base*&base) % &modulos;
        b/=2;
    }
    res.into()
    
    
}
