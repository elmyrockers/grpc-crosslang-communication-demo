#include <string>
#include <iostream>
#include <grpcpp/grpcpp.h>


import grpc.service;





int main(){
	// Register service and listening port
		UserServer userSv;
		grpc::ServerBuilder builder;
		builder.AddListeningPort( "0.0.0.0:50051", grpc::InsecureServerCredentials() );
		builder.RegisterService( &userSv );

	// Start server
		std::cout << "\n\ngRPC server is listening on port 50051" << std::endl;
		std::unique_ptr<grpc::Server> server(builder.BuildAndStart());
		if (server) { server->Wait(); }

	return 1;
}