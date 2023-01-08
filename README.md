# `define_into_enum!` v1.0.0

This defines a single macro, `define_into_enum!`, which is meant to be compile as e.g.:

```rust
use define_into_enum::define_into_enum;
define_into_enum! {
  #[derive(Debug, PartialEq)]
  pub enum TestEnum {
    Variant1((usize, usize)),
    Variant2((usize, usize)),
    Variant3((usize, usize)),
  }
  (v1, v2),
  (v1, v2)
}
```

And then it will `impl Into<&'a str> for TestEnum`, converting all `v` to `v`.

Works with meta-tags as well:

```no_compile
define_into_enum! {
  #[non_exhaustive]
  #[derive(enum_kinds::EnumKind)]
  #[enum_kind(WordKind, derive(IsVariant, Unwrap))]
  #[derive(Clone, Debug, PartialEq, Eq, IsVariant, Unwrap)]
  pub enum Word<'a> {
    SimpleWord(Cow<'a, str>),
    SingleQuotedWord(Cow<'a, str>),
    DoubleQuotedWord(Cow<'a, str>),
    DollarSingleQuotedWord(Cow<'a, str>),
    DollarDoubleQuotedWord(Cow<'a, str>),
    UnquotedWord(Cow<'a, str>),
    ErrorWord(Cow<'a, str>),
  }
  ( s1 ),
  ( s1 )
}
```

# The macro
```rust
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
```

## License

Copyright 2023 Fredrick R. Brennan

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
