module;
#include <string>
#include <print>
#include <grpcpp/grpcpp.h>
#include <flatbuffers/grpc.h>
#include "user.pb.h"
#include "user.grpc.pb.h"
#include "user_generated.h"

// #include "boost/pfr.hpp"

export module grpc.service;

export class UserServer final : public user::UserService::Service
{
public:
	UserServer(){
		std::print( "UserServer dilaksanakan!" );
	}

	// rpc All (GetRequest) returns (GetResponse);
	grpc::Status All(grpc::ServerContext* context, const user::GetRequest* request, user::GetResponse* response) override
	{
		user::User* u1 = response->add_users();
		u1->set_id(1);
		u1->set_name("Ahmad");
		u1->set_age(30);
		u1->set_location("Kuala Lumpur");
		u1->set_email("ahmad@gmail.com");

		user::User* u2 = response->add_users();
		u2->set_id(2);
		u2->set_name("Siti");
		u2->set_age(25);
		u2->set_location("Alor Setar");
		u2->set_email("siti@gmail.com");

		return grpc::Status::OK;
	}

	grpc::Status New(grpc::ServerContext* context, const user::PostRequest* request, user::SuccessResponse* response) override
	{
		std::print( stderr, "\n\nName: {}\nEmail: {}\nAge: {}\nLocation: {}", request->name(), request->email(), request->age(), request->location());

		flatbuffers::grpc::MessageBuilder mb;


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