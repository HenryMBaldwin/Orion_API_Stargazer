/// Name: reqres.rs
/// Description: This file defines the structs that are used to communicate between the server and any clients.
/// Any functions that need to be exposed to either cleint must have request and response structs defined here.
/// 
use serde;
use serde::{Serialize, Deserialize};


//this file will define structs for calls/responses to/from OrionAPI through the named pipe
//this should be used by both client and server to ensure that communication is consistant on both ends

//JSON request should include a type field
//ex.
//
// {
//     "type": "Login",
//     "username": "username123",
//     "password": "pass123"
// } 



//types
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RequestType {
    Login(LoginRequest),
    CheckAuth(CheckAuthRequest),
    Query(QueryRequest),
    GetQueryPrompts(GetQueryPromptsRequest),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseType {
    Login(LoginResponse),
    CheckAuth(CheckAuthResponse),
    Query(QueryResponse),
    GetQueryPrompts(GetQueryPromptsResponse),
}


//struct request

//struct response


//login()
#[derive(Serialize, Deserialize)]
pub struct LoginRequest{
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse{
    //where u16 is a status code
    pub status: u16
}

//check_auth()
#[derive(Serialize, Deserialize)]
pub struct CheckAuthRequest{}

#[derive(Serialize, Deserialize)]
pub struct CheckAuthResponse{
    pub result: bool,
}


//query()
#[derive(Serialize, Deserialize)]
pub struct QueryRequest{
    pub id: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryResponse{
    pub status: u16,
    pub result: String,
}

//get_query_prompts()
#[derive(Serialize, Deserialize)]
pub struct GetQueryPromptsRequest{
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetQueryPromptsResponse{
    pub status: u16,
    pub prompts: String,
}
