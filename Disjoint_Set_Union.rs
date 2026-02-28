	use std::io::*;
	use std::cmp::*;
	use std::collections::*;

	struct dsu{
		p:Vec<usize>,
		sz:Vec<usize>,
	}

	impl dsu{
		fn new(n:usize)->Self{
			let mut p=vec![0;n];
			let sz=vec![1;n];
			for i in 0..n{
				p[i]=i;
			}
			Self{p,sz}
		}

		fn find(&mut self,x:usize)->usize{
			if self.p[x]!=x{
				let px=self.find(self.p[x]);
				self.p[x]=px;
			}
			self.p[x]
		}

		fn unite(&mut self,a:usize,b:usize)->bool{
			let mut x=self.find(a);
			let mut y=self.find(b);
			if x==y{return false;}
			if self.sz[x]<self.sz[y]{
				std::mem::swap(&mut x,&mut y);
			}
			self.p[y]=x;
			self.sz[x]+=self.sz[y];
			true
		}
	}
