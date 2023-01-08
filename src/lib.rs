#![doc = include_str!("../README.md")]
#![no_std]

#[macro_export]
macro_rules! define_into_enum {
  {
    $(#[$attr:meta])*
    $vis:vis enum $name:ident $(< $generic:lifetime >)* {
      $(#[$variant_attr1:meta])*
      $variant1:ident ($type1:ty),
      $(
        $(#[$variant_attr:meta])*
        $variant:ident ($type:ty),
      )*
    }
    $into:pat,
    $output:expr
  } => {
    $(#[$attr])*
    $vis enum $name $(<$generic>)* {
      $(#[$variant_attr1])*
      $variant1($type1),
      $(
        $(#[$variant_attr])*
        $variant($type),
      )*
    }

    impl$(<$generic>)* Into <$type1> for $name $(<$generic>)* {
      fn into(self) -> $type1 {
        match self {
          $(
            Self::$variant( $into ) => ( $output ),
          )+
          Self::$variant1( $into ) => ( $output ),
        }
      }
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_into_impl() {
    define_into_enum! {
      #[derive(Debug, PartialEq)]
      pub enum TestEnum<'a> {
        #[allow(dead_code)]
        Variant1(&'a str),
        #[allow(dead_code)]
        Variant2(&'a str),
        #[allow(dead_code)]
        Variant3(&'a str),
      }
      v, v
    }

  }

  #[test]
  fn test_enum_variants() {
    define_into_enum! {
      #[derive(Debug, PartialEq)]
      pub enum TestEnum {
        #[allow(dead_code)]
        Variant1(i32),
        #[allow(dead_code)]
        Variant2(i32),
        #[allow(dead_code)]
        Variant3(i32),
      }
      v, v
    }

    assert_eq!(
      Into::<i32>::into(TestEnum::Variant3(0)),
      0,
      "Incorrect conversion for Variant1"
    );
    assert_eq!(
      Into::<i32>::into(TestEnum::Variant3(8i32)),
      8,
      "Incorrect conversion for Variant2"
    );
    assert_eq!(
      Into::<i32>::into(TestEnum::Variant3(-9i32)),
      -9,
      "Incorrect conversion for Variant3"
    );
  }
}
