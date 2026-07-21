use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;
use bytes::Bytes;
use flatbuffers::FlatBufferBuilder;
use tonic::Status;

use crate::user_generated::user_fb::{
	DeleteRequest, GetResponse, GetResponseArgs,
	PatchRequest, PostRequest, SuccessResponse, SuccessResponseArgs,
	User, UserArgs,
};

use crate::user_pb::user_service_client::UserServiceClient;
use crate::user_pb::GetRequest as PbGetRequest;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
struct UserRecord {
	id: i32,
	name: String,
	age: i32,
	location: String,
	email: String,
}

static NEXT_ID: AtomicI32 = AtomicI32::new(1);

pub struct UserService {
	users: Mutex<Vec<UserRecord>>,
	client: UserServiceClient<Channel>,
}

impl UserService {
	pub fn new(client: UserServiceClient<Channel>) -> Self {
		Self { users: Mutex::new(Vec::new()), client }
	}

	pub async fn all(&self, _raw: &[u8]) -> Result<Bytes, Status> {
		println!("All() called");
		
		// Example: List of users
			// let users = vec![
			// 	UserRecord { id: 1, name: "Alice".into(), age: 30, location: "New York".into(), email: "alice@email.com".into() },
			// 	UserRecord { id: 2, name: "Bob".into(),   age: 25, location: "London".into(),   email: "bob@email.com".into() },
			// 	UserRecord { id: 3, name: "Charlie".into(), age: 35, location: "Tokyo".into(),  email: "charlie@email.com".into() },
			// ];

		// Get list of users
			let mut client = self.client.clone();
			let resp = client
				.all(PbGetRequest {})
				.await
				.map_err(|e| Status::internal(format!("go user service call failed: {}", e)))?
				.into_inner();
			println!("Go response: {:#?}", resp);

		// Re-map users
			let mut b = FlatBufferBuilder::with_capacity(512);
			let user_offsets: Vec<_> = resp.users.iter().map(|u| {
				let name     = b.create_string(&u.name);
				let location = b.create_string(&u.location);
				let email    = b.create_string(&u.email);
				User::create(&mut b, &UserArgs {
					id: u.id,
					name: Some(name),
					age: u.age,
					location: Some(location),
					email: Some(email),
				})
			}).collect();

		// Send response back as flatbuffers
			let users_vec = b.create_vector(&user_offsets);
			let response = GetResponse::create(&mut b, &GetResponseArgs { users: Some(users_vec) });
			b.finish(response, None);

		Ok(Bytes::copy_from_slice(b.finished_data()))
	}

	pub fn add(&self, raw: &[u8]) -> Result<Bytes, Status> {
		let req = flatbuffers::root::<PostRequest>(raw)
			.map_err(|e| Status::invalid_argument(e.to_string()))?;

		self.users.lock().unwrap().push(UserRecord {
			id:       NEXT_ID.fetch_add(1, Ordering::SeqCst),
			name:     req.name().unwrap_or("").to_string(),
			age:      req.age(),
			location: req.location().unwrap_or("").to_string(),
			email:    req.email().unwrap_or("").to_string(),
		});

		self.success(true)
	}

	pub fn edit(&self, raw: &[u8]) -> Result<Bytes, Status> {
		let req = flatbuffers::root::<PatchRequest>(raw)
			.map_err(|e| Status::invalid_argument(e.to_string()))?;

		let mut users = self.users.lock().unwrap();
		match users.iter_mut().find(|u| u.id == req.id()) {
			Some(u) => {
				if let Some(v) = req.name()     { u.name     = v.to_string(); }
				if let Some(v) = req.location() { u.location = v.to_string(); }
				if let Some(v) = req.email()    { u.email    = v.to_string(); }
				if req.age() != 0               { u.age      = req.age(); }
				self.success(true)
			}
			None => self.success(false),
		}
	}

	pub fn delete(&self, raw: &[u8]) -> Result<Bytes, Status> {
		let req = flatbuffers::root::<DeleteRequest>(raw)
			.map_err(|e| Status::invalid_argument(e.to_string()))?;

		let mut users = self.users.lock().unwrap();
		let before = users.len();
		users.retain(|u| u.id != req.id());
		self.success(users.len() < before)
	}

	fn success(&self, ok: bool) -> Result<Bytes, Status> {
		let mut b = FlatBufferBuilder::with_capacity(64);
		let resp = SuccessResponse::create(&mut b, &SuccessResponseArgs { success: ok });
		b.finish(resp, None);
		Ok(Bytes::copy_from_slice(b.finished_data()))
	}
}