use thiserror::Error;
use reqwest::Error as ReqwestError;

pub enum Error {
    Reqwest(ReqwestError),
    Forbidden(String),
    Other(String),
}