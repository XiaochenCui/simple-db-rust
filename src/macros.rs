#![feature(raw)]
#![feature(maybe_uninit_ref)]

#[test]
fn test_trait_obj_as_hashmap_key() {
    fn  eq_box(value: impl 'static + PartialEq) -> CmpDyn<Box<dyn 'static + DynPartialEq>> {
        CmpDyn::new(Box::new(value))
    }

    assert!(eq_box(1u8) == eq_box(1u8));
    assert!(eq_box(1u8) != eq_box(2u8));
    assert!(eq_box(1u8) != eq_box(1i8));
    assert!(eq_box(1u8) != eq_box("hello"));
    
    use std::collections::{HashMap, hash_map::DefaultHasher};
    type Key = CmpDyn<Box<dyn 'static + DynHash<DefaultHasher>>, DefaultHasher>;
    fn key(value: impl 'static + Eq + Hash) -> Key { CmpDyn::new(Box::new(value)) }
    let mut map: HashMap<Key, &'static str> = HashMap::new();
    
    map.insert(key(-1i32), "-1i32");
    map.insert(key("hello"), "the string \"hello\"");
    map.insert(key(4u128), "4u128");
    map.insert(key(()), "()");
    
    assert_eq!(map.remove(&key("hello")), Some("the string \"hello\""));
}

use std::{
	cmp::Ordering,
	mem::{transmute_copy, size_of},
	raw::TraitObject,
	hash::{Hash, Hasher},
	ops::Deref,
	marker::PhantomData,
};

#[derive(Clone, Copy)]
pub struct CmpDyn<T: ?Sized, Phantom = ()> {
	_phantom: PhantomData<Phantom>,
	pub inner: T, 
}

impl<T, Phantom> CmpDyn<T, Phantom> {
	fn new(value: T) -> Self {
		CmpDyn { inner: value, _phantom: PhantomData }
	}
}

unsafe fn trait_object<T>(x: T) -> TraitObject { transmute_copy(&x) }

macro_rules! impl_cmp_dyn_traits{
	(
		$(
			trait $trait:ident $(:$dep:ident)? for $real_trait:ident {
				$(
					$fn:ident -> $ret:ty { $real_fn:ident, $if_diff_vtable:expr }
				)*
			}
		)*
	) => {
		$(
			pub unsafe trait $trait $(:$dep)? {
				$(
					unsafe fn $fn(&self, rhs_ptr: *const ()) -> $ret;
				)*
			}

			unsafe impl<T: ?Sized + $real_trait> $trait for T {
				$(
					#[inline(always)]
					unsafe fn $fn(&self, rhs_ptr: *const ()) -> $ret {
						assert_eq!(size_of::<&Self>(), size_of::<usize>());
						self.$real_fn(transmute_copy::<*const (), &Self>(&rhs_ptr))
					}
				)*
			}

			impl<T, Phantom> $real_trait for CmpDyn<T, Phantom> where
				T: ?Sized + Deref,
				T::Target: $trait,
			{
				$(
					fn $real_fn(&self, rhs: &Self) -> $ret {
						unsafe {
							let ptr = if size_of::<&T::Target>() == size_of::<usize>() {
								// thin pointer

								&rhs.inner as *const _ as _
							} else {
								// fat pointer
								assert_eq!(size_of::<&T::Target>(), size_of::<TraitObject>());

								let rhs_obj = trait_object::<&T::Target>(&*rhs.inner);

								if rhs_obj.vtable != trait_object::<&_>(&*self.inner).vtable {
									return $if_diff_vtable;
								}

								rhs_obj.data as _
							};

							self.inner.$fn(ptr)
						}
					}
				)*
			}
		)*
	};
}

impl_cmp_dyn_traits!{
	trait DynPartialEq for PartialEq {
		dyn_eq -> bool { eq, false }
		dyn_ne -> bool { ne, true }
	}

	trait DynEq: DynPartialEq for Eq {}

	trait DynPartialOrd: DynPartialEq for PartialOrd {
		dyn_partial_cmp -> Option<Ordering> { partial_cmp, None }
		dyn_le -> bool { le, false }
		dyn_lt -> bool { lt, false }
		dyn_ge -> bool { ge, false }
		dyn_gt -> bool { gt, false }
	}
}

pub trait DynHash<H>: DynEq {
	fn dyn_hash(&self, hasher: &mut H);
}

impl<T: ?Sized + Hash + Eq, H: Hasher> DynHash<H> for T {
	fn dyn_hash(&self, hasher: &mut H) {
		self.hash(hasher)
	}
}

impl<T, H1> Hash for CmpDyn<T, H1> where
	T: ?Sized + Deref,
	T::Target: DynHash<H1>,
	H1: Hasher,
{
	fn hash<H2: std::hash::Hasher>(&self, hasher: &mut H2) {
		unsafe {
			unsafe fn type_id<T: Hasher>() -> usize {
				let dummy = std::mem::MaybeUninit::<T>::uninit();
				trait_object(&(dummy.get_ref() as &dyn Hasher)).vtable as _
			}
			assert!(type_id::<H1>() == type_id::<H2>());
			self.inner.dyn_hash(std::mem::transmute::<&mut H2, &mut H1>(hasher));
		}
	}
}