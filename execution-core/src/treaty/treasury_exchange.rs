use super::economic_treaty::Hash; pub fn exchange_treasury(a:Hash,b:Hash)->Hash{ let mut o=[0;32]; for i in 0..32{o[i]=a[i]^b[i];} o }
