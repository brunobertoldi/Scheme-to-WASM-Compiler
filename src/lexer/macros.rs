macro_rules! tokens {
    (
        $name:ident, $tag_name:ident {
            $( $chunks:tt )*
        }
    ) => {
        tokens_impl! {
            @name $name
            @tag $tag_name
            @bytes bytes
            @queue [$( $chunks )*]
            @variants []
            @token_variants []
            @parse_match []
        }
    }
}

macro_rules! tokens_impl {
    (
        @name $name:ident
        @tag $tag_name:ident
        @bytes $bytes:ident
        @queue []
        @variants [ $( $variants:tt )* ]
        @token_variants [ $( $tokens:tt )* ]
        @parse_match [ $( $parse_match:tt )* ]
    ) => {
        #[derive(Clone, PartialEq, Debug)]
        pub enum $name {
            $( $variants )*
        }

        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        pub enum $tag_name {
            $( $tokens )*
        }

        impl $tag_name {
            pub fn parse(&self, $bytes: &[u8]) -> Result<$name, VoidError> {
                match *self {
                    $( $parse_match )*
                }
            }
        }
    };

    (
        @name $name:ident
        @tag $tag_name:ident
        @bytes $bytes:ident
        @queue [
            $variant_name:ident($variant_ty:ty),
            $( $chunks:tt )*
        ]
        @variants [ $( $variants:tt )+ ]
        @token_variants [ $( $tokens:tt )+ ]
        @parse_match [ $( $parse_match:tt )+ ]
    ) => {
        tokens_impl! {
            @name $name
            @tag $tag_name
            @bytes $bytes
            @queue [ $( $chunks )* ]
            @variants [ $( $variants )*, $variant_name($variant_ty) ]
            @token_variants [ $( $tokens )*, $variant_name ]
            @parse_match [
                $( $parse_match )*
                $tag_name::$variant_name =>
                    Ok($name::$variant_name(TokenFromBytes::from_bytes($bytes)?)),
            ]
        }
    };

    (
        @name $name:ident
        @tag $tag_name:ident
        @bytes $bytes:ident
        @queue [
            $variant_name:ident($variant_ty:ty),
            $( $chunks:tt )*
        ]
        @variants []
        @token_variants []
        @parse_match []
    ) => {
        tokens_impl! {
            @name $name
            @tag $tag_name
            @bytes $bytes
            @queue [ $( $chunks )* ]
            @variants [ $variant_name($variant_ty) ]
            @token_variants [ $variant_name ]
            @parse_match [
                $tag_name::$variant_name =>
                    Ok($name::$variant_name(TokenFromBytes::from_bytes($bytes)?)),
            ]
        }
    };

    (
        @name $name:ident
        @tag $tag_name:ident
        @bytes $bytes:ident
        @queue [
            $variant_name:ident: $expected:expr,
            $( $chunks:tt )*
        ]
        @variants [ $( $variants:tt )+ ]
        @token_variants [ $( $tokens:tt )+ ]
        @parse_match [ $( $parse_match:tt )+ ]
    ) => {
        tokens_impl! {
            @name $name
            @tag $tag_name
            @bytes $bytes
            @queue [ $( $chunks )* ]
            @variants [ $( $variants )*, $variant_name ]
            @token_variants [ $( $tokens )*, $variant_name ]
            @parse_match [
                $( $parse_match )*
                $tag_name::$variant_name => if $bytes == $expected {
                    Ok($name::$variant_name)
                } else {
                    Err(VoidError)
                },
            ]
        }
    };

    (
        @name $name:ident
        @tag $tag_name:ident
        @bytes $bytes:ident
        @queue [
            $variant_name:ident: $expected:expr,
            $( $chunks:tt )*
        ]
        @variants []
        @token_variants []
        @parse_match []
    ) => {
        tokens_impl! {
            @name $name
            @tag $tag_name
            @bytes $bytes
            @queue [ $( $chunks )* ]
            @variants [ $variant_name ]
            @token_variants [ $variant_name ]
            @parse_match [
                $tag_name::$variant_name => if $bytes == $expected {
                    Ok($name::$variant_name)
                } else {
                    Err(VoidError)
                },
            ]
        }
    };
}
