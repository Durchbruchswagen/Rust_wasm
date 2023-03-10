use std::ops;

#[derive(Copy,Clone)]
pub struct Complex{
    real: f32,
    im: f32,
}

impl Complex{
    pub fn new(real:f32,im:f32)->Complex{
        Complex{
            real,
            im,
        }
    }
    pub fn rad(&self)->f32{
        self.real*self.real+self.im*self.im
    }
}


impl ops::Add<Complex> for Complex{
     type Output = Complex;

    fn add(self,_rhs:Complex)->Complex{
        Complex{
            real: self.real+_rhs.real,
            im: self.im+_rhs.im,
        }
    }
}

impl ops::Mul<Complex> for Complex{
    type Output = Complex;
    
    fn mul(self,_rhs:Complex)->Complex{
        Complex{
            real:self.real*_rhs.real-self.im*_rhs.im,
            im: self.real*_rhs.im+self.im*_rhs.real,
        }
    }
}

pub fn computecolor(x:u32,y:u32,imheight:u32,imwidth:u32,creal:f32,cim:f32)->(i32,i32,i32){
    let zrad_max =10.0;
    let it_max = 255;
    let mut niter=0;
    let xmin=-2.0;
    let xmax=2.0;
    let ymin=-2.0;
    let ymax=2.0;
    let xwidth=(xmax-xmin)/(imwidth as f32);
    let yheight=(ymax-ymin)/(imheight as f32);
    let c=Complex::new(creal,cim);
    let mut z=Complex::new((x as f32)*xwidth+xmin,(y as f32)*yheight+ymin);
    while z.rad()<zrad_max&& niter<it_max{
        z=z*z+c;
        niter+=1;
    }
    let helpcol:i32=(niter<<21)+(niter<<10)+niter*8;
    let color1=(helpcol>>16)&255;
    let color2=(helpcol>>8)&255;
    let color3=(helpcol)&255;
    (color1,color2,color3)
}