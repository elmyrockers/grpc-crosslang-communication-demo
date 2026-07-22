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

use crate::user_pb::PostRequest as PbPostRequest;
use crate::user_pb::PatchRequest as PbPatchRequest;
use crate::user_pb::DeleteRequest as PbDeleteRequest;


pub struct UserService {
	client: UserServiceClient<Channel>,
}

impl UserService {
	pub fn new(client: UserServiceClient<Channel>) -> Self {
		Self { client }
	}

	pub async fn all(&self, _raw: &[u8]) -> Result<Bytes, Status> {
		println!("all() called");

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

	pub async fn add(&self, raw: &[u8]) -> Result<Bytes, Status> {
		println!("add() called");

		// Send post request to go-service
			let request = flatbuffers::root::<PostRequest>(raw)
				.map_err(|e| Status::invalid_argument(e.to_string()))?;

			let mut client = self.client.clone();
			let response = client
							.add(PbPostRequest {
								name: request.name().unwrap_or("").to_string(),
								age: request.age(),
								location: request.location().unwrap_or("").to_string(),
								email: request.email().unwrap_or("").to_string(),
							})
							.await
							.map_err(|e| Status::internal(format!("go user service call failed: {}", e)))?
							.into_inner();
			println!("Go response: {:#?}", response);

		// Send response back to cpp-service
			self.success(response.success)
	}

	pub async fn edit(&self, raw: &[u8]) -> Result<Bytes, Status> {
		println!("edit() called");

		// Send patch request to go-service
			let request = flatbuffers::root::<PatchRequest>(raw)
				.map_err(|e| Status::invalid_argument(e.to_string()))?;

			let mut client = self.client.clone();
			let response = client
				.edit(PbPatchRequest {
					id: request.id(),
					name: request.name().unwrap_or("").to_string(),
					age: request.age(),
					location: request.location().unwrap_or("").to_string(),
					email: request.email().unwrap_or("").to_string(),
				})
				.await
				.map_err(|e| Status::internal(format!("go user service call failed: {}", e)))?
				.into_inner();
			println!("Go response: {:#?}", response);

		// Send response back to cpp-service
			self.success(response.success)
	}

	pub async fn delete(&self, raw: &[u8]) -> Result<Bytes, Status> {
		println!("delete() called");

		// Send delete request to go-service
			let request = flatbuffers::root::<DeleteRequest>(raw)
								.map_err(|e| Status::invalid_argument(e.to_string()))?;

			let mut client = self.client.clone();
			let response = client.delete(PbDeleteRequest { id: request.id() })
								 .await
								 .map_err(|e| Status::internal(format!("go user service call failed: {}", e)))?
								 .into_inner();
			println!("Go response: {:#?}", response);

		// Send response back to cpp-service
			self.success(response.success)
	}

	fn success(&self, ok: bool) -> Result<Bytes, Status> {
		let mut b = FlatBufferBuilder::with_capacity(64);
		let resp = SuccessResponse::create(&mut b, &SuccessResponseArgs { success: ok });
		b.finish(resp, None);
		Ok(Bytes::copy_from_slice(b.finished_data()))
	}
}