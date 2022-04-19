
use crate::common::{WebError, HTTP_TOKEN_KEY, token_data};
use crate::model::user::{User, get_user_for_phonne};
use axum::extract::{FromRequest, RequestParts};
use async_trait::async_trait;


pub struct  BlogAuth {
   pub user:User,
}
#[async_trait]
impl<B> FromRequest<B> for BlogAuth
    where
        B: Send,
{
    type Rejection = WebError;
    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let header =  request.headers();
        match  header.get(HTTP_TOKEN_KEY){
            Some(d)=>{
                match d.to_str(){
                    Ok(token)=>{
                        match token_data(token){
                            Ok(data)=>{
                               match get_user_for_phonne(&data.claims.phone).await {
                                   Ok(user)=>{
                                       Ok(BlogAuth{user})
                                   },
                                   Err(_e)=>Err(WebError::LoginAuthFailedError)
                               }
                            }
                            Err(_e)=>Err(WebError::LoginAuthFailedError)
                        }
                    },
                    Err(_e)=> Err(WebError::LoginAuthFailedError)
                }
            }
            None => Err(WebError::LoginAuthFailedError)
        }
    }
}
