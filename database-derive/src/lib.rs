use proc_macro::{self, TokenStream};

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DataProvider)]
pub fn derive_from_request(input: TokenStream) -> TokenStream {
    let DeriveInput { ref ident, .. } = parse_macro_input!(input);

    let output = quote! {
        use rocket::request::{FromRequest, Outcome};
        use rocket::Request;

        #[rocket::async_trait]
        impl<'r> FromRequest<'r> for #ident {
            type Error = ();
            async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
                let conn = request.guard::<Connection<TXpressDB>>().await.unwrap();
                Outcome::Success(#ident::new(conn))
            }
        }
    };

    output.into()
}
