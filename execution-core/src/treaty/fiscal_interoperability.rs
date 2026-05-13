use super::economic_treaty::Hash; pub fn coordinate_fiscal(a:Hash,b:Hash)->Hash{ let mut o=[0;32]; for i in 0..32{o[i]=a[i].wrapping_add(b[i]);} o }
