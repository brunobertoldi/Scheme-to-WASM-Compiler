macro_rules! tokens {
    (
        $name:ident, $tag_name:ident {
            $( $chunks:tt )*
        }
    ) => {
        tokens_impl! {
            @impl
            @name $name
            @tag $tag_name
            @queue [$( $chunks )*]
            @variants []
            @token_variants []
        }
    }
}

macro_rules! tokens_impl {
    (
        @impl
        @name $name:ident
        @tag $tag_name:ident
        @queue []
        @variants [ $( $variants:tt )* ]
        @token_variants [ $( $tokens:tt )* ]
    ) => {
        #[derive(Clone, PartialEq, Debug)]
        pub enum $name {
            $( $variants )*
        }

        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        pub enum $tag_name {
            $( $tokens )*
        }
    };

    (
        @impl
        @name $name:ident
        @tag $tag_name:ident
        @queue [
            $variant_name:ident($variant_ty:ty),
            $( $chunks:tt )*
        ]
        @variants [ $( $variants:tt )+ ]
        @token_variants [ $( $tokens:tt )+ ]
    ) => {
        tokens_impl! {
            @impl
            @name $name
            @tag $tag_name
            @queue [ $( $chunks )* ]
            @variants [ $( $variants )*, $variant_name($variant_ty) ]
            @token_variants [ $( $tokens )*, $variant_name ]
        }
    };

    (
        @impl
        @name $name:ident
        @tag $tag_name:ident
        @queue [
            $variant_name:ident($variant_ty:ty),
            $( $chunks:tt )*
        ]
        @variants []
        @token_variants []
    ) => {
        tokens_impl! {
            @impl
            @name $name
            @tag $tag_name
            @queue [ $( $chunks )* ]
            @variants [ $variant_name($variant_ty) ]
            @token_variants [ $variant_name ]
        }
    };

    (
        @impl
        @name $name:ident
        @tag $tag_name:ident
        @queue [
            $variant_name:ident,
            $( $chunks:tt )*
        ]
        @variants [ $( $variants:tt )+ ]
        @token_variants [ $( $tokens:tt )+ ]
    ) => {
        tokens_impl! {
            @impl
            @name $name
            @tag $tag_name
            @queue [ $( $chunks )* ]
            @variants [ $( $variants )*, $variant_name ]
            @token_variants [ $( $tokens )*, $variant_name ]
        }
    };

    (
        @impl
        @name $name:ident
        @tag $tag_name:ident
        @queue [
            $variant_name:ident,
            $( $chunks:tt )*
        ]
        @variants []
        @token_variants []
    ) => {
        tokens_impl! {
            @impl
            @name $name
            @tag $tag_name
            @queue [ $( $chunks )* ]
            @variants [ $variant_name ]
            @token_variants [ $variant_name ]
        }
    };
}
