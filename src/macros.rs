// Copyright (C) 2023 Nitrokey GmbH
// SPDX-License-Identifier: LGPL-3.0-only

macro_rules! enum_u8 {
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident {
            $(#![mask($mask:expr)])?
            $($var:ident = $num:expr),+
            $(,)*
        }
    ) => {
        $(#[$outer])*
        #[repr(u8)]
        $vis enum $name {
            $(
                $var = $num,
            )*
        }

        impl From<$name> for u8 {
            fn from(val: $name) -> u8 {
                match val {
                    $(
                         $name::$var => $num,
                    )*
                }
            }
        }

        impl TryFrom<u8> for $name {
            type Error = ();
            fn try_from(val: u8) -> ::core::result::Result<Self, ()> {
                match val $(& $mask)* {
                    $(
                        $num => Ok($name::$var),
                    )*
                    _ => Err(())
                }
            }
        }

        impl<T : Into<u8>> core::ops::BitOr<T> for $name {
            type Output = u8;
        	fn bitor(self, rhs: T) -> u8 {
        		let a: u8 = self.into();
        		let b: u8 = rhs.into();
        		a | b
                }
        }
    }
}

pub(crate) use enum_u8;
