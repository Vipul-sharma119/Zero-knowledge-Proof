use num_bigint::{BigUint, RandBigInt};
use rand::{Rng, thread_rng};
use std::ops::Rem;

pub struct ZKP{
    pub gen_g:BigUint,
    pub gen_h:BigUint,
    pub modulus_p:BigUint,
    pub subgrp_q:BigUint,
}

impl ZKP{
    
    // to find mod of exp
    pub fn mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
        base.modpow(exp, modulus)
    }

    pub fn random(bound: &BigUint) -> BigUint {
        let mut rng = thread_rng();
        rng.gen_biguint_below(bound)
    }

    //s=r-cx mod(q) , r = random no. , c= challenge , x = secrect key , q= subgrp
    pub fn response(&self ,r:&BigUint,c:&BigUint,x:&BigUint)->BigUint{
        let cx: &BigUint = &(c*x);
        let exp=BigUint::from(1u32);
        if *r >= *cx{
            (r-(cx).modpow(&exp,&self.subgrp_q)).modpow(&exp,&self.subgrp_q) 
        }
        else {
            &self.subgrp_q-(cx-(r).modpow(&exp,&self.subgrp_q)).modpow(&exp,&self.subgrp_q).modpow(&exp,&self.subgrp_q)
        }
    }

    //t1=((g^s)mod p * (y1^c)mod p)mod p && t2=((h^s)modp * (y2^c)mod p)mod p

    fn verify(&self,t1:&BigUint,t2:&BigUint,c:&BigUint,s:&BigUint,y1:&BigUint,y2:&BigUint)->bool{
        let exp=BigUint::from(1u32);
        let lhs_1=t1;
        let lhs_2 =t2;
        let rhs_1=((&self.gen_g.modpow(s, &self.modulus_p)) *(y1.modpow(c, &self.modulus_p))).modpow(&exp, &self.modulus_p);
        let rhs_2=((&self.gen_h.modpow(s, &self.modulus_p)) *(y2.modpow(c, &self.modulus_p))).modpow(&exp, &self.modulus_p);

        lhs_1 ==&rhs_1 && lhs_2==&rhs_2

    }

}

#[cfg(test)]

mod test{
    use super::*;
    #[test]
    fn fixed_values(){
        let gen_h   = BigUint::from(3u32);
        let gen_g  = BigUint::from(2u32);
        let subgrp_q  = BigUint::from(11u32);
        let modulus_p = BigUint::from(23u32);
        let x  = BigUint::from(4u32);
        let c: BigUint =   BigUint::from(5u32);

        let y1=ZKP::mod_exp(&gen_g, &x,&modulus_p);
        let y2=ZKP::mod_exp(&gen_h, &x,&modulus_p);

        assert!(y1 == BigUint::from(16u32)); //assert 
        assert!(y2 == BigUint::from(12u32));

       // let r =ZKP::random(&subgrp_q);
        let t1 =ZKP::mod_exp(&gen_g, &BigUint::from(6u32),&modulus_p);
        let t2 =ZKP::mod_exp(&gen_h, &BigUint::from(6u32),&modulus_p);

        assert!(t1 == BigUint::from(18u32)); //assert 
        assert!(t2 == BigUint::from(16u32));
        
        let ZKproof = ZKP {
            modulus_p,
            subgrp_q,
            gen_g,
            gen_h,
        };
        let s = ZKproof.response(&BigUint::from(6u32), &c, &x);
        assert!(s == BigUint::from(8u32));

         let result = ZKproof.verify(
            &t1,
            &t2,
            &c,
            &s,
            &y1,
            &y2
            
        );
        assert!(result == true);

    }
}

 #[test]
    fn dynamic_values() {
        let modulus_p = BigUint::from(23u32);
        let subgrp_q = BigUint::from(11u32);
        let gen_g = BigUint::from(2u32);
        let gen_h = BigUint::from(3u32);
        let x = BigUint::from(4u32);
        let c = BigUint::from(5u32);


        let y1 = ZKP::mod_exp(&gen_g, &x, &modulus_p);
        let y2 = ZKP::mod_exp(&gen_h, &x, &modulus_p);
        assert!(y1 == BigUint::from(16u32));
        assert!(y2 == BigUint::from(12u32));


        let r = ZKP::random(&subgrp_q);
        let t1 = ZKP::mod_exp(&gen_g, &r, &modulus_p);
        let t2 = ZKP::mod_exp(&gen_h, &r, &modulus_p);


        let zkproof = ZKP {
            modulus_p,
            subgrp_q,
            gen_g,
            gen_h,
        };
        let s = zkproof.response(&r, &c, &x);


        let result = zkproof.verify(
             &t1,
            &t2,
            &c,
            &s,
            &y1,
            &y2 
        );
        assert!(result == true);
    }


#[test]
    fn test_with_random_values() {
        let modulus_p = BigUint::from(23u32);
        let subgrp_q = BigUint::from(11u32);
        let gen_g = BigUint::from(2u32);
        let gen_h = BigUint::from(3u32);


        let zkproof = ZKP {
            modulus_p: modulus_p.clone(),
            subgrp_q: subgrp_q.clone(),
            gen_g: gen_g.clone(),
            gen_h: gen_h.clone(),
        };


        for _ in 0..10 {
            let mut rng = thread_rng();


            let x = rng.gen_biguint_below(&subgrp_q);
            let c = rng.gen_biguint_below(&subgrp_q);
            let r = rng.gen_biguint_below(&subgrp_q);


            let y1 = ZKP::mod_exp(&gen_g, &x, &modulus_p);
            let y2 = ZKP::mod_exp(&gen_h, &x, &modulus_p);


            let t1 = ZKP::mod_exp(&gen_g, &r, &modulus_p);
            let t2 = ZKP::mod_exp(&gen_h, &r, &modulus_p);


            let s = zkproof.response(&r, &c, &x);


            let result = zkproof.verify(
                 &t1,
            &t2,
            &c,
            &s,
            &y1,
            &y2
            );


            assert!(
                result,
                "ZKP failed for random test case:\n\
            x = {}\n c = {}\n r = {}\n s = {}",
                x, c, r, s
            );
        }
    }




