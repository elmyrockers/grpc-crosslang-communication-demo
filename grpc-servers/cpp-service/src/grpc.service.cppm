module;
#include <string>
#include <print>
#include <grpcpp/grpcpp.h>
#include <flatbuffers/grpc.h>
#include "user.pb.h"
#include "user.grpc.pb.h"
#include "user_generated.h"
#include "user.grpc.fb.h"

// #include "boost/pfr.hpp"

export module grpc.service;

export class UserServer final : public user::UserService::Service
{
private:
	std::unique_ptr<user_fb::UserService::Stub> stub;
public:
	UserServer() {
		auto requestChannel = grpc::CreateChannel("localhost:50052", grpc::InsecureChannelCredentials());
		auto stub = user_fb::UserService::NewStub( requestChannel );
		this->stub = std::move(stub);
	}

	// rpc All (GetRequest) returns (GetResponse);
	grpc::Status All(grpc::ServerContext* context, const user::GetRequest* request, user::GetResponse* response) override
	{
		// Send Get Request with flatbuffers
			// Prepare payload
				flatbuffers::grpc::MessageBuilder mb;
				auto fbRequest = user_fb::CreateGetRequest(mb);
				mb.Finish(fbRequest);

			// Send a request
				grpc::ClientContext clientContext;
				auto requestMessage = mb.ReleaseMessage<user_fb::GetRequest>();
				flatbuffers::grpc::Message<user_fb::GetResponse> responseMessage;
				grpc::Status status = this->stub->All(&clientContext, requestMessage, &responseMessage);
				if (!status.ok()) {
					return grpc::Status(grpc::StatusCode::INTERNAL, "Downstream service failed: " + status.error_message());
				}

			// Send response back to http-server
				const user_fb::GetResponse* fbResponse = responseMessage.GetRoot();
				auto fbUsers = fbResponse->users();
				if (!fbUsers) return grpc::Status::OK;

				for (flatbuffers::uoffset_t i = 0; i < fbUsers->size(); ++i) {
					const user_fb::User* fbUser = fbUsers->Get(i);

					user::User* protoUser = response->add_users();
					protoUser->set_id(fbUser->id());
					protoUser->set_name(fbUser->name()->str());
					protoUser->set_age(fbUser->age());
					protoUser->set_location(fbUser->location()->str());
					protoUser->set_email(fbUser->email()->str());

					std::print( stderr, "\n\n\nUser Detail {}:\nName: {}\nEmail: {}\nAge: {}\nLocation: {}", fbUser->id(), fbUser->name()->str(), fbUser->email()->str(), fbUser->age(), fbUser->location()->str());
				}

		return grpc::Status::OK;
	}

	grpc::Status New(grpc::ServerContext* context, const user::PostRequest* request, user::SuccessResponse* response) override
	{
		// gRPC request with flatbuffers
			// Create a request with flatbuffers
				flatbuffers::grpc::MessageBuilder mb;
				auto name = mb.CreateString( request->name() );
				auto location = mb.CreateString( request->location() );
				auto email = mb.CreateString( request->email() );
				auto fbRequest = user_fb::CreatePostRequest(mb, name, request->age(), location, email );
				mb.Finish(fbRequest);

			// Send a request to rust-service
				grpc::ClientContext clientContext;
				auto requestMessage = mb.ReleaseMessage<user_fb::PostRequest>();
				flatbuffers::grpc::Message<user_fb::SuccessResponse> responseMessage;
				grpc::Status status = this->stub->New(&clientContext, requestMessage, &responseMessage);

				std::print( stderr, "\n\n\nRequest Message:\nName: {}\nEmail: {}\nAge: {}\nLocation: {}", requestMessage.GetRoot()->name()->str(), requestMessage.GetRoot()->email()->str(), request->age(), requestMessage.GetRoot()->location()->str());

			// Send response back to http-server
				bool isOK = false;
				if (status.ok()) {
					const user_fb::SuccessResponse* fbResponse = responseMessage.GetRoot();
					isOK = fbResponse->success();
				}
				response->set_success( isOK );
				std::print( stderr, "\n\nResponse Message:\nSuccess: {}", isOK );

		return grpc::Status::OK;
	}

	grpc::Status Edit(grpc::ServerContext* context, const user::PatchRequest* request, user::SuccessResponse* response) override
	{
		std::print( stderr, "\n\nID: {}\nName: {}\nEmail: {}\nAge: {}\nLocation: {}", request->id(), request->name(), request->email(), request->age(), request->location());

		return grpc::Status::OK;
	}

	grpc::Status Delete(grpc::ServerContext* context, const user::DeleteRequest* request, user::SuccessResponse* response) override
	{
		std::print( stderr, "\n\nID: {}", request->id());

		return grpc::Status::OK;
	}

	~UserServer(){}
};